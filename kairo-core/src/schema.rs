// @generated automatically by Diesel CLI.

diesel::table! {
    linked_notes (id) {
        id -> Text,
        from_id -> Text,
        to_id -> Text,
        link_type -> Nullable<Text>,
        created_at -> Timestamp,
        deleted -> Bool,
    }
}

diesel::table! {
    note_tags (note_id, tag_id) {
        note_id -> Text,
        tag_id -> Text,
    }
}

diesel::table! {
    notes (id) {
        id -> Text,
        title -> Text,
        note_type -> Text,
        sub_type -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        archived -> Bool,
        deleted -> Bool,
        project_id -> Nullable<Text>,
        task_id -> Nullable<Text>,
    }
}

diesel::table! {
    project_tags (project_id, tag_id) {
        project_id -> Text,
        tag_id -> Text,
    }
}

diesel::table! {
    projects (id) {
        id -> Text,
        title -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        archived -> Bool,
        deleted -> Bool,
    }
}

diesel::table! {
    tags (id) {
        id -> Text,
        tag_name -> Text,
        created_at -> Timestamp,
        deleted -> Bool,
    }
}

diesel::table! {
    task_tags (task_id, tag_id) {
        task_id -> Text,
        tag_id -> Text,
    }
}

diesel::table! {
    tasks (id) {
        id -> Text,
        title -> Text,
        description -> Nullable<Text>,
        priority -> Nullable<Text>,
        due_date -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        archived -> Bool,
        deleted -> Bool,
        project_id -> Nullable<Text>,
    }
}

diesel::joinable!(note_tags -> notes (note_id));
diesel::joinable!(note_tags -> tags (tag_id));
diesel::joinable!(project_tags -> projects (project_id));
diesel::joinable!(project_tags -> tags (tag_id));
diesel::joinable!(task_tags -> tags (tag_id));
diesel::joinable!(task_tags -> tasks (task_id));

diesel::allow_tables_to_appear_in_same_query!(
    linked_notes,
    note_tags,
    notes,
    project_tags,
    projects,
    tags,
    task_tags,
    tasks,
);
