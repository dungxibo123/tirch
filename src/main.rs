mod core;
use core::vector::Vector;

fn main() {
    let a = vec!(1i32,2i32, -2i32, -5i32, -9i32, -1i32);
    let c = vec!(0i32,2i32, 7i32, 5i32, 6i32, -5i32);
    let b = Vector::new(a);
    let d = Vector::new(c);

    println!("Intialized"); 
    println!("{}", &b + &d);
    println!("{}", &b - &d); 
    
     
}


