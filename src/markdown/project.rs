use crate::interface::{HasItem, MarkdownExportable};
use crate::model::Project;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ProjectFrontMatter {
    #[serde(flatten)]
    pub item: Project,
    pub tags: Vec<String>,
}

impl MarkdownExportable<Project> for ProjectFrontMatter {
    fn get_item(&self) -> &Project {
        &self.item
    }

    fn get_tags(&self) -> &[String] {
        &self.tags
    }
}

impl HasItem for Project {
    fn id(&self) -> &str {
        &self.id
    }

    fn title(&self) -> &str {
        &self.title
    }
}
