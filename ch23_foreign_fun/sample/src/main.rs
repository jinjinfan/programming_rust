mod raw;

#[repr(C)]
enum Tag {
    Float = 0,
    Int   = 1
}
#[repr(C)]
union FloatOrInt {
    f : f32,
    i : i32,
}
#[repr(C)]
struct Value {
    tag : Tag,
    union : FloatOrInt
}

fn is_zero(v : Value) -> bool {
    use self::Tag::*;
    unsafe {
        match v {
            Value { tag : Int, union : FloatOrInt{i : 0}} => true,
            Value { tag : Float, union : FloatOrInt{ f : num}} => num == 0.0,
            _ => false
        }
    }
}
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::os::raw::c_uchar;

use std::ffi::CString;
use std::ffi::CStr;

extern {
    fn strlen(s : *const c_char) -> usize;
}
extern {
    static environ : *mut *mut c_char;
}


fn check(activity : &'static str, status : c_int) -> c_int {
    if status < 0 {
        unsafe {
            let error = &*raw::giterr_last();
            println!("error while {} : {} ({})",
                activity,
                CStr::from_ptr(error.message).to_string_lossy(), error.klass        
            );
            std::process::exit(1);
        }
    }
    status
}

unsafe fn show_commit(commit : *const  raw::git_commit) {
    let author = raw::git_commit_author(commit);
    let name = CStr::from_ptr((*author).name).to_string_lossy();
    let email = CStr::from_ptr((*author).email).to_string_lossy();
    println!("{} <{}>\n", name, email);
    let message = raw::git_commit_message(commit);
    println!("{}", CStr::from_ptr(message).to_string_lossy());
}

use std::mem;
use std::ptr;


fn main() {
    let rust_str = "I'll be back";
    let null_terminated = CString::new(rust_str).unwrap();
    unsafe {
        assert_eq!(strlen(null_terminated.as_ptr()), 12);
    }
    unsafe {
        if !environ.is_null() && !(*environ).is_null() {
            let var = CStr::from_ptr(*environ);
            println!("first environment variable : {}", var.to_string_lossy())
        }
    }
    
    let path = std::env::args().skip(1).next().expect("usage: sample PATH");
    let path = CString::new(path).expect("path contains null characters");
    unsafe {
        check("initializing library", raw::git_libgit2_init());
        let mut repo = ptr::null_mut();
        check("opening repository", raw::git_repository_open(&mut repo, path.as_ptr()));

        let c_name = b"HEAD\0".as_ptr() as *const c_char;
        let oid = {
            let mut oid = mem::MaybeUninit::uninit();
            check("looking up HEAD", raw::git_reference_name_to_id(oid.as_mut_ptr(), repo, c_name));
            oid.assume_init()
        };

        let mut commit = ptr::null_mut();
        check("looking up commit", raw::git_commit_lookup(&mut commit, repo, &oid));

        show_commit(commit);
        raw::git_commit_free(commit);
        raw::git_repository_free(repo);
        check("shutting down library", raw::git_libgit2_shutdown());


    }
}
