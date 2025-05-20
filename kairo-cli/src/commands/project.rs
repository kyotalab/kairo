use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum ProjectCommands {
    Create {
        #[arg(short = 't', long = "title")]
        arg_title: String,
        #[arg(short = 'd', long = "description")]
        arg_description: Option<String>,
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
         *  1. project list --tagで指定のタグを含むノートを表示できるようにする。
         *  2. 現状日付が降順でsortされているため、project list --order desc or ascで並び替えできるようにする。
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
        #[arg(long = "tag")]
        arg_tags: Option<Vec<String>>,
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
