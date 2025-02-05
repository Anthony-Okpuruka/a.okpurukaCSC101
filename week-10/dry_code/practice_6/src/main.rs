fn main() {
    // Initialize mutable variables 'lab', 'class', 'min', and 'max'
    let mut lab = 15;
    let mut class = 50;
    let mut min = 4;
    let mut max = 7;

    // Loop while 'lab' is less than 'class'
    while lab < class {
        // Increment 'lab' by 'min'
        lab += min;
        
        // Decrement 'class' by 'max'
        class -= max;
        
        // Print the current value of 'class' to the console
        println!("The value of class = {}", class);
    }
}