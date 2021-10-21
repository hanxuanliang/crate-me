use anyhow::Result;
use clap::Parser;

mod errors;
pub use errors::GrepError;

#[derive(Parser, Debug)]
#[clap(version = "1.0")]
pub struct GrepConfig {
    pattern: String,
    glob: String,
}

impl GrepConfig {
    fn match_default_strategy(&self) -> Result<(), GrepError> {
        self.match_with()
    }

    fn match_with(&self) -> Result<(), GrepError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
