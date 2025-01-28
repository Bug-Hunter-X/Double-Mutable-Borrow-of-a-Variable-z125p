fn main() {
    let mut x = 5;
    let y = &mut x; 
    let z = &mut x; // This is the problem. 
    *y +=1;
    *z +=2; 
    println!("x = {}", x);
}