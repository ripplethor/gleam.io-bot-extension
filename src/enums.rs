use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub enum Platform {
    Twitter,
    Youtube,
    Twitch,
    Custom,
    Instagram,
    Facebook,
    Share,
    Mixer,
    Media,
    Loyalty,
    Email,
    Discord,
    Steam,
    Submit,
}

impl TryFrom<&str> for Platform {
    type Error = String;

    fn try_from(data: &str) -> Result<Self, Self::Error> {
        match data {
            "youtube" => Ok(Platform::Youtube),
            "twitchtv" => Ok(Platform::Twitch),
            "twitter" => Ok(Platform::Twitter),
            "custom" => Ok(Platform::Custom),
            "instagram" => Ok(Platform::Instagram),
            "facebook" => Ok(Platform::Facebook),
            "share" => Ok(Platform::Share),
            "mixer" => Ok(Platform::Mixer),
            "media" => Ok(Platform::Media),
            "loyalty" => Ok(Platform::Loyalty),
            "email" => Ok(Platform::Email),
            "discord" => Ok(Platform::Discord),
            "steam" => Ok(Platform::Steam),
            "submit" => Ok(Platform::Submit),
            unknow => Err(format!("unknow platform: {}", unknow)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ActionType {
    Enter,
    Follow,
    Retweet,
    Tweet,
    VisitChannel,
    Video,
    Action,
    VisitProfile,
    Visit,
    Loyalty,
    Subscribe,
    Choose,
    ViewPost,
    Hashtags,
    Url,
}

impl TryFrom<&str> for ActionType {
    type Error = String;

    fn try_from(data: &str) -> Result<Self, Self::Error> {
        match data {
            "enter" => Ok(ActionType::Enter),
            "follow" => Ok(ActionType::Follow),
            "retweet" => Ok(ActionType::Retweet),
            "tweet" => Ok(ActionType::Tweet),
            "visit_channel" => Ok(ActionType::VisitChannel),
            "video" => Ok(ActionType::Video),
            "action" => Ok(ActionType::Action),
            "visit_profile" => Ok(ActionType::VisitProfile),
            "visit" => Ok(ActionType::Visit),
            "loyalty" => Ok(ActionType::Loyalty),
            "subscribe" => Ok(ActionType::Subscribe),
            "choose" => Ok(ActionType::Choose),
            "url" => Ok(ActionType::Url),
            "view_post" => Ok(ActionType::ViewPost),
            "hashtags" => Ok(ActionType::Hashtags),
            unknow => Err(format!("unknow action type: {}", unknow)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct EntryType {
    pub platform: Platform,
    pub action_required: ActionType,
}

impl TryFrom<String> for EntryType {
    type Error = String;

    fn try_from(mut data: String) -> Result<Self, Self::Error> {
        if data.starts_with("###APP_NAME### ") {
            for _ in 0..15 {
                data.remove(0);
            }
        }
        let triple_data: Vec<&str> = data.split('|').collect();

        Ok(EntryType {
            platform: Platform::try_from(triple_data[1])?,
            action_required: ActionType::try_from(triple_data[2])?,
        })
    }
}
