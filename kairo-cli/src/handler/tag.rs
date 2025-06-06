use crate::commands::tag::TagCommands;
use diesel::SqliteConnection;
use kairo_core::{store::*, util::print_tags_as_table};

pub fn handle_tag_command(command: TagCommands, conn: &mut SqliteConnection) {
    match command {
        TagCommands::Create { arg_tag_name } => match create_tag(conn, arg_tag_name) {
            Ok(tag) => println!("{:?}", tag),
            Err(e) => eprintln!("Failed to create tag: {}", e),
        },
        TagCommands::List { arg_deleted } => match list_tags(conn, arg_deleted) {
            Ok(tags) => {
                // for tag in tags {
                //     println!("{:?}", tag);
                // }
                print_tags_as_table(&tags);
            }
            Err(e) => eprintln!("Failed to fetch tags: {}", e),
        },
        TagCommands::Get { arg_id } => match get_tag_by_id(conn, &arg_id) {
            Ok(Some(tag)) => {
                println!("{tag}");
            }
            Ok(None) => {
                println!("Tag not found");
            }
            Err(e) => {
                println!("Database error: {:?}", e);
            }
        },
        TagCommands::Update {
            arg_id,
            arg_tag_name,
        } => match rename_tag(conn, &arg_id, arg_tag_name) {
            Ok(tag) => println!("{:?}", tag),
            Err(e) => eprintln!("Failed to update tag: {}", e),
        },
        TagCommands::Delete { arg_id } => match soft_delete_tag(conn, &arg_id) {
            Ok(tag) => println!("{:?}", tag),
            Err(e) => eprintln!("Failed to delete tag: {}", e),
        },
    }
}
