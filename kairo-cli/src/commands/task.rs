use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum TaskCommands {
    Create {
        #[arg(short = 't', long = "title")]
        arg_title: String,
        #[arg(short = 'd', long = "description")]
        arg_description: Option<String>,
        #[arg(short = 'p', long = "priority")]
        arg_priority: Option<String>,
        #[arg(long = "due")]
        arg_due_date: Option<String>,
        #[arg(long = "pid")]
        arg_project_id: Option<String>,
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
        /* TODO
         *  1. task list --tagで指定のタグを含むノートを表示できるようにする。（これタスクには不要かも？）
         *  2. 現状,日付が降順でsortされているため、note list --order desc or ascで並び替えできるようにする。この時の並び替えは「created_at or due_date」？
         *  3. --priority で優先度でsortできるようにする。
         *  4. --pid でプロジェクトでsortできるようにする。
         */
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
        #[arg(short = 'd', long = "description")]
        arg_description: Option<String>,
        #[arg(short = 'p', long = "priority")]
        arg_priority: Option<String>,
        #[arg(long = "due")]
        arg_due_date: Option<String>,
        #[arg(long = "pid")]
        arg_project_id: Option<String>,
        /* TODO
         *  task update --tagでタグ更新はどうする？--tagが指定されなかった場合は、変更なし。--tagが指定されたときは、前のtagは削除して、新たに付け直す？
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
