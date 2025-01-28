fn main() {
    let mut x = 5;
    { // Create a new scope to limit the lifetime of the borrow 
        let y = &mut x;
        *y += 1;
    }
    { // Create a new scope to limit the lifetime of the borrow
        let z = &mut x;
        *z += 2;
    }
    println!("x = {}", x);
}