fn main() {
    // Declare a mutable integer variable 'link' and initialize it with the value 25
    let mut link: i32 = 25;
    
    // Call the 'sledge' function, passing 'link' as an argument
    sledge(link);
    
    // Multiply the value of 'link' by 3 and reassign the result to 'link'
    link = link * 3;
    
    // Print the value of 'link' to the console
    println!("The value of link is: {}", link);
}

// Define the 'sledge' function, which takes a mutable integer parameter 'go_link'
fn sledge(mut go_link: i32) {
    // Divide the value of 'go_link' by 5 and reassign the result to 'go_link'
    go_link = go_link / 5;
    
    // Print the value of 'go_link' to the console
    println!("The value of go_link is: {}", go_link);
}