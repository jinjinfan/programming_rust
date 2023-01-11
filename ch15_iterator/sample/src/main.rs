fn triangle(n:i32) -> i32 {
    (1..=n).fold(0, |sum, item| sum + item)
}

use std::fmt::Debug;

fn dump<T,U>(t : T)
    where T : IntoIterator<Item=U>,
          U : Debug 
{
    for u in t {
        println!("{:?}", u);
    }
}

use rand::random;
use std::iter::from_fn;


fn iterator() {
    println!("Triangle: {}", triangle(10));

    // Iterator and IntoIterator
    //println!("There is: ");
    let v = vec!["antimony", "arsenic", "alumium", "selenium"];
    for element in &v {
        //println!("{}", element);
    }
    let mut iterator = (&v).into_iter();
    while let Some(element) = iterator.next() {
       //println!("{}", element);
    }

    // iter or iter_mut
    use std::ffi::OsStr;
    use std::path::Path;

    let path = Path::new("C:/Users/Jinjin/Downloads/Fedora.iso");
    let mut iterator = path.iter();
    assert_eq!(iterator.next(), Some(OsStr::new("C:")));
    assert_eq!(iterator.next(), Some(OsStr::new("Users")));
    assert_eq!(iterator.next(), Some(OsStr::new("Jinjin")));
    assert_eq!(iterator.next(), Some(OsStr::new("Downloads")));
    assert_eq!(iterator.next(), Some(OsStr::new("Fedora.iso")));

    //from_fn and syccessors
    let lengths : Vec<f64> = 
        from_fn(|| Some((random::<f64>() - random::<f64>()).abs()))
        .take(1000)
        .collect();
    
    use num::Complex;
    use std::iter::successors;
    fn escape_time(c: Complex<f64>, limit: usize) ->Option<usize> {
        let zero = Complex{re:0.0, im:0.0};
        successors(Some(zero), |&z|{Some(z*z +c)})
            .take(limit)
            .enumerate()
            .find(|(_i, z)| z.norm_sqr() > 4.0)
            .map(|(i,_z)|i)
    }
    fn fibonacci() -> impl Iterator<Item=usize> {
        let mut state = (0,1);
        std::iter::from_fn(move || {
            state = (state.1, state.0 + state.1);
            Some(state.0)
        })
    }
    assert_eq!(fibonacci().take(8).collect::<Vec<_>>(),
               vec![1,1,2,3,5,8,13,21]);

    let mut outer = "Earth".to_string();
    let inner = String::from_iter(outer.drain(1..4));
    assert_eq!(outer, "Eh");
    assert_eq!(inner, "art");
}

fn iterator_adapter() {
    let text = " ponies  \n   giraffes\niguanas   \nsquid".to_string();
    let v : Vec<&str> = text.lines()
        .map(str::trim)
        .filter(|s| *s != "iguanas")
        .collect();
    assert_eq!(v, ["ponies", "giraffes", "squid"]);

    use std::str::FromStr;
    let text = "1\nfrond .25   289\n3.1415 estuary\n";
    for number in text
        .split_whitespace()
        .filter_map(|w| f64::from_str(w).ok())
    {
        println!("{:4.2}", number);
    }

    use std::collections::HashMap;
    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo","Kyoto"]);
    major_cities.insert("USA", vec!["Portland","Nashville"]);
    major_cities.insert("Brazil", vec!["San Paulo","Brasilia"]);
    major_cities.insert("Kenya", vec!["Nairobi","Mombassa"]);
    major_cities.insert("The Nertherlands", vec!["Amsterdam","Utrecht"]);

    let coutries = ["Japan", "Brazil", "Kenya"];
    for &city in coutries.iter().flat_map(|country| &major_cities[country]) {
        println!("{}", city);
    }

    use std::collections::BTreeMap;
    let mut parks = BTreeMap::new();
    parks.insert("Portland", vec!["Park1", "Park2"]);
    parks.insert("Kyoto", vec!["Forest1", "Forest2"]);
    parks.insert("Nashville", vec!["Park3", "Park4"]);
    let all_parks :Vec<_> = parks.values().flatten().cloned().collect();
    assert_eq!(all_parks, vec!["Forest1", "Forest2", "Park3", "Park4","Park1", "Park2"]);

    assert_eq!(vec![None,Some("day"), None, Some("one")]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
            vec!["day", "one"]);
    fn to_uppercase(data :&String) -> String {
        data.chars()
        .flat_map(char::to_uppercase)
        .collect()
    }
    let message = "To: jimb\r\n\
               From: superego <editor@oreilly.com>\r\n\
               \r\n\
               Did you get any writing done today?\r\n\
               When will you stop wasting time plotting fractals?\r\n";
    for header in message.lines().take_while(|l| !l.is_empty()) {
        println!("{}", header);
    }
    for body in message.lines()
        .skip_while(|l| !l.is_empty())
        .skip(1) {
        println!("{}", body);
    }

    use std::iter::Peekable;

    fn parse_number<I>(tokens : &mut Peekable<I>) -> u32 
        where I : Iterator<Item = char>
    {
        let mut n = 0;
        loop {
            match tokens.peek() {
                Some(r) if r.is_digit(10) => {
                    n = n * 10 + r.to_digit(10).unwrap();
                }
                _ => return n
            }
            tokens.next();
        }
    }

    let mut chars = "226153980,1766319049".chars().peekable();
    assert_eq!(parse_number(&mut chars), 226153980);
    assert_eq!(chars.next(), Some(','));
    assert_eq!(parse_number(&mut chars), 1766319049);
    assert_eq!(chars.next(), None);

    struct Flaky(bool);
    impl Iterator for Flaky {
        type Item = &'static str;
        fn next(&mut self) -> Option<Self::Item> {
            if self.0 {
                self.0 = false;
                Some("totally the last item")
            } else {
                self.0 = true;
                None
            }
        }
    }
    let mut flaky = Flaky(true);
    assert_eq!(flaky.next(), Some("totally the last item"));
    assert_eq!(flaky.next(), None);
    assert_eq!(flaky.next(), Some("totally the last item"));
    assert_eq!(flaky.next(), None);

    let mut not_flaky = Flaky(true).fuse();
    assert_eq!(not_flaky.next(), Some("totally the last item"));
    assert_eq!(not_flaky.next(), None);
    assert_eq!(not_flaky.next(), None);

    let bee_parts = ["head", "thorax", "abdomen"];
    let mut iter = bee_parts.iter();
    assert_eq!(iter.next(), Some(&"head"));
    assert_eq!(iter.next_back(), Some(&"abdomen"));
    assert_eq!(iter.next(), Some(&"thorax"));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);

    let meals = ["breakfast", "lunch", "dinner"];
    let mut iter = meals.iter().rev();
    assert_eq!(iter.next(), Some(&"dinner"));
    assert_eq!(iter.next(), Some(&"lunch"));
    assert_eq!(iter.next(), Some(&"breakfast"));
    assert_eq!(iter.next(), None);

    let upper_case : String = "grobe".chars()
        .inspect(|c| println!("before: {:?}", c))
        .flat_map(|c| c.to_uppercase())
        .inspect(|c| println!("after: {:?}", c))
        .collect();
    assert_eq!(upper_case, "GROBE");

    let v : Vec<i32> = (1..4).chain([20,30,40]).collect();
    assert_eq!(v, [1,2,3,20,30,40]);
    let v : Vec<i32> = (1..4).chain([20,30,40]).rev().collect();
    assert_eq!(v, [40,30,20,3,2,1]);

    let v : Vec<_> = (0..).zip("ABCD".chars()).collect();
    assert_eq!(v, vec![(0, 'A'),(1, 'B'), (2, 'C'), (3, 'D')]);

    use std::iter::repeat;
    let endings = ["once", "twice", "chicken soup with rice"];
    let rhyme : Vec<_> = repeat("going")
        .zip(endings)
        .collect();
    assert_eq!(rhyme, vec![("going", "once"),
                            ("going", "twice"),
                            ("going", "chicken soup with rice")
    ]);

    let message = "To: jimb\r\n\
               From: superego <editor@oreilly.com>\r\n\
               \r\n\
               Did you get any writing done today?\r\n\
               When will you stop wasting time plotting fractals?\r\n";
    let mut lines = message.lines();
    println!("Headers:");
    for header in lines.by_ref().take_while(|l| !l.is_empty()) {
        println!("{}", header);
    }
    println!("\nBody:");
    for body in lines {
        println!("{}", body);
    }

    let a = ['1', '2', '3', '$'];
    assert_eq!(a.iter().next(), Some(&'1'));
    assert_eq!(a.iter().cloned().next(), Some('1'));

    use std::iter::{once};
    let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
    let buzzes = repeat("").take(4).chain(once("buzz")).cycle();

    let fizzes_buzzes = fizzes.zip(buzzes);

    let fizz_buzz = (1..10).zip(fizzes_buzzes)
        .map(|tuple| 
                match tuple {
                    (i, ("", "")) => i.to_string(),
                    (_, (fizz, buzz)) => format!("{}{}", fizz, buzz)
                });
    for line in fizz_buzz {
        println!("{}", line);
    }

}

fn consumer_iterator() {
    fn triangle(n : u64) -> u64 {
        (1..=n).sum()
    }
    assert_eq!(triangle(20), 210);
    fn factorial(n : u64) -> u64 {
        (1..=n).product()
    }
    assert_eq!(factorial(20), 2432902008176640000);

    use std::cmp::Ordering;
    fn cmp(lhs:&f64, rhs:&f64) -> Ordering {
        lhs.partial_cmp(rhs).unwrap()
    }
    let numbers = [1.0, 4.0, 2.0];
    assert_eq!(numbers.iter().copied().max_by(cmp), Some(4.0));
    assert_eq!(numbers.iter().copied().min_by(cmp), Some(1.0));

    use std::collections::HashMap;
    let mut populations = HashMap::new();
    populations.insert("Portland",  583_776);
    populations.insert("Fossil",        449);
    populations.insert("Greenhorn",       2);
    populations.insert("Boring",      7_762);
    populations.insert("The Dalles", 15_340);
    assert_eq!(populations.iter().max_by_key(|&(_name, pop)| pop),
                Some((&"Portland", &583_776)));
    assert_eq!(populations.iter().min_by_key(|&(_name, pop)| pop),
                Some((&"Greenhorn", &2))); 
    assert_eq!(populations.iter().find(|&(_name, &pop)| pop > 1_000_000), None);
    assert_eq!(populations.iter().find(|&(_name, &pop)| pop > 500_000), 
            Some((&"Portland", &583_776)));

    let text = "Xerxes";
    assert_eq!(text.chars().position(|c| c == 'e'), Some(1));
    assert_eq!(text.chars().position(|c| c == 'z'), None);
    let bytes = b"Xerxes";
    assert_eq!(bytes.iter().rposition(|&c| c == b'e'), Some(4));
    assert_eq!(bytes.iter().rposition(|&c| c == b'X'), Some(0));

    let a = [5,6,7,8,9,10];
    assert_eq!(a.iter().fold(0, |n, _| n + 1), 6);
    assert_eq!(a.iter().fold(0, |n, i| n + i), 45);
    assert_eq!(a.iter().fold(1, |n, i| n * i), 151200);
    assert_eq!(a.iter().cloned().fold(i32::min_value(), std::cmp::max), 10);

    let a = ["Pack", "my", "box", "with",
         "five", "dozen", "liquor", "jugs"];
    let pangram = a.iter().fold(String::new(), |s, w| s + w + " ");
    assert_eq!(pangram, "Pack my box with five dozen liquor jugs ");
    let pangram1 = a.iter().rfold(String::new(), |s, w| s + w + " ");
    assert_eq!(pangram1, "jugs liquor dozen five with box my Pack ");
    
    let mut squares = (0..10).map(|i| i*i);
    assert_eq!(squares.nth(4), Some(16));
    assert_eq!(squares.nth(0), Some(25));
    assert_eq!(squares.last(), Some(81));

    let things = ["doorknob", "mushroom", "noodle", "giraffe", "grapefruit"];
    let (living, nonliving) : (Vec<&str>, Vec<&str>) 
        = things.iter().partition(|name| name.as_bytes()[0]& 1 != 0);
    assert_eq!(living,    vec!["mushroom", "giraffe", "grapefruit"]);
    assert_eq!(nonliving, vec!["doorknob", "noodle"]);

    ["doves", "hens", "birds"].iter()
        .zip(["turtle", "french", "calling"])
        .zip(2..5)
        .rev()
        .map(|((item, kind), quantity)| {
            format!("{} {} {}", quantity, kind, item)
        })
        .for_each(|gift| {
            println!("You have received: {}", gift);
        });
        
}
fn main() {
    iterator();
    iterator_adapter();
    consumer_iterator();
}