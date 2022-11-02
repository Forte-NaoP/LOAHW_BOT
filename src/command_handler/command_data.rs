use serenity::{
    model::{
        prelude::{
            interaction::application_command::{CommandDataOption, CommandDataOptionValue},
        },
        user::User,
        channel::{PartialChannel, Attachment},
        guild::Role,
    },
};

pub struct DataWrapper {
    pub data: CommandDataOptionValue
}

impl DataWrapper {
    pub fn from(options: &[CommandDataOption], idx: usize) -> DataWrapper {
        let option = options
            .get(idx)
            .expect("")
            .resolved
            .as_ref()
            .expect("");
        DataWrapper { data: option.clone() }
    }
}

impl From<DataWrapper> for Option<String> {
    fn from(item: DataWrapper) -> Option<String> {
        match item.data {
            CommandDataOptionValue::String(data) => Some(data),
            _ => None,
        }
    }
}

impl From<DataWrapper> for Option<i64> {
    fn from(item: DataWrapper) -> Option<i64> {
        match item.data {
            CommandDataOptionValue::Integer(data) => Some(data),
            _ => None,
        }
    }
}

impl From<DataWrapper> for Option<bool> {
    fn from(item: DataWrapper) -> Option<bool> {
        match item.data {
            CommandDataOptionValue::Boolean(data) => Some(data),
            _ => None,
        }
    }
}

impl From<DataWrapper> for Option<User> {
    fn from(item: DataWrapper) -> Option<User> {
        match item.data {
            CommandDataOptionValue::User(data, _) => Some(data),
            _ => None,
        }
    }
}

impl From<DataWrapper> for Option<PartialChannel> {
    fn from(item: DataWrapper) -> Option<PartialChannel> {
        match item.data {
            CommandDataOptionValue::Channel(data) => Some(data),
            _ => None,
        }
    }
}

impl From<DataWrapper> for Option<Role> {
    fn from(item: DataWrapper) -> Option<Role> {
        match item.data {
            CommandDataOptionValue::Role(data) => Some(data),
            _ => None,
        }
    }
}

impl From<DataWrapper> for Option<f64> {
    fn from(item: DataWrapper) -> Option<f64> {
        match item.data {
            CommandDataOptionValue::Number(data) => Some(data),
            _ => None,
        }
    }
}

impl From<DataWrapper> for Option<Attachment> {
    fn from(item: DataWrapper) -> Option<Attachment> {
        match item.data {
            CommandDataOptionValue::Attachment(data) => Some(data),
            _ => None,
        }
    }
}