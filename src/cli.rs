use clap::{Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Subcommand, Debug)]
pub enum Commands {
    New {
        content: String,
        title: Option<String>,
        #[arg(short, long, num_args= 1.., value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },
    List {
        tags: Option<Vec<String>>,
    },
    View {
        title: String,
    },
    Delete {
        title: String,
    },
    Edit {
        title: String,
        content: String,
    },
}
#[allow(dead_code)]
impl Cli {
    pub fn parse() -> Self {
        clap::Parser::parse()
    }
}
