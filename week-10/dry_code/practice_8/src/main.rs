fn main() {
    // Outer loop: iterate over the range from 8 to 9 (inclusive of 8, exclusive of 10)
    for num1 in 8..10 {
        // Middle loop: iterate over the range from 16 to 16 (inclusive of 16, exclusive of 17)
        for num2 in 16..17 {
            // Inner loop: iterate over the range from 15 to 16 (inclusive of 15, exclusive of 17)
            for num3 in 15..17 {
                // Calculate 'val' as the sum of 'num1', 'num2', and 'num3'
                let val = num1 + num2 + num3;
                // Print the value of 'val' to the console
                println!("{}", val);
            }
        }
    }
}