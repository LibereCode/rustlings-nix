///btw, rustdoc (rendered with _render-markdown.nvim_)
///```markdown
///> [!INFO] changed this function slightly.
///> Changed type `Vec<i32>` to `&[i32]`,
///and added `.to_owned()`
fn fill_vec(vec_p: &[i32]) -> Vec<i32> {
    let mut vec = vec_p.to_owned();

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    ///**Simpler solution** (Also, use old version of: `fill_vec(Vec<i32>)`)
    ///```rust
    /// let vec1 = fill_vec(vec0.clone());
    ///```
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        let vec1 = fill_vec(&vec0);

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
