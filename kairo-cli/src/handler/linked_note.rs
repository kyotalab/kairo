use crate::{commands::linked_note::LinkCommands, kairo_core::store::*};
use diesel::SqliteConnection;
use kairo_core::util::print_links_as_table;

pub fn handle_link_command(command: LinkCommands, conn: &mut SqliteConnection) {
    match command {
        LinkCommands::Create {
            arg_from,
            arg_to,
            arg_link_type,
        } => match create_link(conn, arg_from, arg_to, arg_link_type) {
            Ok(link) => println!("{:?}", link),
            Err(e) => eprintln!("Failed to create link: {}", e),
        },
        LinkCommands::List { arg_from, arg_to } => match list_links(conn, arg_from, arg_to) {
            Ok(links) => {
                print_links_as_table(conn, &links);
            }
            Err(e) => {
                eprintln!("Failed to fetch links: {}", e);
            }
        },
        LinkCommands::Get { arg_id } => match get_link_by_id(conn, &arg_id) {
            Ok(Some(link)) => {
                println!("{link}");
            }
            Ok(None) => {
                println!("Link not found");
            }
            Err(e) => {
                println!("Database error: {:?}", e);
            }
        },
        LinkCommands::Delete { arg_id } => match soft_delete_link(conn, &arg_id) {
            Ok(link) => println!("{:?}", link),
            Err(e) => eprintln!("Failed to delete link: {}", e),
        },
    }
}
