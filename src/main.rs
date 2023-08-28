use anyhow::{Context, Result};
use clap::Parser;
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// -P, --physical avoid all symlinks
    #[clap(short('P'), long)]
    physical: bool,

    /// -L, --logical use PWD from environment, even if it contains symlinks
    #[clap(short('L'), long)]
    logical: bool,
}

fn main() -> Result<()> {
    let args: Cli = Cli::parse();

    // カレントディレクトリを取得
    let mut current_dir = env::current_dir()?;

    // オプションに従って動作を変更
    current_dir = match (args.logical, args.physical) {
        (_, true) => current_dir
            .canonicalize()
            .with_context(|| "Failed to resolve symlinks")?,
        (true, _) | (_, _) => {
            if let Some(logical_path) = env::var_os("PWD") {
                PathBuf::from(logical_path)
            } else {
                current_dir
            }
        }
    };

    // パスを表示
    writeln!(io::stdout(), "{}", current_dir.display())?;
    Ok(())
}
