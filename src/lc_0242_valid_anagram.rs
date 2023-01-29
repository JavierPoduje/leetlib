pub fn is_anagram(s: String, t: String) -> bool {
    let mut s = s.chars().collect::<Vec<char>>();
    let mut t = t.chars().collect::<Vec<char>>();
    s.sort();
    t.sort();
    s == t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        assert_eq!(
            is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );
    }

    #[test]
    fn ex2_test() {
        assert_eq!(is_anagram("rat".to_string(), "carg".to_string()), false);
    }
}
