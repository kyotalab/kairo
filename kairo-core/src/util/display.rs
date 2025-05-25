use crate::model::{Note, Project, Task};
use prettytable::{Table, row};

pub fn print_notes_as_table(notes: &[Note]) {
    let mut table = Table::new();

    // Header
    table.add_row(row![
        "ID",
        "Title",
        "Type",
        "SubType",
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
        let mut project_id = String::new();
        let mut task_id = String::new();
        if let Some(sub) = note.sub_type {
            sub_type = format!("{:?}", sub);
        }
        if let Some(pid) = &note.project_id {
            project_id = format!("{:?}", pid);
        }
        if let Some(tid) = &note.task_id {
            task_id = format!("{:?}", tid);
        }

        let format_created = note.created_at.format("%Y/%m/%d %H:%M:%S").to_string();
        let format_updated = note.updated_at.format("%Y/%m/%d %H:%M:%S").to_string();

        table.add_row(row![
            note.id,
            note.title,
            format!("{:?}", note.note_type),
            sub_type,
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

pub fn print_projects_as_table(projects: &[Project]) {
    let mut table = Table::new();

    // Header
    table.add_row(row![
        "ID",
        "Title",
        "Description",
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

        table.add_row(row![
            project.id,
            project.title,
            description,
            format_created,
            format_updated,
            project.archived,
            project.deleted
        ]);
    }

    table.printstd();
}

pub fn print_tasks_as_table(tasks: &[Task]) {
    let mut table = Table::new();

    // Header
    table.add_row(row![
        "ID",
        "Title",
        "Description",
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

        table.add_row(row![
            task.id,
            task.title,
            description,
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
