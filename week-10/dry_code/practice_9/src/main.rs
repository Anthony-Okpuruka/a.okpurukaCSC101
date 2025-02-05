fn main() {
    // Initialize mutable variables 'game' and 'two'
    let mut game: f32 = 25.0;
    let mut two: f32 = 15.0;

    // Check if 'game' is greater than 0
    if game > 0.0 {
        // Increment 'game' by 3
        game += 3.0;
        // Decrement 'two' by 2
        two -= 2.0;
        // Calculate 'grass' as the sum of 'game' and 'two'
        let grass = game + two;
        // Set 'game' to half of 'grass'
        game = grass / 2.0;
        // Set 'two' to three times 'game'
        two = game * 3.0;
        // Print the values of 'grass', 'game', and 'two' to the console
        println!("grass, game and two are: {}, {}, {}", grass, game, two);
    }
}