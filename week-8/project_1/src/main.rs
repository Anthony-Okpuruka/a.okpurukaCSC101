fn main() {
    let aps_levels = vec![
        vec!["APS 1-2", "Intern", "Paralegal", "Placement"],
        vec!["APS 3-5", "Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"],
        vec!["APS 5-8", "Senior Administrator", "PhD Candidate", "Associate", "Snr Teacher"],
        vec!["EL 1 8-10", "Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"],
        vec!["EL2 10-13", "Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"],
        vec!["SES", "CEO", "Dean", "Partner", "Principal"],
    ];

    println!("Enter your job title:");
    let mut title = String::new();
    std::io::stdin().read_line(&mut title).expect("wrong input");
    title = title.trim().to_lowercase();

    println!("Enter the number of years of experience:");
    let mut experience = String::new();
    std::io::stdin().read_line(&mut experience).expect("wrong input");
    let experience: u32 = experience.trim().parse().expect("inalid input");

    let mut aps_level = "Unknown APS level";
    for i in aps_levels.iter() {
       
        for job in &i[1..] {
            
            if job.to_lowercase() == title {
                aps_level = i[0]; // Set the APS level
                break; // Exit the inner loop when a match is found
            }
        }
    }

    println!("The APS level for {} with {} years of experience is {}", title, experience, aps_level);
}