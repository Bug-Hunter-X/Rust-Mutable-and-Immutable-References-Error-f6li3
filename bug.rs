fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x;  // z is an immutable reference to x

    *y += 1; // Modifying x through y
    println!("x = {}", x); // Prints x = 6

    // The following line will cause a compile-time error
    // because we are trying to modify x through y after creating an immutable reference z
    *y += 1; 
}