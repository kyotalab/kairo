use crate::{
    interface::{FrontMatterExportable, HasItem, MarkdownExportable, MarkdownParsable},
    model::Note,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NoteContent {
    pub front_matter: NoteFrontMatter,
    pub body: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct NoteFrontMatter {
    #[serde(flatten)]
    pub item: Note,
    pub tags: Vec<String>,
}

impl MarkdownExportable<NoteFrontMatter> for NoteContent {
    fn get_front_matter(&self) -> &NoteFrontMatter {
        &self.front_matter
    }

    fn get_body(&self) -> &Option<String> {
        &self.body
    }
}

impl FrontMatterExportable<Note> for NoteFrontMatter {
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

impl MarkdownParsable<Note> for Note {
    fn get_item(&self) -> &Note {
        &self
    }
}
