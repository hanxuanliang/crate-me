use anyhow::Result;

use clap::Parser;
use mgrep::GrepConfig;

fn main() -> Result<()> {
    let config: GrepConfig = GrepConfig::parse();
    config.match_with_default_strategy()?;

    Ok(())
}
