use std::error::Error;
use std::io::{Write, stderr};

fn print_error(mut err : &dyn Error) {
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), " caused by: {}", source);
        err = source;
    }
}
/*
use std::fs;
use std::io;
use std::path::Path;

fn move_all(src : &Path, dst : &Path) -> io::Result<()> {
    for entry_result in src.read_dir()? {
        let entry = entry_result?;
        let dst_file = dst.join(entry.file_name())?;
        fs::rename(entry.path(), dst_file)?;
    }
    Ok(())
}
*/
use std::io::{self, BufRead};

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

fn read_numbers(file : &mut dyn BufRead) -> GenericResult<Vec<i64>> {
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?;
        numbers.push(line.parse()?);
    }
    Ok(numbers)
}

fn main() {
    println!("Hello, world!");
}
