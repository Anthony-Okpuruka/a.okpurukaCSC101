use std::io::Write;
fn main() {
    let exp = vec!["33 Export,"," Desperados,"," Goldberg,"," Gulder,"," Heineken,"," & Star."];
    let leg = vec!["Legend,"," Turbo King,"," & Williams."];
    let mal = vec!["Maltina,"," Amstel Malta,"," Malta Gold,"," & Fayrouz."];
    let header_1 = "Something for heading;\n";
    let header_2 = " Something for heading;\n";
    let header_3 = "Below is our rich portfolio of Non-Alcoholic drinks;\n";


    let mut file = std::fs::File::create("").expect("");
    file.write_all("Welcome to Nigerian Brewery Limited\n\n".as_bytes()).expect("");
    file.write_all(header_1.as_bytes()).expect("");
    for x in exp.iter() { file.write_all(x.as_bytes());}
    file.write_all(b"\n\n");
    file.write_all(header_2.as_bytes()).expect("write failed");
    for y in leg.iter() { file.write_all(y.as_bytes());}
        file.write_all(b"\n\n");
    file.write_all(header_3.as_bytes()).expect("write failed");
    for z in mal.iter() {file.write_all(z.as_bytes());}
        file.write_all(b"\n");
    println!("\nDrinks log successful!");

}