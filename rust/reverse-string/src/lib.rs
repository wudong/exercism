pub fn reverse(input: &str) -> String {
    let mut stack = std::collections::LinkedList::new();
    let chars = input.chars();
    for c in chars {
        stack.push_back(c)
    }

    let mut result = String::new();
    while !stack.is_empty() {
        let char = stack.pop_back().expect("is empty should be checked");
        result.push(char)
    }

    return result;
}
