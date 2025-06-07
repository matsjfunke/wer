use clap::Parser;

#[derive(Parser)]
#[command(name = "wer")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(disable_version_flag = true)]
#[command(about = "Find out who last edited any file or directory in a Git repository")]
#[command(long_about = r#"Find out who last edited any file or directory in a Git repository

MODES:
  Regular mode (default): Shows the last commit that touched a file or directory
    Format: "61fcdda Author Name - 07 Jun 2025: commit message"
    Works with both files and directories
    
  Blame mode (-b): Shows line-by-line git blame with syntax highlighting  
    Format: "61fcdda (Author Name - 07 Jun) | 1 | code content"
    Only works with files, not directories

EXAMPLES:
  wer Cargo.toml              Show who last edited Cargo.toml
  wer src/                    Show who last touched the src/ directory
  wer -b src/main.rs          Show git blame with syntax highlighting  
  wer -d .                    Show only the date of last change
  wer -l 3 src/               Show last 3 contributors to src/ directory
  wer -b -m file.py           Show blame with commit messages"#)]
#[command(arg(clap::Arg::new("version")
    .short('v')
    .long("version")
    .action(clap::ArgAction::Version)
    .help("Print version")))]
pub struct Cli {
    /// File or directory path
    pub path: Option<String>,
    
    /// Show git blame with syntax highlighting (files only)
    #[arg(short = 'b', long = "blame")]
    pub blame: bool,
    
    /// Show dates only
    /// Regular mode: "07 Jun 2025"
    /// Blame mode: "07 Jun | 1 | code content"
    #[arg(short = 'd', long = "date-only")]
    pub date_only: bool,
    
    /// Show commit messages on separate indented lines
    #[arg(short = 'm', long = "commit-message")]
    pub commit_message: bool,
    
    /// Show the last N unique contributors (regular mode only)
    ///
    /// Lists the most recent N unique people who modified the path, with an
    /// indication if fewer contributors exist than requested.
    #[arg(short = 'l', long = "last", value_name = "N")]
    pub last: Option<usize>,
    
    /// Disable colored output and syntax highlighting
    #[arg(long = "no-color")]
    pub no_color: bool,
} 