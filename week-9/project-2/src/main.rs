use std::fs::File;
use std::io::Write;
use std::io::Result;
fn fileInput() -> String { let mut output = String::new();
    // Define the table headers
    let headers = vec!["Student Name", "Matric. Number", "Department","Level"];
 // Define the table rows
    let rows = vec![vec!["Oluchi Mordi", "ACC10211111", "Accounting","300"],
        vec!["Adams Aliyu", "ECO10110101", "Economics","100"],
        vec!["Shania Bolade", "CSC10328828", "Computer","200"],
        vec!["Adekunle Gold", "EEE11020202", "Electrical","200"],
        vec!["Blanca Edemoh", "MEE10202001", "Mechanical","100"],    ];
// For the headers
    for header in &headers {output.push_str(&format!("{:<15}", header));}output.push('\n');
 // For the separator line
    for _ in &headers {output.push_str(&format!("{:-<15}", ""));} output.push_str("\n");
// For the rows
    for row in &rows {for cell in row {output.push_str(&format!("{:<15}", cell));}output.push('\n');}output}
fn main() -> Result<()> {
    println!("\t\tWelcome to PAU-SMIS");
    let content = fileInput();
    println!("{}", content);
    let mut file = File::create("PAU-SMIS_output")?;
    file.write_all(content.as_bytes())?;
    println!("File has been created and the necessary input logged");Ok(())}
