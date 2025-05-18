use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum NoteCommands {
    Create {
        #[arg(short = 't', long = "title")]
        arg_title: String,
        #[arg(short = 'n', long = "note-type")]
        arg_note_type: String,
        #[arg(short = 's', long = "sub-type")]
        arg_sub_type: String,
        #[arg(long = "pid")]
        arg_project_id: Option<String>,
        #[arg(long = "tid")]
        arg_task_id: Option<String>,
        #[arg(long = "tag")]
        arg_tags: Option<Vec<String>>,
    },
    List {
        #[arg(long = "archived")]
        arg_archived: Option<bool>,
        #[arg(long = "deleted")]
        arg_deleted: Option<bool>,
        #[arg(long = "tag")]
        arg_tags: Option<Vec<String>>,
        #[arg(long = "order")]
        arg_order: Option<String>,
    },
    Get {
        #[arg(long = "id")]
        arg_id: String,
    },
    Update {
        #[arg(long = "id")]
        arg_id: String,
        #[arg(short = 't', long = "title")]
        arg_title: Option<String>,
        #[arg(short = 'n', long = "note-type")]
        arg_note_type: Option<String>,
        #[arg(short = 's', long = "sub-type")]
        arg_sub_type: Option<String>,
        #[arg(long = "pid")]
        arg_project_id: Option<String>,
        #[arg(long = "tid")]
        arg_task_id: Option<String>,
        #[arg(long = "tag")]
        arg_tags: Option<Vec<String>>,
        /* TODO
         *  note update --tagでタグ更新はどうする？--tagが指定されなかった場合は、変更なし。--tagが指定されたときは、前のtagは削除して、新たに付け直す？
         */
    },
    Archive {
        #[arg(long = "id")]
        arg_id: String,
    },
    Delete {
        #[arg(long = "id")]
        arg_id: String,
    },
    Purge {
        #[arg(long = "id")]
        arg_id: String,
    },
    Unarchive {
        #[arg(long = "id")]
        arg_id: String,
    },
    Restore {
        #[arg(long = "id")]
        arg_id: String,
    },
}
