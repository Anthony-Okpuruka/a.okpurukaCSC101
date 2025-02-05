fn main() {
	let n1 = "Electrical".to_string();
	let n2 = "Electronic".to_string();
	let n3 = "Engineering".to_string();
	let n4 = n1 + &n2 + &n3; // n2 & n3 reference is passed

	// About Electrical/Electronics
    println!("\nThe {} is informed by the aspiration to train 
    	electrical/electronics engineering professionals in the areas
    	of design, building and maintenance of electrical control systems", n4);

    let u1 = "Computer".to_string();
    let u2 = "Science".to_string();
    let u3 = u1 + &u2; //u2 reference is passed
    println!();
    println!("{} is aimed at developing content, creative,
    	innovative, entrepreneurial and ethically-minded person,
    	capable of creating value in the diverse fields of Computer Science.", u3);
}
