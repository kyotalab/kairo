use crate::{
    config::AppConfig,
    markdown::{ProjectContent, ProjectFrontMatter},
    store::*,
    util::{parse_markdown, write_to_markdown},
};
use anyhow::Ok;
use diesel::SqliteConnection;

pub fn handle_create_project(
    config: &AppConfig,
    conn: &mut SqliteConnection,
    title: String,
    description: Option<String>,
    tags: Option<Vec<String>>,
) -> Result<(), anyhow::Error> {
    let project = create_project(conn, title, description, tags)?;

    let dir = &config.paths.projects_dir;
    println!("{:?}", project);
    let tags = get_tags_by_project_id(conn, &project.id).unwrap();
    let tags_str = tags.into_iter().map(|t| t.tag_name).collect();

    let front_matter = ProjectFrontMatter {
        item: project,
        tags: tags_str,
    };

    let project_content = ProjectContent {
        front_matter,
        body: None,
    };

    if let Err(e) = write_to_markdown(&project_content, dir) {
        eprintln!("Failed to write project: {}", e)
    }
    println!("Run `kairo tui` to open dashboard");
    Ok(())
}

pub fn handle_list_projects(
    conn: &mut SqliteConnection,
    include_archived: Option<bool>,
    include_deleted: Option<bool>,
    include_tags: Option<Vec<String>>,
    include_order: Option<String>,
) -> Result<(), anyhow::Error> {
    let projects = list_projects(
        conn,
        include_archived,
        include_deleted,
        include_tags,
        include_order,
    )?;
    for project in projects {
        println!("{:?}", project);
    }
    Ok(())
}

pub fn handle_get_project(
    conn: &mut SqliteConnection,
    project_id: String,
) -> Result<(), anyhow::Error> {
    let project = get_project_by_id(conn, &project_id)?;
    match project {
        Some(exist) => {
            println!("{:?}", exist);
        }
        None => {
            println!("Project not found");
        }
    }
    Ok(())
}

pub fn handle_update_project(
    config: &AppConfig,
    conn: &mut SqliteConnection,
    project_id: String,
    title: Option<String>,
    description: Option<String>,
    tags: Option<Vec<String>>,
) -> Result<(), anyhow::Error> {
    let updated_project = update_project(conn, &project_id, title, description, tags)?;

    let dir = &config.paths.projects_dir;
    println!("{:?}", &updated_project);
    let tags = get_tags_by_project_id(conn, &updated_project.id).unwrap();
    let tags_str: Vec<_> = tags.into_iter().map(|t| t.tag_name).collect();

    let contents = parse_markdown(&updated_project, dir)?;
    // let front_matter = contents.0;
    let body = Some(contents.1);

    let project_front_matter = ProjectFrontMatter {
        item: updated_project.clone(),
        tags: tags_str,
    };

    let project_content = ProjectContent {
        front_matter: project_front_matter,
        body,
    };

    if let Err(e) = write_to_markdown(&project_content, dir) {
        eprintln!("Failed to write project: {}", e)
    }

    // println!("Updated project: {:?}", updated_project.id);
    // println!("Run `kairo tui` to open dashboard");
    Ok(())
}

pub fn handle_archive_project(
    conn: &mut SqliteConnection,
    project_id: String,
) -> Result<(), anyhow::Error> {
    let project = archive_project(conn, &project_id)?;

    println!("Archived project: {:?}", project.id);
    Ok(())
}

pub fn handle_delete_project(
    conn: &mut SqliteConnection,
    project_id: String,
) -> Result<(), anyhow::Error> {
    let project = soft_delete_project(conn, &project_id)?;

    println!("Deleted project: {:?}", project.id);
    Ok(())
}

pub fn handle_purge_project(
    conn: &mut SqliteConnection,
    project_id: String,
) -> Result<(), anyhow::Error> {
    let _project = delete_project(conn, &project_id)?;

    println!("Purged project: {:?}", project_id);
    Ok(())
}

pub fn handle_unarchive_project(
    conn: &mut SqliteConnection,
    project_id: String,
) -> Result<(), anyhow::Error> {
    let project = unarchive_project(conn, &project_id)?;

    println!("Unarchived project: {:?}", project.id);
    Ok(())
}

pub fn handle_restore_project(
    conn: &mut SqliteConnection,
    project_id: String,
) -> Result<(), anyhow::Error> {
    let project = restore_project(conn, &project_id)?;

    println!("Restored project: {:?}", project.id);
    Ok(())
}
