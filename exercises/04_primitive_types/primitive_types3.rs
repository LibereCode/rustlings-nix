fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let a = "Crazy? I was crazy once. Locked in a room. A rubber-room. Rubber-room with rats. Rats make me crazy.".repeat(10);
    println!("Length == {}:\n{}", a.len(), a);

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
