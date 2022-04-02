#[cfg(test)]
mod tests {
    use crate::trans::Locale;

    #[test]
    fn str_to_enum() {
        let string = "en_GB";
    
        let locale = Locale::from_string(string);
    
        assert_eq!(locale, Locale::English);
    }

    #[test]
    fn enum_to_str() {
        let locale = Locale::Slovak;

        let string = locale.to_string();

        assert_eq!(string, "sk");
    }
}