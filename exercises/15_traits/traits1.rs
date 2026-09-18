// The trait `AppendBar` has only one function which appends "Bar" to any object
// implementing this trait.
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for the type `String`.
    fn append_bar(self) -> Self {
        self + "Bar"
    }
}
impl AppendBar for i64 {
    // TODO: Implement `AppendBar` for the type `String`.
    fn append_bar(self) -> Self {
        self * 2
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {s}");

    let i = 69;
    let i = i.append_bar();
    println!("i: {i}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), "FooBar");
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(String::from("").append_bar().append_bar(), "BarBar");
    }
}
