pub mod Util {
    pub fn is_letter(ch: char) -> bool {
        match ch {
            'a'..='z' | 'A'..='Z' => true,
            _ => false
        }
    }

    pub fn is_number(ch: char) -> bool {
        match ch {
            '0'..='9' => true,
            _ => false
        }
    }
}
