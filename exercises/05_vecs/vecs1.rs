fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    // TODO: Create a vector called `v` which contains the exact same elements as in the array `a`.
    // Use the vector macro.
    let v = vec![10, 20, 30, 40];

    (a, v)
}

///This is equivalent to:
///```rust
///let _v = vec![10, 20, 30, 40];
///```
///(If you remove the `println!("")` ...)
fn main() {
    // You can optionally experiment here.
    let mut vmut: Vec<i32> = Vec::new();
    for i in [10, 20, 30, 40] {
        vmut.push(i);
        println!("Consider using vec![...] macro instead");
    }
    let _v = vmut;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
