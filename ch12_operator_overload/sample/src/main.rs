#[derive(Clone, Copy, Debug)]
struct Complex<T> {
    re : T,
    im : T,
}

use std::ops::Add;

impl<T> Add for Complex<T> 
where
    T : Add<Output=T>,
{
    type Output  = Self;
    fn add(self, rhs:Self) -> Self {
        Complex {
            re : self.re + rhs.re,
            im : self.im + rhs.im,
        }
    }
}
/*
impl<L, R> Add<Complex<R>> for Complex<L> 
where
    L : Add<R>,
{
    type Output  = Complex<L::Output>;
    fn add(self, rhs:Complex<R>) -> Self::Output {
        Complex {
            re : self.re + rhs.re,
            im : self.im + rhs.im,
        }
    }
}
*/

use std::ops::Neg;

impl<T> Neg for Complex<T> 
where T : Neg<Output=T>,
{
    type Output = Complex<T>;
    fn neg(self) -> Complex<T> {
        Complex {
            re : -self.re,
            im : -self.im
        }
    }
}

use std::ops::AddAssign;
impl<T> AddAssign for Complex<T>
where T : AddAssign<T>,
{
    fn add_assign(&mut self,rhs:Complex<T>){
        self.re += rhs.re;
        self.im += rhs.im;
    }
}
impl<T:PartialEq> PartialEq for Complex<T> {
    fn eq(&self, other: &Complex<T>) -> bool {
        self.re == other.re && self.im == other.im
    }
}

impl<T:Eq> Eq for Complex<T> {}

#[derive(Debug, PartialEq)]
struct Interval<T> {
    lower : T,
    upper : T,
}

use std::cmp::{Ordering, PartialOrd};
impl<T:PartialOrd> PartialOrd<Interval<T>> for Interval<T> {
    fn partial_cmp(&self, other:&Interval<T>) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.lower >= other.upper {
            Some(Ordering::Greater)
        } else if self.upper <= other.lower {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

#[test]
fn test_interval() {
    assert!(Interval{lower: 10, upper: 20} < Interval{lower: 20, upper: 40});
    assert!(Interval{lower: 7, upper: 8} >= Interval{lower: 0, upper: 1});
    assert!(Interval{lower: 7, upper: 8} <= Interval{lower: 7, upper: 8});

    let left = Interval{lower: 10, upper: 30};
    let right = Interval{lower: 20, upper: 40};
    assert!(!(left < right));
    assert!(!(left >= right));
}

struct Image<P> {
    width : usize,
    pixels : Vec<P>,
}

impl<P: Default + Copy> Image<P> {
    fn new(width :usize, height:usize) -> Image<P> {
        Image {
            width,
            pixels :vec![P::default();width* height],
        }
    }
}

impl<P> std::ops::Index<usize> for Image<P> {
    type Output = [P];
    fn index(&self, row : usize) -> &[P] {
        let start = row * self.width;
        &self.pixels[start..start + self.width]
    }
}

impl<P> std::ops::IndexMut<usize> for Image<P> {
    fn index_mut(&mut self, row : usize) -> &mut [P] {
        let start = row * self.width;
        &mut self.pixels[start..start + self.width]
    }
}

fn main() {
    println!("Hello, world!");
}
