fn main() {
    let heading = vec!["Name of Commisioner","      Ministry","          Geopolitical Zone"];
       for era in &heading{print!("{}",era);}
        println!("");
   
    let time = vec!["    Internal Affairs","      Justice"," Defense","     Power & Steel","     Petroleum"];
     let one = time[0];
     let two = time[1];
     let three = time[2];
     let four = time[3];
     let five = time[4];
   
    let zone = vec!["  South West","           North East","           South South","     South West","         South East"];
     let a = zone[0];
     let b = zone[1];
     let c = zone[2];
     let d = zone[3];
     let e = zone[4];
   
    let boxes = vec![vec!["Aigbogun Alamba Dauda"],vec!["Murtala Afeez Bendu"],vec!["Okorocha Calistus Obgona"],vec!["Adewale Jimoh Akandi"],vec!["Osazuwa Faith Etieye"]];
   
     let mut com_1 = boxes[0].clone();
      com_1.push(&one);
      com_1.push(&a);
       for line in &com_1{print!("{}",line);}
        println!("");
   
     let mut com_2 = boxes[1].clone();
      com_2.push(&two);
      com_2.push(&b);
       for row in &com_2{print!("{}",row);}
        println!("");
     let mut com_3 = boxes[2].clone();
      com_3.push(&three);
      com_3.push(&c);
       for record in &com_3{print!("{}",record);}
        println!("");
     let mut com_3 = boxes[3].clone();
      com_3.push(&four);
      com_3.push(&d);
       for entry in &com_3{print!("{}",entry);}
        println!("");
   let mut com_5 = boxes[4].clone();
      com_5.push(&five);
      com_5.push(&e);
       for tuple in &com_5{print!("{}",tuple);}
        println!("");
}
   