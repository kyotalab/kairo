use crate::{
    interface::{FrontMatterExportable, HasItem, MarkdownExportable, MarkdownParsable},
    model::Task,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TaskContent {
    pub front_matter: TaskFrontMatter,
    pub body: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TaskFrontMatter {
    #[serde(flatten)]
    pub item: Task,
    pub tags: Vec<String>,
}

impl MarkdownExportable<TaskFrontMatter> for TaskContent {
    fn get_front_matter(&self) -> &TaskFrontMatter {
        &self.front_matter
    }

    fn get_body(&self) -> &Option<String> {
        &self.body
    }
}

impl FrontMatterExportable<Task> for TaskFrontMatter {
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

impl MarkdownParsable<Task> for Task {
    fn get_item(&self) -> &Task {
        &self
    }
}
