use crate::commands::project::ProjectCommands;
use crate::config::AppConfig;
use crate::markdown::ProjectFrontMatter;
use crate::store::*;
use crate::util::write_to_markdown;
use diesel::SqliteConnection;

pub fn handle_project_command(
    command: ProjectCommands,
    conn: &mut SqliteConnection,
    config: &AppConfig,
) {
    match command {
        ProjectCommands::Create {
            arg_title,
            arg_description,
            arg_tags,
        } => match create_project(conn, arg_title, arg_description, arg_tags) {
            Ok(project) => {
                let dir = &config.paths.projects_dir;
                println!("{:?}", project);
                let tags = get_tags_by_project_id(conn, &project.id).unwrap();
                let tags_str = tags.into_iter().map(|t| t.tag_name).collect();

                let front_matter = ProjectFrontMatter {
                    item: project,
                    tags: tags_str,
                };

                if let Err(e) = write_to_markdown(&front_matter, dir) {
                    eprintln!("Failed to write project: {}", e)
                }
                println!("Run `kairo tui` to open dashboard")
            }
            Err(e) => eprintln!("Failed to create project: {}", e),
        },
        ProjectCommands::List {
            arg_archived,
            arg_deleted,
        } => match list_projects(conn, arg_archived, arg_deleted) {
            Ok(projects) => {
                for project in projects {
                    println!("{:?}", project);
                }
            }
            Err(e) => eprintln!("Failed to fetch projects: {}", e),
        },
        ProjectCommands::Get { arg_id } => match get_project_by_id(conn, &arg_id) {
            Ok(Some(project)) => {
                println!("{:?}", project);
            }
            Ok(None) => {
                println!("Project not found");
            }
            Err(e) => {
                println!("Database error: {:?}", e);
            }
        },
        ProjectCommands::Update {
            arg_id,
            arg_title,
            arg_description,
        } => match update_project(conn, &arg_id, arg_title, arg_description) {
            Ok(project) => println!("{:?}", project),
            Err(e) => eprintln!("Failed to update project: {}", e),
        },
        ProjectCommands::Archive { arg_id } => match archive_project(conn, &arg_id) {
            Ok(project) => println!("{:?}", project),
            Err(e) => eprintln!("Failed to archive project: {}", e),
        },
        ProjectCommands::Delete { arg_id } => match soft_delete_project(conn, &arg_id) {
            Ok(project) => println!("{:?}", project),
            Err(e) => eprintln!("Failed to delete project: {}", e),
        },
        ProjectCommands::Purge { arg_id } => match delete_project(conn, &arg_id) {
            Ok(project) => println!("{:?}", project),
            Err(e) => eprintln!("Failed to purge project: {}", e),
        },
        ProjectCommands::Unarchive { arg_id } => match unarchive_project(conn, &arg_id) {
            Ok(project) => println!("{:?}", project),
            Err(e) => eprintln!("Failed to un-archive project: {}", e),
        },
        ProjectCommands::Restore { arg_id } => match restore_project(conn, &arg_id) {
            Ok(project) => println!("{:?}", project),
            Err(e) => eprintln!("Failed to restore project: {}", e),
        },
    }
}
