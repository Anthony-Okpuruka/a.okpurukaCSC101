use std::io;

fn main() {
    
    println!("\nPlease Enter the number of siblings you have:");
    let mut number_of_siblings = String::new();
    io::stdin().read_line(&mut number_of_siblings).expect("Failed to read input");
    let siblings:u32 = number_of_siblings.trim().parse().expect("invalid input");

    for i in 0..siblings {
        println!("\nEnter details for siblings {}:",i + 1);
    
        println!("\nName:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read input");
        let name = name.trim();

        println!("\nAge:");
        let mut age_input = String::new();
        io::stdin().read_line(&mut age_input).expect("Failed to read input");
        let age: u32 = age_input.trim().parse().expect("Invalid input, age must be a number");

        println!("\nGender:");
        let mut gender = String::new();
        io::stdin().read_line(&mut gender).expect("Failed to read input");
        let gender = gender.trim(); 
    
        println!("\nCountry of residence:");
        let mut country = String::new();
        io::stdin().read_line(&mut country).expect("Failed to read input");
        let country = country.trim();
    
        println!("\nSibling {}: Name: {}, Age: {}, Gender: {}, Country: {}",i + 1,
        name, age, gender, country);
        
        if age >= 18 {
            println!("\nWhat is your relationship status?");
            println!("
                  a = married
                  b = single
                  c = in a relationship");

            let  mut input1 = String::new();
            io::stdin().read_line(&mut input1).expect("invalid input");
            let input1 = input1.trim();
            println!("\nYour are {}", input1);
    
            if input1 == "a" {
                println!("\n Do you have any children? (yes/no)");
                let mut has_children = String::new();
                io::stdin().read_line(&mut has_children).expect("failed to read input");
            
                if has_children.trim().to_lowercase() == "yes" {
                    println!("\nHow many children do you have?");
                    let  mut number_of_children = String::new();
                    io::stdin().read_line(&mut number_of_children).expect("Failed to read input");
                    let number_of_children:u32 = number_of_children.trim().parse().expect("invalid input");
                    for j in 0..number_of_children {
                        
                        println!("\nEnter details for child {}:", j + 1);

                        println!("\nChild's name:");
                        let mut child_name = String::new();
                        io::stdin().read_line(&mut child_name).expect("Failed to read input");
                        let child_name = child_name.trim();

                        println!("\nChild's age:");
                        let mut child_age = String::new();
                        io::stdin().read_line(&mut child_age).expect("Failed to read input");
                        let child_age:u32 = child_age.trim().parse().expect("invalid input");

                        println!("\nChild's School or daycare name:");
                        let mut school_name = String::new();
                        io::stdin().read_line(&mut school_name).expect("Failed to read input");
                        let school_name = school_name.trim();

                        println!("\nCity where your family currently lives:");
                        let mut city = String::new();
                        io::stdin().read_line(&mut city).expect("Failed to read input");
                        let city = city.trim();

                        println!("\nchild {}: name: {}, age: {}, school or daycare name: {}, city: {}",
                        j + 1, child_name, child_age, school_name, city);
                    }
                }
                else if has_children.trim().to_lowercase() == "no" {
                    println!("Thanks for sharing");
                }  
            }else if input1 == "b" {
                println!("\nAre you a student or employed? (student/employed)");
                let mut occupational_status = String::new();
                io::stdin().read_line(&mut occupational_status).expect("failed to read input");
                let occupational_status = occupational_status.trim();
                
                if occupational_status.trim().to_lowercase() == "student" {

                    println!("\nName of univerity:");
                        let mut university_name = String::new();
                        io::stdin().read_line(&mut university_name).expect("Failed to read input");
                        let _university_name = university_name.trim();

                    println!("\nCourse of study:");
                        let mut course = String::new();
                        io::stdin().read_line(&mut course).expect("Failed to read input");
                        let _course = course.trim();
                    
                    println!("\nYear of study:");
                        let mut year_input = String::new();
                        io::stdin().read_line(&mut year_input).expect("Failed to read input");
                        let _year: u32 = year_input.trim().parse().expect("invalid input");

                    println!("\nAre you studying in your home country or abroad? (home country/abroad)");
                    let mut place_of_study = String::new();
                    io::stdin().read_line(&mut place_of_study).expect("failed to read input");
                    let place_of_study = place_of_study.trim();

                    if place_of_study.trim().to_lowercase() == "abroad" {
                        println!("\nWhat country are you studying in?");
                        let mut country = String::new();
                        io::stdin().read_line(&mut country).expect("Failed to read input");
                        let _country = country.trim();
                        println!("\nYou are studying in {}",_country);
                        println!("Thanks for sharing");
                    }
                    
                    else if place_of_study.trim().to_lowercase() == "home country" {
                        println!("\nYou are studying in your home country");
                    }
                else if occupational_status.trim().to_lowercase() == "employed" {
                    println!("\nIs your job remote, on-site, or hybrid? (remote/on-site/hybrid)");
                }
                    let mut job_status = String::new();
                    io::stdin().read_line(&mut job_status).expect("failed to read input");
                    let job_status = job_status.trim();
                    
                    if job_status == "remote" {
                    println!("\nYour job is remote");
                    }
                    else if job_status == "on-site" {
                        let mut company_name = String::new();
                        io::stdin().read_line(&mut company_name).expect("Failed to read input");
                        let company = company_name.trim();

                        let mut job_title = String::new();
                        io::stdin().read_line(&mut job_title).expect("Failed to read input");
                        let job_title = job_title.trim();

                        let mut industry_sector = String::new();
                        io::stdin().read_line(&mut industry_sector).expect("Failed to read input");
                        let industry_sector = industry_sector.trim();
                        println!("\nYour company name is: {}, job title is: {}, industry sector is {}",company, job_title, industry_sector);
                    }
                    else if job_status == "hybrid" {
                    println!("\nYour job is hybrid");
                    }                
                }
            }else if input1 == "c" {
                println!("\nHow long have you been in a relationship?");

                let mut duration_input = String::new();
                io::stdin().read_line(&mut duration_input).expect("Failed to read input");
                let duration: u32 = duration_input.trim().parse().expect("invalid input");

                let mut first_name = String::new();
                io::stdin().read_line(&mut first_name).expect("Failed to read input");
                let firstname = first_name.trim();
                
                println!("\n Your partner's first name is {} and you have been together for {} years", firstname, duration);

                println!("\nDo you live with your partner? (yes/no)");
                let mut cohabitation = String::new();
                io::stdin().read_line(&mut cohabitation).expect("failed to read input");
                let cohabitation_status = cohabitation.trim();

                if cohabitation_status == "yes" {
                    println!("\nWhat city do you reside");
                    
                    let mut input2 = String::new();
                    io::stdin().read_line(&mut input2).expect("failed to read input");
                    let input2 = input2.trim();

                    println!("\nYou reside in {}", input2);
                }
                else if cohabitation_status == "no" {
                println!("\nThanks for sharing");
                }
            }    
        }else if age < 18 {
            println!("Has he/she completed WAEC exams? (yes/no)");
            
            let mut exam = String::new();
            io::stdin().read_line(&mut exam).expect("failed to read input");
            let exam_status = exam.trim();

            if exam_status == "yes" {
            println!("\n What is the name of the secondary school you attended?");
            let mut secondary_school = String::new();
            io::stdin().read_line(&mut secondary_school).expect("failed to read input");
            let secondary_school = secondary_school.trim();
            
            println!("What is your final grade?");
            let mut final_grade = String::new();
            io::stdin().read_line(&mut final_grade).expect("failed to read input");
            let final_grade = final_grade.trim();
             
            println!("What year did you graduate?");
            let mut graduation_year = String::new();
            io::stdin().read_line(&mut graduation_year).expect("failed to read input");
            let graduation_year: u32 = graduation_year.trim().parse().expect("invalid input");
            println!("\nYou attended {}, your final grade was {} and you graduated from school in {}",secondary_school, final_grade, graduation_year );
            }
            else if exam_status == "no" {
                println!("\n What class are you currently in?");
                
                let mut class = String::new();
                io::stdin().read_line(&mut class).expect("failed to read input");
                let class = class.trim();
                println!("\nYour currently in {}", class);

                println!("Do you plan on writing waec soon? (yes/no)");
                let mut waec = String::new();
                io::stdin().read_line(&mut waec).expect("failed to read input");
                let waec = waec.trim();

                if waec == "yes" {
                    println!("What year do you plan on writing waec?");
                    
                    let mut waec_year = String::new();
                    io::stdin().read_line(&mut waec_year).expect("failed to read input");
                    let waec_year: u32 = waec_year.trim().parse().expect("invalid input");
                    println!("Your going to write WAEC in {}, make sure you prepare well", waec_year);
                }
                else if waec == "no" {
                println!("Keep studying to get good grades");
                }
            }
        }
    }
}
    
    
    