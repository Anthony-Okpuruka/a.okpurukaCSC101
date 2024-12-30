use std::io;

 fn main() {
    println!("What would you like to calculate?");
    println!("
              a = Area of Trapezium  
              b = Area of the rhombus  
              c = Area of Parallelogram 
              d = Area of Cube 
              e = Volume of Cylinder");

    let  mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("invalid input");
    let input1 = input1.trim();
    println!("Your are calculating: {}", input1);

    if  input1 == "a" {
    
    println!("\ninput base1");
    let mut base1 = String::new();
    io::stdin().read_line(&mut base1).expect("invalid input");
    let base1:f32 = base1.trim().parse().expect("wrong input");
    println!("Your base1 is {}", base1);
    
    println!("\ninput base2");
    let mut base2 = String::new();
    io::stdin().read_line(&mut base2).expect("invalid input");
    let base2:f32 = base2.trim().parse().expect("wrong input");
    println!("Your base2 is {}", base2);
    
    println!("\ninput height");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("invalid input");
    let height:f32 = height.trim().parse().expect("wrong input");
    println!("Your height is {}", height);
    
    let _areaa = 0.5 * (base1 + base2) * height;

    println!("\nArea of Trapezium {}", _areaa);
    }
    else if input1 == "b"{
    
    println!("\ninput diagonal1");
    let mut diagonal1 = String::new();
    io::stdin().read_line(&mut diagonal1).expect("invalid input");
    let diagonal1:f32 = diagonal1.trim().parse().expect("wrong input");
    println!("Your diagonal1 is {}", diagonal1);

    println!("\ninput diagonal2"); 
    let mut diagonal2 = String::new();
    io::stdin().read_line(&mut diagonal2).expect("invalid input");
    let diagonal2:f32 = diagonal2.trim().parse().expect("wrong input");
    println!("Your diagonal2 is {}", diagonal2);

    let _areab = 0.5 * diagonal1 * diagonal2;

    println!("\nArea of rhombus is {}", _areab);

    }
    else if input1 == "c" {
    
    println!("\ninput base");
    let mut base = String::new();
    io::stdin().read_line(&mut base).expect("invalid input");
    let base:f32 = base.trim().parse().expect("wrong input");
    println!("Your base is {}", base);
    
    println!("\ninput altitude");
    let mut altitude = String::new();
    io::stdin().read_line(&mut altitude).expect("invalid input");
    let altitude:f32 = altitude.trim().parse().expect("wrong input");
    println!("Your altitude is {}", altitude);

    let _areac = base * altitude;

    println!("\nArea of Parallelogram is {}", _areac);
    }
     
    else if input1 == "d" {
    
    println!("\ninput length");
    let mut length = String::new();
    io::stdin().read_line(&mut length).expect("invalid input");
    let length:f32 = length.trim().parse().expect("wrong input");
    println!("You length is {}", length);
    
    let _aread = 6.0 * length * length;
    
    println!("\nArea of Cube is {}", _aread);
    }

    else if input1 == "e" {

    println!("\ninput radius");
    let mut radius = String::new();
    io::stdin().read_line(&mut radius).expect("invalid input");
    let radius:f32 = radius.trim().parse().expect("wrong input");
    println!("You radius is {}", radius);

    println!("\ninput height");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("invalid input");
    let height:f32 = height.trim().parse().expect("wrong input");
    println!("Your height is {}", height);

    let _volume = (22.0/7.0) * radius * radius;
    
    println!("\nVolume of Cylinder is {}", _volume);
    }
}