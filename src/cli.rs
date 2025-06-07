use clap::Parser;

#[derive(Parser)]
#[command(name = "wer")]
#[command(about = "Find who last edited a file or directory")]
pub struct Cli {
    /// File or directory path
    pub path: Option<String>,
} 