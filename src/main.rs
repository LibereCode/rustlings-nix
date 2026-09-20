use std::io::{self, Write};
fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.write_all(b"main.rs larp, ")?;
    stdout.flush()?; // Manual flush
    stdout.write_all(b"stdout::write_all() btw\n")?; // Automatically flushed

    using_foobar();

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
