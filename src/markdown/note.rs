use crate::interface::{HasItem, MarkdownExportable};
use crate::model::Note;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NoteFrontMatter {
    #[serde(flatten)]
    pub item: Note,
    pub tags: Vec<String>,
}

impl MarkdownExportable<Note> for NoteFrontMatter {
    fn get_item(&self) -> &Note {
        &self.item
    }

    fn get_tags(&self) -> &[String] {
        &self.tags
    }
}

impl HasItem for Note {
    fn id(&self) -> &str {
        &self.id
    }

    fn title(&self) -> &str {
        &self.title
    }
}
