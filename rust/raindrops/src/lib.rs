pub fn raindrops(n: u32) -> String {
    let divisor:[u32; 3] =[3,5,7];
    let str:[&str; 3] =["Pling","Plang","Plong"];
    
    let strs: Vec<&str> = divisor.iter().zip(str)
           .filter_map(|(d, s)| if n % d == 0 {Some(s)} else {None})
           .collect();
    match strs.len() {
        0 => n.to_string(),
        _ => strs.join(""),
    } 
}
