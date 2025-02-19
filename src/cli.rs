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
        title: String,
        content: String,
        #[arg(short, long, num_args= 1.., value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },
    List,
    View {
        title: String,
    },
    Delete {
        title: String,
    },
}
impl Cli {
    pub fn parse() -> Self {
        clap::Parser::parse()
    }
}
