use crate::{
    model::Tag,
    schema::{note_tags, project_tags, tags, tags::dsl::*, task_tags},
};
use chrono::{NaiveDateTime, Utc};
use diesel::{SqliteConnection, prelude::*, result::Error};

// ==============================
// ▼ Structs / Create
// ==============================
#[derive(Insertable)]
#[diesel(table_name = tags)]
pub struct NewTag {
    pub id: String,
    pub tag_name: String,
    pub created_at: NaiveDateTime,
    pub deleted: bool,
}

// ==============================
// ▼ Structs / Update
// ==============================
#[derive(AsChangeset)]
#[diesel(table_name = tags)]
pub struct UpdatedTag {
    pub tag_name: String,
}

// ==============================
// ▼ Structs / SoftDelete
// ==============================
#[derive(AsChangeset)]
#[diesel(table_name = tags)]
pub struct SoftDeletedTag {
    pub deleted: bool,
}

// ==============================
// ▼ Create / Insert
// ==============================
fn generate_tag_id(conn: &mut SqliteConnection) -> Result<String, diesel::result::Error> {
    use regex::Regex;

    // タグIDの最大数値部分を取得
    let all_ids: Vec<String> = tags.select(id).load::<String>(conn)?;

    // 正規表現で "t-001" 形式の数字部分を抽出して最大値を見つける
    let re = Regex::new(r"t-(\d{3})").unwrap();
    let max_num = all_ids
        .iter()
        .filter_map(|tag_id| {
            re.captures(tag_id)
                .and_then(|caps| caps.get(1)?.as_str().parse::<u32>().ok())
        })
        .max()
        .unwrap_or(0);

    // 次の番号を3桁ゼロ埋めで生成
    let next_id = format!("t-{:03}", max_num + 1);
    Ok(next_id)
}

pub fn create_tag(conn: &mut SqliteConnection, input_tag_name: String) -> Result<Tag, Error> {
    let tag_id = generate_tag_id(conn)?;
    let new_tag = NewTag {
        id: tag_id,
        tag_name: input_tag_name,
        created_at: Utc::now().naive_utc(),
        deleted: false,
    };

    diesel::insert_into(tags::table)
        .values(&new_tag)
        .returning(Tag::as_select())
        .get_result(conn)
}

// ==============================
// ▼ Read / Select
// ==============================
pub fn list_tags(
    conn: &mut SqliteConnection,
    include_deleted: Option<bool>,
) -> Result<Vec<Tag>, Error> {
    let mut query = tags.into_boxed();

    match include_deleted.unwrap_or(false) {
        false => query = query.filter(deleted.eq(false)),
        true => query = query.filter(deleted.eq(true)),
    }

    Ok(query
        .select(Tag::as_select())
        .order(created_at.desc())
        .load(conn)?)
}

pub fn get_tag_by_id(conn: &mut SqliteConnection, tag_id: &str) -> Result<Option<Tag>, Error> {
    let tag = tags
        .find(tag_id)
        .select(Tag::as_select())
        .first(conn)
        .optional()?;

    Ok(tag)
}

pub fn get_tag_by_name(
    conn: &mut SqliteConnection,
    input_tag_name: String,
) -> Result<Option<Tag>, Error> {
    let tag = tags
        .filter(tag_name.eq(input_tag_name))
        .select(Tag::as_select())
        .first(conn)
        .optional()?;

    Ok(tag)
}

pub fn get_tags_by_note_id(conn: &mut SqliteConnection, note_id: &str) -> Result<Vec<Tag>, Error> {
    let note_tags = note_tags::table
        .inner_join(tags::table.on(note_tags::tag_id.eq(tags::id)))
        .filter(note_tags::note_id.eq(note_id))
        .select(tags::all_columns)
        .load::<Tag>(conn)?;

    Ok(note_tags)
}

pub fn get_tags_by_project_id(
    conn: &mut SqliteConnection,
    project_id: &str,
) -> Result<Vec<Tag>, Error> {
    let project_tags = project_tags::table
        .inner_join(tags::table.on(project_tags::tag_id.eq(tags::id)))
        .filter(project_tags::project_id.eq(project_id))
        .select(tags::all_columns)
        .load::<Tag>(conn)?;

    Ok(project_tags)
}

pub fn get_tags_by_task_id(conn: &mut SqliteConnection, task_id: &str) -> Result<Vec<Tag>, Error> {
    let task_tags = task_tags::table
        .inner_join(tags::table.on(task_tags::tag_id.eq(tags::id)))
        .filter(task_tags::task_id.eq(task_id))
        .select(tags::all_columns)
        .load::<Tag>(conn)?;

    Ok(task_tags)
}

// ==============================
// ▼ Update
// ==============================
pub fn rename_tag(
    conn: &mut SqliteConnection,
    tag_id: &str,
    updated_tag_name: String,
) -> Result<Tag, Error> {
    let _exist_tag = ensure_tag_exists(conn, tag_id)?;

    let updated_tag = UpdatedTag {
        tag_name: updated_tag_name,
    };

    diesel::update(tags.find(tag_id))
        .set(updated_tag)
        .returning(Tag::as_select())
        .get_result(conn)
}

// ==============================
// ▼ Delete
// ==============================
pub fn soft_delete_tag(conn: &mut SqliteConnection, tag_id: &str) -> Result<Tag, Error> {
    let exist_tag = ensure_tag_exists(conn, tag_id)?;

    if exist_tag.deleted {
        return Err(Error::QueryBuilderError("Tag is already deleted".into()));
    }

    diesel::update(tags.find(tag_id))
        .set(SoftDeletedTag { deleted: true })
        .returning(Tag::as_select())
        .get_result(conn)
}

// ==============================
// ▼ Internal Common Utils
// ==============================
fn ensure_tag_exists(conn: &mut SqliteConnection, tag_id: &str) -> Result<Tag, Error> {
    match get_tag_by_id(conn, tag_id)? {
        Some(tag) => Ok(tag),
        None => Err(Error::QueryBuilderError("Tag not found".into())),
    }
}
