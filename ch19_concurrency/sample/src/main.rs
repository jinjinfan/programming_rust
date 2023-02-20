use std::cmp::Reverse;
use std::path::{Path,PathBuf};
use std::process::Output;
use std::thread::JoinHandle;
use std::{thread, io};
use std::sync::Arc;

fn splot_vec_into_chunks(filenames : Vec<String>, num : usize) -> Vec<String> {
    filenames
}

fn process_files(file : String, glossary : &Arc<std::collections::HashMap<usize,String>>) -> io::Result<()> {
    Ok(())
}

fn process_files_in_parallel(filenames:Vec<String>,
                             glossary : Arc<std::collections::HashMap<usize,String>>) -> io::Result<()> {
    const NTHREAD : usize = 8;
    let worklists = splot_vec_into_chunks(filenames, NTHREAD);
    // Fork
    let mut thread_handles = vec![];
    for worklist in worklists {
        // clone only clones the Arc and bumps the reference count
        let glossary_for_child = glossary.clone();
        thread_handles.push(
            thread::spawn(move || process_files(worklist, &glossary_for_child))
        );
    }
    // Join
    for handle in thread_handles {
        handle.join().unwrap()?;
    }
    Ok(())
}

use rayon::prelude::*;

fn process_files_in_parallel2(filenames:Vec<String>,
    glossary : Arc<std::collections::HashMap<usize,String>>) -> io::Result<()> {
    filenames.par_iter()
        .map(|filename| process_files(filename.to_string(), &glossary))
        .reduce_with(|r1, r2| {
            if r1.is_err() {r1} else {r2}
        })
        .unwrap_or(Ok(()))
}

use std::{fs};
use std::sync::mpsc::{self, Receiver};

fn start_file_reader_thread(documents : Vec<PathBuf>)
     -> (mpsc::Receiver<String>,thread::JoinHandle<io::Result<()>>){
    let documents : Vec<PathBuf> = vec![];
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move || {
        for filename in documents {
            let text = fs::read_to_string(filename)?;
            if sender.send(text).is_err() {
                break;
            }
        }
        Ok(())
    });
    (receiver, handle)
}
enum InMemoryIndex {
    In
}
fn start_file_indexing_thread(texts : mpsc::Receiver<String>)
    -> (mpsc::Receiver<InMemoryIndex>, thread::JoinHandle<()>)
{
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move || {
        for (doc_id, text) in texts.into_iter().enumerate() {
            let index = InMemoryIndex::In;
            if sender.send(index).is_err() {
                break;
            }
        }
    });
    (receiver, handle)
}

fn start_in_memory_merge_thread(file_indexes : Receiver<InMemoryIndex>)
  -> (Receiver<InMemoryIndex>, JoinHandle<()>)
{
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move || {
        for fi in file_indexes {
            let index = InMemoryIndex::In;
            if sender.send(index).is_err() {
                break;
            }
        }
    });
    (receiver, handle)
}

fn  start_index_writer_thread(big_indexes : Receiver<InMemoryIndex>, output_dir : &Path)
    -> (Receiver<PathBuf>, JoinHandle<io::Result<()>>)
{
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move || {
        for index in big_indexes {
            let file = PathBuf::new();
            if sender.send(file).is_err() {
                break;
            }
        }
        Ok(())
    });
    (receiver, handle)
}

fn merge_index_files(files : Receiver<PathBuf>, output_dir : &Path) -> io::Result<()> {
    Ok(())
}

fn run_pipeline(documents : Vec<PathBuf>, output_dir : &PathBuf) -> io::Result<()> {
    let (texts, h1) = start_file_reader_thread(documents);
    let (pints, h2) = start_file_indexing_thread(texts);
    let (gallons, h3) = start_in_memory_merge_thread(pints);
    let (files, h4) = start_index_writer_thread(gallons, &output_dir);
    let result = merge_index_files(files, &output_dir);
    
    // wait for threads to finish
    let r1 = h1.join().unwrap();
    h2.join().unwrap();
    h3.join().unwrap();
    let r4 = h4.join().unwrap();
    r1?;
    r4?;
    result
}


pub trait OffThreadExt : Iterator {
    /// Transform this iterator into an off-thread iterator : the
    /// `next()` calls happen on a separate worker thread, so the
    /// iterator and the body of your loop run concurrently
    fn off_thread(self) -> mpsc::IntoIter<Self::Item>;
}

impl<T> OffThreadExt for T
    where T : Iterator + Send + 'static,
          T::Item : Send + 'static
{
    fn off_thread(self) -> mpsc::IntoIter<Self::Item> {
        let (sender, receiver) = mpsc::sync_channel(1024);
        thread::spawn(move || {
            for item in self {
                if sender.send(item).is_err() {
                    break;
                }
            }
        });
        receiver.into_iter()
    }
}

use std::sync::Mutex;
use std::sync::RwLock;

type PlayerId = u32;
const GAME_SIZE : usize = 8;
type WaitingList = Vec<PlayerId>;
struct FernEmpireApp {
    waiting_list : Mutex<WaitingList>,
    config : RwLock<String>,
}
impl FernEmpireApp {
    fn join_waiting_list(&self, player:PlayerId) {
        let mut guard = self.waiting_list.lock().unwrap();
        guard.push(player);
        if guard.len() == GAME_SIZE {
            let players = guard.split_off(0);
            drop(guard);  // don't keep the list locked while starting the gane
            self.start_game(players);
        }
    }
    fn start_game(&self, players : Vec<u32>) {

    }
    fn test(&self) -> String {
        self.config.read().unwrap().to_string()
    }
}
fn game_waiting_list() {


    let app = Arc::new(FernEmpireApp {
        waiting_list : Mutex::new(vec![]),
        config : RwLock::new(String::new()),
    });

}
use std::sync::atomic::AtomicUsize;

static PACKETS_SERVERD : AtomicUsize = AtomicUsize::new(0);

struct Color {
    red : u8,
    green : u8,
    blue : u8,
    alpha : u8,
}
const fn mono_to_rgba(level : u8) -> Color{
    Color {
        red : level,
        green : level,
        blue : level,
        alpha : 0xFF
    }
}

const WHITE : Color = mono_to_rgba(255);
const BLACK : Color = mono_to_rgba(0);
fn main() {
    use std::sync::atomic::{AtomicIsize, Ordering};
    let atom = AtomicIsize::new(0);
    atom.fetch_add(1, Ordering::SeqCst);
}
