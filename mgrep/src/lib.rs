use std::{fs::File, io, io::{BufReader, Write, Read, Stdout}, ops::Range, path::Path, ptr::read};
use anyhow::Result;
use clap::Parser;

mod errors;
use colored::Colorize;
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
    pub fn match_with_default_strategy(&self) -> Result<(), GrepError> {
        self.match_with(defaule_strategy)
    }

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

fn defaule_strategy<W: Write, R: Read>(
    path: &Path, reader: BufReader<R>, pattern: &Regex, writer: &mut W) -> Result<(), GrepError> {
    reader.
    
    Ok(())
}

fn format_line(line: &str, lineno: usize, range: Range<usize>) -> String {
    let Range { start, end } = range;
    let prefix = &line[..start];

    format!("{0: >6}:{1: <3} {2}{3}{4}", 
        lineno.to_string().blue(), 
        (prefix.chars().count() + 1).to_string().cyan(), 
        prefix,
        &line[start..end].red(),
        &line[end..])
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
