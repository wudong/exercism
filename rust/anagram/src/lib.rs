use std::{collections::HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut sorted_crs: Vec<char> = word.chars().flat_map(|x|x.to_lowercase()).collect();    
    sorted_crs.sort();
    let result = possible_anagrams.iter()
        .map(|&str| (str, str.chars().flat_map(|x|x.to_lowercase()).collect::<Vec<char>>()))
        .filter_map(|(ss, chars)| 
            if ss.to_lowercase()!=word.to_lowercase() 
               && match_chars(chars, &sorted_crs) {
                Some(ss)
            } else {
                None
            } 
        )
        .collect::<HashSet<&str>>();
    result
}

pub fn match_chars(v1: Vec<char>, v2: &Vec<char>)-> bool {
    let mut v3 = v1; // make a move // Can it be done without this?
    v3.sort();
    v3.len()==v2.len() && v3.eq(v2)
}


#[cfg(test)]
mod  tests {
    use crate::match_chars;

    #[test]
    fn test_match_chars() {
        let v1 = vec!['1', '2', '3', '4', '5'];  // sorted
        let v2 = vec!['5', '3', '2', '4', '1']; 
        assert_eq!(match_chars(v2, &v1), true);
        
        let v2 = vec!['5', '3', '2', '4']; 
        assert_eq!(match_chars(v2, &v1), false);
    }
}
