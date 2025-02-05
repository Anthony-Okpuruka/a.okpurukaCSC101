fn main() {
    // Outer loop: iterate over the range from 29 to 30 (inclusive of 29, exclusive of 31)
    for x in 29..31 {
        // Inner loop: iterate over the range from 15 to 16 (inclusive of 15, exclusive of 17)
        for mut m in 15..17 {
            // Increment 'm' by 3
            m += 3;
            // Calculate 'z' as the sum of 'm' and 'x'
            let z = m + x;
            // Print the value of 'z' to the console
            println!("The value of z is: {} \n", z);
        }
    }
    //println!("x is {}", x); // This line is commented out and will not be executed
}