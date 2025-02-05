fn main() {
    // Create an array 'data' with string elements
    let data = ["Ade", "Ola", "Joye", "Adam", "Yemi", "Sam", "Tola"];
    
    // Call the 'pass_me' function, passing a slice of 'data' starting from index 4 to the end
    pass_me(&data[4..]);
}

fn pass_me(use_data: &[&str]) {
    // Print the length of the 'use_data' slice
    println!("The length of use_data is: {:?}", use_data.len());
    
    // Print the contents of the 'use_data' slice
    println!("{:?}", use_data);
}