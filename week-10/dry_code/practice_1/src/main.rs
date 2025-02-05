fn main() {
    let mut wood: i32 = 35; // Initialize wood with value 35
    bush(&mut wood); // Pass wood by mutable reference to the bush function
    wood = wood * 2; // Double the value of wood
    println!("The value of wood is: {}", wood); // Print the final value of wood
}

fn bush(plank: &mut i32) {
    *plank = *plank / 5; // Divide the value of plank (which is wood) by 5
    let wood = *plank / 3; // Calculate wood as plank divided by 3 (not used outside this function)
    println!("The value of plank is: {}", plank); // Print the value of plank
}