use clap::{Clap, AppSettings};

mod errors;
pub use errors::GrepError;

#[derive(Clap, Debug)]
#[clap(version = "1.0")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct GrepConfig {
    pattern: String,
    glob: String,
}

impl GrepConfig {
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
