fn main() {
    let mut x = 5;

    { // Create a new scope for the immutable reference
        let z = &x;  // z is an immutable reference to x
        println!("x = {}", *z); // Prints x = 5
    } // z goes out of scope here

    let y = &mut x; // y is a mutable reference to x
    *y += 1; // Modifying x through y
    println!("x = {}", x); // Prints x = 6
    *y += 1; // Modifying x through y again after the immutable reference is out of scope
    println!("x = {}", x); // Prints x = 7
} 