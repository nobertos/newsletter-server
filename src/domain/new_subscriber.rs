use super::subscriber_email::SubscriberEmail;
use super::subscriber_name::SubscriberName;
use super::Parser;

#[derive(Debug)]
pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}

impl NewSubscriber {
    pub fn new(email: String, name: String) -> Result<Self, String> {
        Ok(NewSubscriber {
            email: SubscriberEmail::parse(email)?,
            name: SubscriberName::parse(name)?,
        })
    }
}
