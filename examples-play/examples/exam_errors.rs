use std::{env, fs::File, io::{self, Read}, num};

use thiserror::Error;
use anyhow::{Context, Result};

// thiserror 代替了之前需要自己手写实现 error 的步骤
#[derive(Error, Debug)]
enum CliError {
    #[error("{0}")]
    IoError(#[from] io::Error),

    #[error("{0}")]
    ParseError(#[from] num::ParseIntError),
}

// anyhow 替代了
fn run(filename: &str) -> Result<i32, anyhow::Error> {
    let mut file = File::open(filename)
        .context(format!("unable to open {}", filename))?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;
    let mut sum = 0;
    for c in content.lines() {
        let n = c.parse::<i32>()?;
        sum += n;
    }

    Ok(sum)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let res = run(filename)?;
    println!("{:?}", res);

    Ok(())
}
