use clap::Subcommand;

#[derive(Debug, Subcommand)]
#[command(about = "Manage tags (used to classify notes and tasks)")]
pub enum TagCommands {
    Create {
        #[arg(short = 'n', long = "name")]
        arg_tag_name: String,
    },
    List {
        #[arg(long = "deleted")]
        arg_deleted: Option<bool>,
    },
    Get {
        #[arg(long = "id")]
        arg_id: String,
    },
    Update {
        #[arg(long = "id")]
        arg_id: String,
        #[arg(short = 'n', long = "name")]
        arg_tag_name: String,
    },
    Delete {
        #[arg(long = "id")]
        arg_id: String,
    },
}
