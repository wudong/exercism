
pub fn abbreviate(phrase: &str) -> String {
    let cs = phrase.split(&[' ','-']).into_iter()    
    .map(|x| de_all_cap(x))    
    .flat_map(|s|
            s.chars().enumerate()
                 .filter(|(idx, c)| c.is_ascii_uppercase() || (*idx==0 && *c!='_') )
        ).map(|s|s.1.to_ascii_uppercase());
        
    String::from_iter(cs)
}


pub fn de_all_cap(ss: &str) -> &str {
    if is_all_cap(ss) {  // only care about the first letter.
       &ss[0..1]
    }else {
        ss
    }
}

pub fn is_all_cap(ss: &str) -> bool {
    ss.len() > 0 && ss.chars().all(|c|c.is_ascii_uppercase() || c.is_ascii_punctuation())
}

#[test]
fn test_de_all_cap() {
    assert_eq!("", de_all_cap(""))
}