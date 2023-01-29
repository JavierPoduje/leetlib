pub fn is_palindrome(s: String) -> bool {
    let forward: String = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();
    let backward: String = forward.chars().rev().collect();
    forward == backward
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        assert_eq!(
            is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
    }

    #[test]
    fn ex2_test() {
        assert_eq!(is_palindrome("race a car".to_string()), false);
    }

    #[test]
    fn ex3_test() {
        assert_eq!(is_palindrome(" ".to_string()), true);
    }
}
