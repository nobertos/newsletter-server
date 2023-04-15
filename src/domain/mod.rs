pub mod new_subscriber;
pub mod subscriber_email;
pub mod subscriber_name;

pub trait Parser {
    type Item;
    type Error;
    fn parse(value: Self::Item) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
