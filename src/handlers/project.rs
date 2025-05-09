use crate::commands::project::ProjectCommands;
use crate::repository::*;
use diesel::SqliteConnection;

pub fn handle_project_command(command: ProjectCommands, conn: &mut SqliteConnection) {
    match command {
        ProjectCommands::Create {
            arg_title,
            arg_description,
        } => match create_project(conn, arg_title, arg_description) {
            Ok(project) => println!("{:?}", project),
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
