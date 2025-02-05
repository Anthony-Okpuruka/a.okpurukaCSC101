fn main() {
    // Loop through the range from 20 to 28 (inclusive of 20, exclusive of 29)
    for magic_key in 20..29 {
        
        // If the value of 'magic_key' is less than or equal to 25, skip the rest of the loop iteration
        if magic_key <= 25 {
            continue;
        }
        
        // Print the value of 'magic_key' to the console
        println!("The key is: {}", magic_key);
    }
}