pub mod gtinny {
    use regex::Regex;

    pub fn is_valid(gtin: &str) -> bool {
        let re = Regex::new(r"^\d{8}$|^\d{12,14}$").unwrap();
        let is_match = re.is_match(gtin);
        if !is_match {
            return false;
        }
        let last_char = gtin.chars().last().unwrap();
        let reversed = gtin[0..gtin.len() - 1].chars().rev();

        let mut list: [u32; 13] = [0; 13];
        for (i, el) in reversed.enumerate() {
            let c = if i % 2 == 0 { 3 } else { 1 };
            list[i] = el.to_digit(10).unwrap() * c;
        }
        let sum = list.into_iter().reduce(|a, b| a + b).unwrap() % 10;

        return last_char.to_string() == (10 - (if sum == 0 { 10 } else { sum })).to_string();
    }
}

#[cfg(test)]
mod tests {
    use crate::util::gtinny::is_valid;

    #[test]
    fn valid_gtin() {
        let result = is_valid("97350053850012");
        assert_eq!(result, true);
    }

    #[test]
    fn invalid_gtin() {
        let result = is_valid("12398748906");
        assert_eq!(result, false);
    }
}