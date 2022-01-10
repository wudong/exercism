use std::fmt::Write;

pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
       0=> "".to_string(),       
       _=>build_proverb_verified(list),
    }    
}

pub fn build_proverb_verified(list: &[&str]) -> String {
    let vec = zip_array(list);
    let mut str = String::new();
    vec.for_each(|f|{
        writeln!(str, "For want of a {} the {} was lost.", f.0, f.1).unwrap();
    });
    write!(str, "And all for the want of a {}.", list[0]).unwrap();
    str
}

// is the lifetime annotation used correctly here?
// I feel the list's life cycle would be different from the &str it contains?
pub fn zip_array<'a> (list: &'a[&str]) -> impl Iterator<Item=(&'a str, &'a str)>{    
    list.iter().zip(list[1..].iter()).map(|(&x, &y)|(x,y))    
}
