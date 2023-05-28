use super::Parser;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Parser for SubscriberName {
    type Item = String;
    type Error = String;

    fn parse(value: Self::Item) -> Result<Self, Self::Error> {
        let is_empty = value.trim().is_empty();
        let is_too_long = value.graphemes(true).count() > 256;
        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_fc = value.contains(forbidden_chars);

        if is_empty || is_too_long || contains_fc {
            return Err(format!("{} is not a valid subscriber name", value));
        }
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use super::SubscriberName;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_valid() {
        let name = "a".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn name_longer_than_256_graphemes_rejected() {
        let name = "a".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn whitespace_only_names_rejected() {
        let name = " ".into();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn empty_string_rejected() {
        let name = "".into();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn names_containing_invalid_char_rejected() {
        for name in [
            "na/ss", "na(ss", "na)ss", "na\"ss", "na<ss", "na>ss", "na\\ss", "na{ss", "na}ss",
        ] {
            let name = name.into();
            assert_err!(SubscriberName::parse(name));
        }
    }

    #[test]
    fn valid_name_parsed_successfully() {
        let name = "rayene nassim".into();
        assert_ok!(SubscriberName::parse(name));
    }
}
