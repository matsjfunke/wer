use clap::Parser;

#[derive(Parser)]
#[command(name = "wer")]
#[command(about = "Find who last edited a file or directory")]
pub struct Cli {
    /// File or directory path
    pub path: Option<String>,
    
    /// Show git blame for the file (only works with files, not directories)
    #[arg(short = 'b', long = "blame")]
    pub blame: bool,
    
    /// Disable colored output
    #[arg(long = "no-color")]
    pub no_color: bool,
} 