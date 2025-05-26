use crate::interface::HasItem;
use crate::model::{LinkedNote, Note, Project, Tag, Task};
use crate::store::{
    get_note_by_id, get_tags_by_note_id, get_tags_by_project_id, get_tags_by_task_id,
};
use diesel::SqliteConnection;
use prettytable::{Table, row};

pub fn print_notes_as_table(conn: &mut SqliteConnection, notes: &[Note]) {
    let mut table = Table::new();

    // Header
    table.add_row(row![
        "ID",
        "Title",
        "Type",
        "SubType",
        "Tags",
        "Related Project",
        "Related Task",
        "Created",
        "Updated",
        "Archived",
        "Deleted"
    ]);

    // Rows
    for note in notes {
        let mut sub_type = String::new();
        if let Some(sub) = note.sub_type {
            sub_type = format!("{:?}", sub);
        }
        let project_id = note.project_id.as_deref().unwrap_or("").to_string();
        let task_id = note.task_id.as_deref().unwrap_or("").to_string();

        let format_created = note.created_at.format("%Y/%m/%d %H:%M:%S").to_string();
        let format_updated = note.updated_at.format("%Y/%m/%d %H:%M:%S").to_string();

        let tags = get_tags_by_note_id(conn, &note.id).expect("failed");
        let tag_names: Vec<String> = tags.into_iter().map(|t| t.tag_name).collect();
        let sep = String::from(",");
        let str_tags = tag_names.join(&sep);

        table.add_row(row![
            note.id,
            note.title,
            format!("{:?}", note.note_type),
            sub_type,
            str_tags,
            project_id,
            task_id,
            format_created,
            format_updated,
            note.archived,
            note.deleted
        ]);
    }

    table.printstd();
}

pub fn print_projects_as_table(conn: &mut SqliteConnection, projects: &[Project]) {
    let mut table = Table::new();

    // Header
    table.add_row(row![
        "ID",
        "Title",
        "Description",
        "Tags",
        "Created",
        "Updated",
        "Archived",
        "Deleted"
    ]);

    // Rows
    for project in projects {
        let mut description = String::new();
        if let Some(str) = &project.description {
            description = format!("{:?}", str);
        }

        let format_created = project.created_at.format("%Y/%m/%d %H:%M:%S").to_string();
        let format_updated = project.updated_at.format("%Y/%m/%d %H:%M:%S").to_string();

        let tags = get_tags_by_project_id(conn, &project.id).expect("failed");
        let tag_names: Vec<String> = tags.into_iter().map(|t| t.tag_name).collect();
        let sep = String::from(",");
        let str_tags = tag_names.join(&sep);

        table.add_row(row![
            project.id,
            project.title,
            description,
            str_tags,
            format_created,
            format_updated,
            project.archived,
            project.deleted
        ]);
    }

    table.printstd();
}

pub fn print_tasks_as_table(conn: &mut SqliteConnection, tasks: &[Task]) {
    let mut table = Table::new();

    // Header
    table.add_row(row![
        "ID",
        "Title",
        "Description",
        "Tags",
        "Priority",
        "DueDate",
        "Created",
        "Updated",
        "Archived",
        "Deleted"
    ]);

    // Rows
    for task in tasks {
        let mut description = String::new();
        let mut priority = String::new();
        let mut due_date = String::new();

        if let Some(pri) = task.priority {
            priority = format!("{:?}", pri);
        }
        if let Some(str) = &task.description {
            description = format!("{:?}", str);
        }
        if let Some(due) = task.due_date {
            due_date = due.format("%Y/%m/%d").to_string();
        }

        let format_created = task.created_at.format("%Y/%m/%d %H:%M:%S").to_string();
        let format_updated = task.updated_at.format("%Y/%m/%d %H:%M:%S").to_string();

        let tags = get_tags_by_task_id(conn, &task.id).expect("failed");
        let tag_names: Vec<String> = tags.into_iter().map(|t| t.tag_name).collect();
        let sep = String::from(",");
        let str_tags = tag_names.join(&sep);

        table.add_row(row![
            task.id,
            task.title,
            description,
            str_tags,
            priority,
            due_date,
            format_created,
            format_updated,
            task.archived,
            task.deleted
        ]);
    }

    table.printstd();
}

pub fn print_tags_as_table(tags: &[Tag]) {
    let mut table = Table::new();

    // Header
    table.add_row(row!["ID", "Tag name",]);

    // Rows
    for tag in tags {
        table.add_row(row![tag.id, tag.tag_name,]);
    }

    table.printstd();
}

pub fn print_links_as_table(conn: &mut SqliteConnection, links: &[LinkedNote]) {
    let mut table = Table::new();

    // Header
    table.add_row(row!["ID", "From Note", "To Note", "Link Type", "Created"]);

    // Rows
    for link in links {
        let ln_type = match link.link_type {
            Some(ty) => format!("{:?}", ty),
            None => "-".to_string(),
        };

        let format_created = link.created_at.format("%Y/%m/%d %H:%M:%S").to_string();

        let from_note = get_note_by_id(conn, &link.from_id).expect("from_id lookup failed");
        let to_note = get_note_by_id(conn, &link.to_id).expect("to_id lookup failed");

        let from_str = match from_note {
            Some(note) => format!("{} - {}", note.id(), note.title()),
            None => "(not found)".to_string(),
        };

        let to_str = match to_note {
            Some(note) => format!("{} - {}", note.id(), note.title()),
            None => "(not found)".to_string(),
        };

        table.add_row(row![link.id, from_str, to_str, ln_type, format_created]);
    }

    table.printstd();
}
