pub fn is_valid(s: String) -> bool {
    let mut stack = "".to_string();

    for ch in s.chars() {
        match ch {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            ')' | ']' | '}' => {
                if let Some(m) = stack.pop() {
                    if m != ch {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => panic!("can't handle char '{ch}'"),
        };
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        assert_eq!(is_valid("()".to_string()), true);
    }

    #[test]
    fn ex2_test() {
        assert_eq!(is_valid("()[]{}".to_string()), true);
    }

    #[test]
    fn ex3_test() {
        assert_eq!(is_valid("(]".to_string()), false);
    }

    #[test]
    fn ex4_test() {
        assert_eq!(is_valid("".to_string()), true);
    }
}
