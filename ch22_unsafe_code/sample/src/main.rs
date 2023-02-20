mod my_ascii {
    #[derive(Debug, Eq, PartialEq)]
    pub struct Ascii (
        Vec<u8>
    );
    impl Ascii {
       pub fn from_bytes(bytes:Vec<u8>) -> Result<Ascii, NotAsciiError> {
        if bytes.iter().any(|&byte| !byte.is_ascii()) {
            return Err(NotAsciiError(bytes));
        }
        Ok(Ascii(bytes))
       } 
    }
    #[derive(Debug, Eq, PartialEq)]
    pub struct NotAsciiError(pub Vec<u8>);

    impl From<Ascii> for String {
        fn from(value: Ascii) -> Self {
            unsafe {
                String::from_utf8_unchecked(value.0)
            }
        }
    }
}
/*
use core::nonzero::Zeroable;

fn zeroed_vector<T>(len:usize) -> Vec<T>
    where T:Zeroable
{
    let mut vec = Vec::with_capacity(len);
    unsafe {
        std::ptr::write_bytes(vec.as_mut_ptr(), 0, len);
        vec.set_len(len)
    }
    vec
}
*/
use my_ascii::Ascii;
fn option_to_raw<T>(opt:Option<&T>) -> *const T {
    match opt {
        None => std::ptr::null(),
        Some(r) => r as *const T
    }
}

fn raw_pointer() {
    let mut x = 10;
    let ptr_x = &mut x as *mut i32;
    let y = Box::new(20);
    let ptr_y = &*y as *const i32;
    unsafe {
        *ptr_x += *ptr_y;
    }
    assert_eq!(x, 30);
    assert!(!option_to_raw(Some(&("pea", "pod"))).is_null());
    assert_eq!(option_to_raw::<i32>(None), std::ptr::null());

    let trucks = vec!["garbage truck", "dump truck", "moonstruck"];
    let first : *const &str = &trucks[0];
    let last  : *const &str = &trucks[2];
    assert_eq!(unsafe{last.offset_from(first)}, 2);
    assert_eq!(unsafe{first.offset_from(last)}, -2);
}

mod ref_with_flag {
    use std::cell::Ref;
    use std::marker::PhantomData;
    use std::mem::align_of;
    /// A `&T` and a `bool`, wrapped up in a single word.
    /// The type `T` must require at least two-byte aligment.
    pub struct RefWithFlag<'a, T> {
        ptr_and_bit : usize,
        behaves_like : PhantomData<&'a T> // occupies no space
    }

    impl<'a, T:'a> RefWithFlag<'a, T> {
        pub fn new(ptr : &'a T, flag : bool) -> RefWithFlag<T> {
            assert!(align_of::<T>() % 2 == 0);
            RefWithFlag { 
                ptr_and_bit: ptr as *const T as usize | flag as usize, 
                behaves_like: PhantomData
            }
        }
        pub fn get_ref(&self) ->&'a T {
            unsafe {
                let ptr = (self.ptr_and_bit & !1) as *const T;
                &*ptr
            }
        }
        pub fn get_flag(&self) -> bool {
            self.ptr_and_bit & 1 != 0
        }
    }
}

fn refWithFlag() {
    use ref_with_flag::RefWithFlag;
    let vec = vec![10, 20, 30];
    let flagged = RefWithFlag::new(&vec, true);
    assert_eq!(flagged.get_ref()[1], 20);
    assert_eq!(flagged.get_flag(), true);
}

fn offset<T>(ptr : *const T, count : isize) -> *const T
    where T : Sized
{
    let bytes_per_element = std::mem::size_of::<T>() as isize;
    let byte_offset = count * bytes_per_element;
    (ptr as isize).checked_add(byte_offset).unwrap() as *const T
}


use std;
use std::ops::Range;

pub struct GapBuffer<T> {
    // Storage for elements
    storage : Vec<T>,
    // Range of uninitialied elements in the middle of `storage`
    gap     : Range<usize>
}

impl<T> GapBuffer<T> {
    pub fn new() -> GapBuffer<T> {
        GapBuffer { storage: Vec::new(), gap: 0..0 }
    }
    pub fn capacity(&self) -> usize {
        self.storage.capacity()
    }
    pub fn len(&self) ->usize {
        self.capacity()-self.gap.len()
    }
    pub fn position(&self) -> usize {
        self.gap.start
    }
    unsafe fn space(&self, index : usize) -> *const T {
        self.storage.as_ptr().offset(index as isize)
    }
    unsafe fn space_mut(&mut self, index : usize) -> *mut T {
        self.storage.as_mut_ptr().offset(index as isize)
    }
    fn index_to_raw(&self, index:usize) -> usize {
        if index < self.gap.start {
            index
        } else {
            index + self.gap.len()
        }
    }

    pub fn get(&self, index : usize) -> Option<&T> {
        let raw = self.index_to_raw(index);
        if raw < self.capacity() {
            unsafe {
                Some(&*self.space(raw))
            }
        } else {
            None
        }
    }
    pub fn set_position(&mut self, pos:usize) {
        if pos > self.len() {
            panic!("index {} out of range for GapBuffer", pos);
        }
        unsafe {
            let gap = self.gap.clone();
            if pos > gap.start {
                let distance = pos-gap.start;
                std::ptr::copy(self.space(gap.end), 
                                self.space_mut(gap.start), distance);
            } else if pos < gap.start {
                let distance = gap.start - pos;
                std::ptr::copy(self.space(pos),
                                self.space_mut(gap.end-distance),
                                distance)
            }
            self.gap = pos..pos + gap.len();
        }
    }
    pub fn insert(&mut self, elt : T) {
        if self.gap.len() == 0 {
            self.enlarge_gap();
        }
        unsafe {
            let index = self.gap.start;
            std::ptr::write(self.space_mut(index), elt);
        }
        self.gap.start += 1;
    }
    pub fn insert_iter<I>(&mut self, iterable : I)
        where I : IntoIterator<Item = T>
    {
        for item in iterable {
            self.insert(item)
        }
    }
    pub fn remove(&mut self) -> Option<T> {
        if self.gap.end == self.capacity() {
            return None;
        }
        let element = unsafe {
            std::ptr::read(self.space(self.gap.end))
        };
        self.gap.end += 1;
        Some(element)
    }
    fn enlarge_gap(&mut self) {
        let mut new_capacity = self.capacity()*2;
        if new_capacity == 0 {
            new_capacity = 4;
        }
        let mut new = Vec::with_capacity(new_capacity);
        let after_gap = self.capacity() - self.gap.end;
        let new_gap = self.gap.start .. new.capacity() - after_gap;
        unsafe {
            // Move the elements that fall before the gap
            std::ptr::copy_nonoverlapping(self.space(0), new.as_mut_ptr(), self.gap.start);
            // Move the elements that fall after the gap
            let new_gap_end = new.as_mut_ptr().offset(new_gap.end as isize);
            std::ptr::copy_nonoverlapping(self.space(self.gap.end), new_gap_end, after_gap);
        }
        self.storage = new;
        self.gap = new_gap;
    }
}

impl<T> Drop for GapBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0 .. self.gap.start {
                std::ptr::drop_in_place(self.space_mut(i));
            }
            for i in self.gap.end .. self.capacity() {
                std::ptr::drop_in_place(self.space_mut(i));
            }
        }
    }
}
fn gap_buffer()
{
    let mut buf = GapBuffer::new();
    buf.insert_iter("Lord of the Rings".chars());
    buf.set_position(12);
    buf.insert_iter("Onion ".chars());
}

union FloatOrInt {
    f : f32,
    i : i32
}

union SmallOrLarge {
    s : bool,
    l : u64
}

#[repr(C)]
union SignExtractor {
    value : i64,
    byte  : [u8;8]
}
fn sign(int : i64) -> bool {
    let se = SignExtractor{value : int};
    println!("{:b} ({:?})", unsafe{se.value}, unsafe {
        {se.byte}
    });
    unsafe{se.byte[7] >= 0b10000000}
}
fn main() {
    let bytes : Vec<u8> = b"ASCII and ye shall receive".to_vec();
    let ascii : Ascii = Ascii::from_bytes(bytes).unwrap();
    let string = String::from(ascii);
    assert_eq!(string, "ASCII and ye shall receive");
    //let v : Vec<usize> = zeroed_vector(100_000);
    //assert!(v.iter().all(|&u| u ==0));
    raw_pointer();
    refWithFlag();
    assert_eq!(std::mem::size_of::<i64>(), 8);
    assert_eq!(std::mem::align_of::<(i32, i32)>(), 4);
    let slice : &[i32] = &[1,3,9,27,81];
    assert_eq!(std::mem::size_of_val(slice), 20);
    let text : &str = "alligator";
    assert_eq!(std::mem::size_of_val(text), 9);

    use std::fmt::Display;
    let unremarkable : &dyn Display = &193_u8;
    let remarkable : &dyn Display = &0.0072793525664;
    assert_eq!(std::mem::size_of_val(unremarkable), 1);
    assert_eq!(std::mem::align_of_val(remarkable), 8);

    let u = SmallOrLarge{l : 1337};
    println!(" union SmallOrLarge {}", unsafe{u.l} );

    //sign
    assert_eq!(sign(-1), true);
    assert_eq!(sign(1), false);
    assert_eq!(sign(i64::MAX), false);
    assert_eq!(sign(i64::MIN), true);
    assert_eq!(sign(-1), false);
}
