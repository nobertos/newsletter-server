use unicode_segmentation::UnicodeSegmentation;
pub trait Parser {
    type Item;
    type Error;
    fn parse(value: Self::Item) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

impl NewSubscriber {
    pub fn new(email: String, name: String) -> Self {
        NewSubscriber {
            email,
            name: SubscriberName::parse(name).expect("Failed to validate the subscriber name"),
        }
    }
}

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
        let is_empty = value.is_empty();
        let is_too_long = value.graphemes(true).count() > 256;
        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_fc = value.contains(forbidden_chars);

        if is_empty || is_too_long || contains_fc {
            return Err(format!("{} is not a valid subscriber name", value));
        }
        Ok(Self(value))
    }
}
