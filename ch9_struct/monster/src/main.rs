#[derive(Copy, Clone, Debug)]
enum BroomIntent {
    FetchWater,
    DumpWater
}

#[derive(Debug)]
struct Broom {
    name : String,
    height : u32,
    health : u32,
    position : (f32, f32, f32),
    intent : BroomIntent
}

fn chop(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom{height:b.height/2, ..b};
    let mut broom2 = Broom{name: broom1.name.clone(), ..broom1};
    broom1.name.push_str(" I");
    broom2.name.push_str(" II");
    (broom1, broom2)
}

pub struct Queue<T> {
    older : Vec<T>,
    younger : Vec<T>
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue{older:Vec::new(), younger:Vec::new()}
    }
    pub fn push(&mut self, c:T) {
        self.younger.push(c);
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        self.older.pop()
    }
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }
}

pub struct Vector2 {
    x : f32,
    y : f32
}

impl Vector2 {
    const ZERO : Vector2 = Vector2{x:0.0, y : 0.0};
    const UNIT : Vector2 = Vector2{x:1.0, y : 0.0};
    const NAME : &'static str =  "Vector2";
    const ID: u32 = 18;
}

// struct with lifetime paremeters
#[derive(Debug)]
struct Extrema<'elt> {
    greatest : &'elt i32,
    least : &'elt i32,
}

fn find_extrema<'s>(slice : &'s [i32]) -> Extrema<'s> {
    let mut greatest = &slice[0];
    let mut least = &slice[0];
    for i in 1..slice.len() {
        if slice[i] < *least {least = &slice[i]};
        if slice[i] > *greatest {greatest = &slice[i]};
    }
    Extrema{greatest, least}
}

// Polynomial with const generic type
#[derive(Debug)]
struct Polynomial<const N: usize> {
    coefficients: [f64;N]
}

impl<const N :usize> Polynomial<N> {
    fn new(coefficients : [f64;N]) -> Polynomial<N> {
        Polynomial{coefficients}
    }
    fn eval(&self, x : f64) -> f64 {
        let mut sum = 0.0;
        for i in (0..N).rev() {
            sum = self.coefficients[i] + x * sum;
        }
        sum
    }
}
fn main() {
    let hokey  = Broom {
        name : "Hokey".to_string(),
        height: 60,
        health : 100,
        position : (100.0, 200.0, 0.0),
        intent :BroomIntent::FetchWater,
    };
    let (hokey1, hokey2) = chop(hokey);
    println!("hokey1 : {:?}", hokey1);
    println!("hokey1 : {:?}", hokey2);

    //
    let mut q = Queue{older:Vec::new(), younger : Vec::new()};
    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));
    q.push('2');
    let (older, younger) = q.split();
    println!("older: {:?}", older);
    println!("younger: {:?}", younger);

    // find greast and least
    let a = [0, -3, 0, 15, 48];
    let e = find_extrema(&a);
    println!("Greast and lease: {:?}", e);

    //
    use std::f64::consts::FRAC_PI_2;
    let sine_poly = Polynomial::new([0.0,1.0, 0.0, -1.0/6.0, 0.0, 1.0/120.0]);
    assert_eq!(sine_poly.eval(0.0), 0.0);
    assert!((sine_poly.eval(FRAC_PI_2)-1.).abs() < 0.005);

    //interior mutability
    use std::cell::RefCell;
    let ref_cell : RefCell<String> = RefCell::new("hello".to_string());
    let mut r = ref_cell.borrow_mut();
    r.push_str(" world");
    println!("ref count : {:?}", r.len());
}
