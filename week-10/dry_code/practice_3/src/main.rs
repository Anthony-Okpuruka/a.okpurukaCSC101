fn main() {
    // Create a String variable 'first' with the value "Santa Claus"
    let first = "Santa Claus".to_string();

    // Create a string slice 'noel' that references a part of 'first' from index 3 to 10
    let noel = &first[3..10];

    // Print the value of 'noel' to the console
    println!("{}", noel);
}