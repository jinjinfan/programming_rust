use std::io::{self, Read, Write, ErrorKind};
const DEFAULT_BUF_SIZE : usize = 8 * 1024;

pub fn copy<R : ?Sized, W:?Sized>(reader : &mut R, writer : &mut W)
    -> io::Result<u64>
    where R:Read, W: Write
{
    let mut buf = [0;DEFAULT_BUF_SIZE];
    let mut writtern = 0;
    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => return Ok(writtern),
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        writer.write_all(&buf[..len])?;
        writtern += len as u64;
    }    
}

use std::io::prelude::*;
fn grep(target : &str) -> io::Result<()> {
    let stdin = io::stdin();
    for line_result in stdin.lock().lines() {
        let line = line_result?;
        if line.contains(target) {
            println!("{}", line);
        }
    }
    Ok(())
}
fn grep_gen<R>(target : &str, reader : R) -> io::Result<()>
    where R : BufRead
{
    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(target) {
            println!("{}", line);
        }
    }
    Ok(())
}

use std::fs::File;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

fn grep_main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let target = match args.next() {
        Some(s) => s,
        None => Err("Usage: grep PATTERN FILE...")?
    };
    let files : Vec<PathBuf> = args.map(PathBuf::from).collect();
    if files.is_empty() {
        let stdin = io::stdin();
        grep_gen(&target, stdin.lock())?;
    } else {
        for file in files {
            let f = File::open(file)?;
            grep_gen(&target, io::BufReader::new(f))?;
        }
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>>{
    /*let result = grep_main();
    if let Err(err) = result {
        eprintln!("Got error {}", err);
        std::process::exit(1);
    }

    use std::process::{Command, Stdio};
    let mut child = Command::new("grep")
            .arg("-e")
            .arg("a.*e.*i.*o.*u")
            .stdin(Stdio::piped())
            .spawn()?;
    let mut to_child = child.stdin.take().unwrap();
    let my_words = vec![
        "".to_string()
    ];
    for word in my_words {
        writeln!(to_child, "{}", word)?
    }
    drop(to_child);
    child.wait()?;*/

    #[derive(serde::Serialize, serde::Deserialize)]
    struct Player {
        location : String,
        items    : Vec<String>,
        health   : u32
    }

    use std::path::Path;
    use std::io;
    
    fn swizzle_file<P>(path_arg : P) -> io::Result<()>
        where P: AsRef<Path>
    {
        let path = path_arg.as_ref();
        Ok(())
    }

    assert_eq!(Path::new("/home/fwolfe/program.txt").parent(),
            Some(Path::new("/home/fwolfe")));
    use std::ffi::OsStr;
    assert_eq!(Path::new("/home/fwolfe/program.txt").file_name(), 
                Some(OsStr::new("program.txt")));
    
    let path1 = Path::new("/usr/share/dict");
    assert_eq!(path1.join("words"),
                Path::new("/usr/share/dict/words"));
    let abs_path = std::env::current_dir()?.join("word");
    println!("abs path: {}", abs_path.display());

    let file = Path::new("/home/jinjin/calendars/calendar-18x18.pdf");
    assert_eq!(file.ancestors().collect::<Vec<_>>(),
                vec![
                    Path::new("/home/jinjin/calendars/calendar-18x18.pdf"),
                    Path::new("/home/jinjin/calendars"),
                    Path::new("/home/jinjin"),
                    Path::new("/home/"),
                    Path::new("/"),
                ]);

    use std::fs;

    /// Copy the existing directory `src` to the target path `dst`
    fn copy_dir_to(src : &Path, dst : &Path) -> io::Result<()> {
        if !dst.is_dir() {
            fs::create_dir(dst)?
        }
        for entry_result in src.read_dir()? {
            let entry = entry_result?;
            let file_type = entry.file_type()?;
            copy_to(&entry.path(), &file_type, &dst.join(entry.file_name()))?;
        }
        Ok(())
    }

    /// Copy whatever is at `src` to the target path `dst`
    fn copy_to(src : &Path, src_type : &fs::FileType, dst : &Path) -> io::Result<()> {
        if src_type.is_file() {
            fs::copy(src, dst)?;
        } else if src_type.is_dir() {
            copy_dir_to(src, dst)?;
        } else if src_type.is_symlink() {
            let target = src.read_link()?;
            std::os::unix::fs::symlink(target, dst)?;
        }
        else {
            return Err(io::Error::new(io::ErrorKind::Other,
                format!("don not know how to copy: {}",src.display())));
        }
        Ok(())
    }




    Ok(())
}
