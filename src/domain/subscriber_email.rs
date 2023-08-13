use super::Parser;

use validator::validate_email;

#[derive(Debug, PartialEq)]
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

impl std::fmt::Display for SubscriberEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
    use claim::assert_err;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    use quickcheck::Arbitrary;
    use quickcheck::Gen;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(String);

    impl Arbitrary for ValidEmailFixture {
        fn arbitrary(g: &mut Gen) -> Self {
            let mut rng = StdRng::seed_from_u64(u64::arbitrary(g));
            let email = SafeEmail().fake_with_rng(&mut rng);
            Self(email)
        }
    }

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
    fn nassim_esi_email_success() {
        let email = "jr_zorgani@esi.dz".to_string();
        assert_eq!(
            Ok(SubscriberEmail("jr_zorgani@esi.dz".to_string())),
            SubscriberEmail::parse(email)
        );
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        SubscriberEmail::parse(valid_email.0).is_ok()
    }
}
