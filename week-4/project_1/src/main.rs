// Rust program to find the roots of quadratic equation
use std::io;
fn main() {
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();
	println!("Enter value for a: ");
	io::stdin().read_line(&mut input1).expect("Not a valid String");
    let a:f32 = input1.trim().parse().expect("Not a valid number");
    println!("Enter value for b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid String");
    let b:f32 = input2.trim().parse().expect("Not avalid number");
    println!("Enter value for c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid String");
    let c:f32 = input3.trim().parse().expect("Not a valid number");
    let discriminant = b.powf(2.0) - 4.0 * a * c;
    if discriminant > 0.0 // The equation has two distinct real roots
    {  
    	let x1 = -b + discriminant.sqrt() / 2.0 * a;
    	let x2 = -b - discriminant.sqrt() / 2.0 * a;
    	println!(" There are two distinct roots: ");
    	println!("root 1 is {}", x1);
    	println!("root 2 is {}", x2);
    }
    else if discriminant == 0.0 // The equation has exactly one real root
    {   
        let x = -b / 2.0 * a; 
    	println!("There is one real root: ");
    	println!("root is {}", x)
    }
    else if discriminant < 0.0 // There are no real roots
    {
    	println!("The are no real roots ");
    }
   
}
