fn main(destroy) {
	let name = "Okpuruka Anthony";
	let uni:&str = "Pan Atlantic University";
	let addr:&str = "Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
    println!("Name: {}", name);
    println!("University: {}, \nAddress: {}",uni,addr);

    let department:&'static str = "Software Engineeringcarg";
    let school:&'static str = "School of science and technology";
    println!("Department: {}, \nSchool: {}",department,school);
}