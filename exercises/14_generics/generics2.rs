// This powerful wrapper provides the ability to store a positive integer value.
// TODO: Rewrite it using a generic so that it supports wrapping ANY type.
struct Wrapper<T> {
    value: T,
}

// TODO: Adapt the struct's implementation to be generic over the wrapped value.
impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // You can optionally experiment here.
    let wrap = Wrapper::new("(w)Rapper");
    println!("Hello, {}!\n", wrap.value);

    // could just use this, why use ::new() ? -- Answer: convenience
    let simpler = Wrapper {
        value: "simpler use",
    };
    println!("Hello, {}!\n", simpler.value);

    // From: <https://doc.rust-lang.org/book/ch10-01-syntax.html#in-method-definitions>
    struct Point<Type> {
        x: Type,
        y: Type,
    }
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
        fn y(&self) -> &T {
            &self.y
        }
        fn new(x: T, y: T) -> Self {
            Point { x, y }
        }
    }
    fn print_p(name: &str, p: Point<i32>) {
        println!("{} = {{\n\tx = {},\n\ty = {},\n}}", name, p.x(), p.y());
    }
    let p1 = Point { x: 5, y: 10 };
    print_p("p1", p1);

    let p2 = Point::new(6, 7);
    print_p("p2", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
