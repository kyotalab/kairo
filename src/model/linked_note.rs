use chrono::NaiveDateTime;
use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    expression::*,
    prelude::*,
    serialize::{Output, ToSql},
    sql_types::Text,
    sqlite::Sqlite,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum LinkType {
    Structure,
    Reference,
    Support,
    Related,
    Refute,
}

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::linked_notes)]
pub struct LinkedNote {
    pub id: String,
    pub from_id: String,
    pub to_id: String,
    pub link_type: Option<LinkType>,
    pub created_at: NaiveDateTime,
    pub deleted: bool,
}

// --- ToSql<Text, Sqlite> 実装 ---
impl ToSql<Text, Sqlite> for LinkType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        let value = match self {
            LinkType::Structure => "structure",
            LinkType::Reference => "reference",
            LinkType::Support => "support",
            LinkType::Related => "related",
            LinkType::Refute => "refute",
        };
        <str as ToSql<Text, Sqlite>>::to_sql(value, out)
    }
}

// --- FromSql<Text, Sqlite> 実装 ---
impl FromSql<Text, Sqlite> for LinkType {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let s = <*const str as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        match unsafe { &*s } {
            "structure" => Ok(LinkType::Structure),
            "reference" => Ok(LinkType::Reference),
            "support" => Ok(LinkType::Support),
            "related" => Ok(LinkType::Related),
            "refute" => Ok(LinkType::Refute),
            other => Err(format!("Unrecognized LinkType variant: {}", other).into()),
        }
    }
}
