use kairo::repository::*;

fn main() {
    let connection = &mut establish_connection();

    // =====================================================================
    // Insert note test
    let input_title = String::from("The project: Rust CLI Kairo");
    let input_note_type = "fleeting";
    let input_sub_type = "idea";
    let input_project_id = String::new();
    let input_task_id = String::new();

    let note = create_note(
        connection,
        input_title,
        &input_note_type,
        &input_sub_type,
        Some(input_project_id),
        Some(input_task_id),
    );
    println!("{:?}", note);

    // =====================================================================
    // list notes test
    match list_notes(connection, false, false) {
        Ok(notes) => {
            for note in notes {
                println!("{:?}", note);
            }
        }
        Err(e) => {
            eprintln!("Failed to fetch notes: {}", e);
        }
    }

    // =====================================================================
    // get note by id test
    let search_id = "20250502T002021";
    match get_note_by_id(connection, search_id) {
        Ok(Some(note)) => {
            println!("{:?}", note);
        }
        Ok(None) => {
            println!("Note not found");
        }
        Err(e) => {
            println!("Database error: {:?}", e);
        }
    }

    // =====================================================================
    // update note by id test
    let updated_title = String::from("Changed title");
    let updated_note_type = "permanent";
    let updated_sub_type = "question";
    let updated_project_id = String::new();
    let updated_task_id = String::new();

    let updated_note = update_note(
        connection,
        search_id,
        updated_title,
        &updated_note_type,
        &updated_sub_type,
        Some(updated_project_id),
        Some(updated_task_id),
    );

    println!("{:?}", updated_note);
}
