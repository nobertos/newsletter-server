use super::Parser;

use validator::validate_email;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl Parser for SubscriberEmail {
    type Item = String;

    type Error = String;

    fn parse(value: Self::Item) -> Result<Self, Self::Error> {
        if validate_email(&value) {
            return Ok(Self(value));
        }
        Err(format!("{} is not a valid subscriber email.", value))
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use super::SubscriberEmail;
    use claim::{assert_err, assert_ok};
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    #[test]
    fn empty_email_rejected() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_at_symbol_rejected() {
        let email = "esi.dz".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_subject_rejected() {
        let email = "@esi.dz".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn valid_emails_pared_successfully() {
        let email = SafeEmail().fake();
        assert_ok!(SubscriberEmail::parse(email));
    }
}
