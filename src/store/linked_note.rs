use crate::{
    model::{LinkType, LinkedNote},
    schema::{linked_notes, linked_notes::dsl::*},
};
use chrono::{NaiveDateTime, Utc};
use diesel::{SqliteConnection, prelude::*, result::Error};

// ==============================
// ▼ Structs / Create
// ==============================
#[derive(Insertable)]
#[diesel(table_name = linked_notes)]
pub struct NewLinkedNote {
    pub id: String,
    pub from_id: String,
    pub to_id: String,
    pub link_type: Option<LinkType>,
    pub created_at: NaiveDateTime,
    pub deleted: bool,
}

// ==============================
// ▼ Structs / SoftDelete
// ==============================
#[derive(AsChangeset)]
#[diesel(table_name = linked_notes)]
pub struct SoftDeletedLinkedNote {
    pub deleted: bool,
}

// ==============================
// ▼ Create / Insert
// ==============================
fn generate_link_id(conn: &mut SqliteConnection) -> Result<String, diesel::result::Error> {
    use regex::Regex;

    // タグIDの最大数値部分を取得
    let all_ids: Vec<String> = linked_notes.select(id).load::<String>(conn)?;

    // 正規表現で "p-001" 形式の数字部分を抽出して最大値を見つける
    let re = Regex::new(r"ln-(\d{3})").unwrap();
    let max_num = all_ids
        .iter()
        .filter_map(|link_id| {
            re.captures(link_id)
                .and_then(|caps| caps.get(1)?.as_str().parse::<u32>().ok())
        })
        .max()
        .unwrap_or(0);

    // 次の番号を3桁ゼロ埋めで生成
    let next_id = format!("ln-{:03}", max_num + 1);
    Ok(next_id)
}

pub fn create_link(
    conn: &mut SqliteConnection,
    input_from_id: String,
    input_to_id: String,
    input_link_type: String,
) -> Result<LinkedNote, Error> {
    let link_id = generate_link_id(conn)?;
    let validated_link_type = parse_link_type(&input_link_type)?;

    let new_link = NewLinkedNote {
        id: link_id,
        from_id: input_from_id,
        to_id: input_to_id,
        link_type: validated_link_type,
        created_at: Utc::now().naive_utc(),
        deleted: false,
    };

    diesel::insert_into(linked_notes::table)
        .values(&new_link)
        .returning(LinkedNote::as_select())
        .get_result(conn)
}

// ==============================
// ▼ Read / Select
// ==============================
pub fn list_links(
    conn: &mut SqliteConnection,
    include_from_id: Option<String>,
    include_to_id: Option<String>,
) -> Result<Vec<LinkedNote>, Error> {
    let mut query = linked_notes.into_boxed();

    match (include_from_id, include_to_id) {
        (Some(include_from_id), None) => {
            query = query.filter(from_id.eq(include_from_id));
        }
        (None, Some(include_to_id)) => {
            query = query.filter(to_id.eq(include_to_id));
        }
        (None, None) => {
            // 全件取得
        }
        (Some(_), Some(_)) => {
            return Err(Error::QueryBuilderError(
                "Cannot filter by both from_id and to_id at the same time.".into(),
            ));
        }
    }

    Ok(query
        .select(LinkedNote::as_select())
        .order(created_at.desc())
        .load(conn)?)
}

pub fn get_link_by_id(
    conn: &mut SqliteConnection,
    link_id: &str,
) -> Result<Option<LinkedNote>, Error> {
    let link = linked_notes
        .find(link_id)
        .select(LinkedNote::as_select())
        .first(conn)
        .optional()?;

    Ok(link)
}

// ==============================
// ▼ Delete
// ==============================
pub fn soft_delete_link(conn: &mut SqliteConnection, link_id: &str) -> Result<LinkedNote, Error> {
    let exist_link = ensure_link_exists(conn, link_id)?;

    if exist_link.deleted {
        return Err(Error::QueryBuilderError("Link is already deleted".into()));
    }

    diesel::update(linked_notes.find(link_id))
        .set(SoftDeletedLinkedNote { deleted: true })
        .returning(LinkedNote::as_select())
        .get_result(conn)
}

// ==============================
// ▼ Internal Common Utils
// ==============================
fn ensure_link_exists(conn: &mut SqliteConnection, link_id: &str) -> Result<LinkedNote, Error> {
    match get_link_by_id(conn, link_id)? {
        Some(link) => Ok(link),
        None => Err(Error::QueryBuilderError("Link not found".into())),
    }
}

fn parse_link_type(input: &str) -> Result<Option<LinkType>, Error> {
    match input {
        "structure" => Ok(Some(LinkType::Structure)),
        "reference" => Ok(Some(LinkType::Reference)),
        "support" => Ok(Some(LinkType::Support)),
        "related" => Ok(Some(LinkType::Related)),
        "refute" => Ok(Some(LinkType::Refute)),
        "" | "_" => Ok(None),
        other => Err(Error::QueryBuilderError(
            format!("Invalid link_type: {}", other).into(),
        )),
    }
}
