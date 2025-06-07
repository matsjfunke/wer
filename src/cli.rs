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
    
    /// Show dates only 
    #[arg(short = 'd', long = "date-only")]
    pub date_only: bool,
    
    /// Show commit message on the next line
    #[arg(short = 'm', long = "commit-message")]
    pub commit_message: bool,
    
    /// Show last N people who touched the file/directory (normal mode only)
    #[arg(long = "top")]
    pub top: Option<usize>,
    
    /// Disable colored output / syntax highlighting
    #[arg(long = "no-color")]
    pub no_color: bool,
} 