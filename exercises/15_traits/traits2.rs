trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement the trait `AppendBar` for a vector of strings.
// `append_bar` should push the string "Bar" into the vector.

impl AppendBar for Vec<String> {
    ///
    /// ```rust old version
    /// /// **Old version:**
    /// fn append_bar(self) -> Self {
    ///     let mut v = self;
    ///     v.push(String::from("Bar"));
    ///     v
    /// }
    ///
    /// // TESTS
    /// let mut foo = vec![String::from("Foo"), "AAAAAAAAAA"].append_bar();
    /// assert_eq!(foo.pop(), String::from("Bar"));
    /// assert_eq!(foo.pop(), "AAAAAAAAAA");
    /// assert_eq!(foo.pop().unwrap(), "Foo");
    /// assert_eq!(foo.pop(), None);
    /// ```
    fn append_bar(mut self) -> Self {
        self.push(String::from("Bar"));
        self
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
