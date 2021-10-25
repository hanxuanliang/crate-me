use std::{fs::File, io, io::{BufReader, Stdout}, path::Path};
use anyhow::Result;
use clap::Parser;

mod errors;
pub use errors::GrepError;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use regex::Regex;

#[derive(Parser, Debug)]
#[clap(version = "1.0")]
pub struct GrepConfig {
    pattern: String,
    glob: String,
}

type StrategyFn<W, R> = fn(&Path, BufReader<R>, &Regex, &mut W) -> Result<(), GrepError>;

impl GrepConfig {
    fn match_with(&self, strategy: StrategyFn<Stdout, File>) -> Result<(), GrepError> {
        let regex = Regex::new(&self.pattern)?;
        let files: Vec<_> = glob::glob(&self.glob)?.collect();

        files.into_par_iter().for_each(|v| {
            if let Ok(filename) = v {
                if let Ok(file) = File::open(&filename) {
                    let read = BufReader::new(file);
                    let mut stdout = io::stdout();
                    
                    if let Err(e) = strategy(filename.as_path(), read, &regex, &mut stdout) {
                        println!("Internal Error: {:?}", e);
                    }
                }
            }
        });
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
