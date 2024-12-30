use std::io;
fn main() {
    let mut age_input = String::new();
    let mut experience_input = String::new();
    println!("Enter the age of the employee:");
    io::stdin().read_line(&mut age_input).expect("Failed to read line");
    let age: f32 = age_input.trim().parse().expect("Please enter a valid number");
    println!("Enter the years of experience of the employee:");
    io::stdin().read_line(&mut experience_input).expect("Failed to read line");
    let experience: f32 = experience_input.trim().parse().expect("Please enter a valid number");
    let incentive = calculate_incentive(age, experience);
    println!("The annual incentive for the employee is: {}", incentive);
}
// Function to calculate incentive based on age and experience
fn calculate_incentive(age: f32, experience: f32) -> f32 {
    if experience >= 5.0 {
        // Employee is experienced
        if age >= 40.0 {
            1_560_000.0  // N1,560,000 for employees aged 40 or more
        } else if age >= 30.0 {
            1_480_000.0  // N1,480,000 for employees aged between 30 and 39
        } else if age >= 28.0 {
            1_300_000.0  // N1,300,000 for employees aged below 28 but experienced
        } else {
            1_300_000.0  // Same incentive if age is below 28 and still experienced
        }
    } else {
        // Employee is inexperienced
        100_000.0  // N100,000 for inexperienced employees
    }
}
