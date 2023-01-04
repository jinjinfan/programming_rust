#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TimeUnit {
    Seconds, Minutes,Hours, Days, Months, Years,
}

impl TimeUnit {
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }
    fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum RoughTime {
    InThePast(TimeUnit, u32), // tuple variant
    JustNow,
    InTheFuture(TimeUnit, u32),
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Shape {
    Sphere {center : u32, radius : f32}, // struct variant
    Cuboid {corner1 : u32, corner2 : u32},
}

use std::collections::HashMap;
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

// An ordered collection of `T`s
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

struct TreeNode<T> {
    element : T,
    left : BinaryTree<T>,
    right : BinaryTree<T>,
}
impl<T:Ord> BinaryTree<T> {
    fn add(&mut self, value : T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new({TreeNode {
                    element : value,
                    left : BinaryTree::Empty,
                    right : BinaryTree::Empty,
                }}))
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value)
                } else {
                    node.right.add(value)
                }
            }
        }
    }
}

fn main() {
    let four_score_and_seven_years_ago = RoughTime::InThePast(TimeUnit::Years, 4 * 20 +7);
    let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Hours, 3);
    println!("four_score_and_seven_years_ago {:?}", four_score_and_seven_years_ago);
    println!("three_hours_from_now {:?}", three_hours_from_now);

    use self::BinaryTree::*;
    let jupiter_tree = NonEmpty(Box::new(TreeNode{
        element : "Jupiter",
        left : Empty,
        right : Empty,
    }));
    let mercury_tree = NonEmpty(Box::new(TreeNode{
        element : "Mercury",
        left : Empty,
        right : Empty,
    }));
    let mars_tree = NonEmpty(Box::new(TreeNode{
        element : "Mars",
        left : jupiter_tree,
        right : mercury_tree,
    }));
    let mut tree = BinaryTree::Empty;
    tree.add("Saturn");
    tree.add("Venus");

    #[derive(Debug)]
    struct Account {
        name : String,
        language : String,
        id : u32,
    }

    let account = Account {
        name: "a".to_string(),
        language : "english".to_string(),
        id : 1
    };
    match account {
        Account{ref name, ref language, ..} => {
            println!("name: {:?} language: {}", name, language);
            println!("account: {:?}", &account);
        }
    }

    fn point_to_hex(click : u32) -> Option<u32> {
        Some(1)
    }
    
    //match guard
    let click = 1;
    let current_click = 2;
    let _ : Result<u32, &str> = match point_to_hex(click) {
        None => Err("That's not a game space"),
        Some(hex) if hex == current_click => Err("You have already here!"),
        Some(hex) => Ok(hex)
    };

}
