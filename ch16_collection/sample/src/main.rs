use std::collections::HashSet;

fn main() {
    let mut slice = [0,1,2,3];
    {
        let last = slice.last_mut().unwrap();
        assert_eq!(*last, 3);
        *last = 100;
    }
    assert_eq!(slice, [0,1,2,100]);
    
    let mut byte_vec = b"Missssssssisssppi".to_vec();
    let mut seen = HashSet::new();
    byte_vec.retain(|r| seen.insert(*r));
    assert_eq!(&byte_vec, b"Misp");

    #[derive(Debug)]
    struct Student {
        first_name  : String,
        last_name  : String
    }
    let mut students : Vec<Student> =  vec![];
    students.sort_by(|a,b|{
        let a_key = (&a.last_name, &a.first_name);
        let b_key = (&b.last_name, &b.first_name);
        a_key.cmp(&b_key)
    });

    assert_eq!([1,2,3,4].starts_with(&[1,2]), true);
    assert_eq!([1,2,3,4].starts_with(&[3,2]), false);

    use rand::seq::SliceRandom;
    use rand::thread_rng;
    students.shuffle(&mut thread_rng());

    let mut my_vec = vec![1,3,5,7,9];
    my_vec.retain(|&val| val <=4);
    println!("{:?}", my_vec);

    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::from(vec![2,3,8,6,9,5,4]);
    assert_eq!(heap.peek(), Some(&9));
    assert_eq!(heap.pop(), Some(9));
    assert_eq!(heap.pop(), Some(8));
    assert_eq!(heap.pop(), Some(6));

    use std::collections::binary_heap::PeekMut;
    let mut heap2 = BinaryHeap::from(vec![2,3,8,6,9,5,4]);
    if let Some(top) = heap2.peek_mut() {
        if *top > 10 {
            PeekMut::pop(top);
        }
    }
    assert_eq!(heap2.peek(), Some(&9));

    use std::collections::HashMap;
    let text  = String::from("i have dheds");
    let mut word_frequency : HashMap<&str, u32> = HashMap::new();
    for c in text.split_whitespace() {
        word_frequency.entry(c)
            .and_modify(|count| *count += 1)
            .or_insert(0);
    }

    struct Artifact {
        id : u32,
        name : String
    }
    impl PartialEq for Artifact {
        fn eq(&self, other : &Artifact) -> bool {
            self.id == other.id
        } 
    }
    impl Eq for Artifact {}
    use std::hash::{Hash, Hasher};
    impl Hash for Artifact {
        fn hash<H: Hasher>(&self, hasher : &mut H) {
            self.id.hash(hasher);
        }
    }
}
