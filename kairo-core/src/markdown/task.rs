use crate::{
    interface::{HasItem, MarkdownExportable},
    model::Task,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TaskFrontMatter {
    #[serde(flatten)]
    pub item: Task,
    pub tags: Vec<String>,
}

impl MarkdownExportable<Task> for TaskFrontMatter {
    fn get_item(&self) -> &Task {
        &self.item
    }

    fn get_tags(&self) -> &[String] {
        &self.tags
    }
}

impl HasItem for Task {
    fn id(&self) -> &str {
        &self.id
    }

    fn title(&self) -> &str {
        &self.title
    }
}
