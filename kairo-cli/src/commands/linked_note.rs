use clap::Subcommand;

#[derive(Debug, Subcommand)]
#[command(about = "Create, view, and remove links between notes")]
pub enum LinkCommands {
    Create {
        #[arg(long = "from")]
        arg_from: String,
        #[arg(long = "to")]
        arg_to: String,
        #[arg(long = "link-type")]
        arg_link_type: String,
    },
    List {
        #[arg(long = "from")]
        arg_from: Option<String>,
        #[arg(long = "to")]
        arg_to: Option<String>,
    },
    Get {
        #[arg(long = "id")]
        arg_id: String,
    },
    Delete {
        #[arg(long = "id")]
        arg_id: String,
    },
}
