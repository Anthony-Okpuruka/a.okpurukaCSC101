struct Laptop {
    brand: String, price: u32,
}

impl Laptop {
    fn total(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    // create instances for each laptop brand
    let hp = Laptop{
        brand: String::from("HP"),
        price: 650_000,
    };
    
    let ibm = Laptop{
        brand: String::from("IBM"),
        price: 755_000, 
    };

    let toshiba = Laptop{
        brand: String::from("Toshiba"),
        price: 550_000,
    };

    let dell = Laptop{
        brand: String::from("Dell"),
        price: 850_000,
    };
    //calculate the total cost for 3 laptops from each brand
    let quantity = 3;

    let total_hp = hp.total(quantity);
    println!("The total cost for {} laptops is {}", hp.brand,total_hp);

    let total_ibm = ibm.total(quantity);
    println!("The total cost for {} laptops is {}", ibm.brand,total_ibm);

    let total_toshiba = toshiba.total(quantity);
    println!("The total cost for {} laptops is {}", toshiba.brand,total_toshiba);

    let total_dell = dell.total(quantity);
    println!("The total cost for {} laptops is {}", dell.brand,total_dell);
    
    let total = total_hp + total_ibm + total_toshiba + total_dell;

    println!("The total cost for 3 laptops for each brand is: {}", total);
}
