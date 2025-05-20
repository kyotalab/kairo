use std::fmt;
use std::fmt::Display;

use crate::{
    interface::{FrontMatterExportable, HasItem, MarkdownExportable, MarkdownParsable},
    model::Project,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ProjectContent {
    pub front_matter: ProjectFrontMatter,
    pub body: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProjectFrontMatter {
    #[serde(flatten)]
    pub item: Project,
    pub tags: Vec<String>,
}

impl MarkdownExportable<ProjectFrontMatter> for ProjectContent {
    fn get_front_matter(&self) -> &ProjectFrontMatter {
        &self.front_matter
    }

    fn get_body(&self) -> &Option<String> {
        &self.body
    }
}

impl FrontMatterExportable<Project> for ProjectFrontMatter {
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

impl MarkdownParsable<Project> for Project {
    fn get_item(&self) -> &Project {
        &self
    }
}

impl Display for ProjectContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let serialized = serde_yaml::to_string(&self.front_matter).map_err(|_| fmt::Error)?;

        writeln!(f, "---\n{}---", serialized)?;

        if let Some(body) = &self.body {
            writeln!(f, "\n{}", body)?;
        }

        Ok(())
    }
}
