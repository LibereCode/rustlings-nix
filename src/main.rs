use std::io::{self, Write};
fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.write_all(b"main.rs larp, ")?;
    stdout.flush()?; // Manual flush
    stdout.write_all(b"stdout::write_all() btw\n")?; // Automatically flushed

    using_foobar();

    // counter();

    list_2_box();

    // from_hashmaps_vikings();

    Ok(())
}

fn using_foobar() {
    println!("\nFrom the function `foobar()`:");
    let returned_value = foobar();
    println!("- This is the returned value: {}!", returned_value.as_str());
}

/// Rustdoc comment, btw
/// ```rust
/// // this is evaluated in tests (I think?):
/// asserteq!(foobar(), String::from("Hello World"))
/// ```
fn foobar() -> String {
    let msg = String::from("Hello World");
    println!("- This is printed during call: {}?", msg.as_str());
    msg
}

// /// A simple hello-world, that will print "Hello, World!".
// ///
// /// All the variables used here aren't needed.
// /// The type annotation for the variables aren't needed.
// ///
// /// ```rust main.rs
// /// /// A more simple "Hello, World!" (that will do the same):
// /// fn main() {
// ///     println!("Hello, World!")
// /// }
// /// ```
// fn main() {
//     let mut hi: String = String::from("Hello");
//     let comma: &str = ", ";
//     hi += comma;
//     let planet = "World";
//     println!("{}{}!", hi, planet)
// }

// /// Count to ten (See also `counter2`)
// /// [!WARN]
// /// `toolazy` will not work, becaue it is... too lazy.
// fn counter() {
//     for i in 1..=10 {
//         println!("{}", i);
//     }
//     // let toolazy = (1..=10).map(|i| println!("{}", i)); // BAD
// }

fn list_2_box() {
    #[derive(Debug)]
    // enum Value {
    //     I32(i32),
    //     U32(u32),
    //     Text(String),
    // }
    // /// Is this how macros are made?
    // macro_rules! impl_from {
    // ($($ty:ty => $variant:ident),* $(,)?) => {
    //         $(impl From<$ty> for Value {
    //             fn from(v: $ty) -> Self {
    //                 Value::$variant(v.into())
    //             }
    //         })*
    //     };
    // }
    // impl_from!(
    //     i32 => I32,
    //     u32 => U32,
    //     String => Text,
    // );

    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    fn list_from_slice(values: &[i32]) -> List {
        match values {
            [] => List::Nil,
            [first, rest @ ..] => List::Cons(*first, Box::new(list_from_slice(rest))),
        }
    }

    fn print_list(list: &List) {
        match list {
            List::Cons(value, next) => {
                println!("{}", value);
                print_list(next);
            }
            List::Nil => {}
        }
    }

    // let values: Vec<Value> = vec![42.into(), 100u32.into(), String::from("hello").into()];
    let values = [1, 42, 723];

    let list = list_from_slice(&values);
    println!();
    println!("Debug pretty-print:\n{:#?}", &list);
    println!();
    println!("match print:");
    print_list(&list);
}

// /// This is an example from the lib-src of HashMap
// /// (find it by using lsp-hover on "**HashMap**" in `use std::collections::HashMap`)
// fn from_hashmaps_vikings() {
//     use std::collections::HashMap;
//
//     #[derive(Hash, Eq, PartialEq, Debug)]
//     struct Viking {
//         name: String,
//         country: String,
//     }
//
//     impl Viking {
//         /// Creates a new Viking.
//         fn new(name: &str, country: &str) -> Viking {
//             Viking {
//                 name: name.to_string(),
//                 country: country.to_string(),
//             }
//         }
//     }
//
//     // pretty new-line
//     println!();
//
//     // Use a HashMap to store the vikings' health points.
//     let vikings = HashMap::from([
//         (Viking::new("Einar", "Norway"), 25),
//         (Viking::new("Olaf", "Denmark"), 24),
//         (Viking::new("Harald", "Iceland"), 12),
//     ]);
//
//     // Use derived implementation to print the status of the vikings.
//     for (viking, health) in &vikings {
//         println!("{viking:?} has {health} hp");
//     }
// }
