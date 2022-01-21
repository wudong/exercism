use std::{collections::VecDeque};

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut queue = VecDeque::<char>::new();

    let mut success = true;
    string.chars().for_each(|f|{
        if f=='('||f=='[' || f=='{'{
            queue.push_back(f);
        } else  if f==')'||f==']' || f=='}'{
            let c=queue.pop_back();
            let match_successful = match c {
                Some(cc)=> {
                     match_pair(cc,f)                                             
                },
                None => {
                    false
                }
            };            
            if !match_successful {
                success = false;
            }
        }
    });

    return success && queue.len()==0 
}

fn match_pair(c1: char, c2: char) -> bool {
    return c1=='(' && c2 == ')'
        ||c1=='[' && c2 == ']'
        ||c1=='{' && c2 == '}'
}
