use yew::prelude::*;

pub enum Message<T: std::fmt::Display> {
    Warning(T),
    Tip(T),
    Stat(T),
    Info(T),
    Other(T),
    Danger(T),
    Error(T),
}

impl<T: std::fmt::Display> Message<T> {
    pub fn as_html(&self) -> Html {
        match self {
            Message::Warning(message) => html! {
                <div><div class="warn_message"><b>{"Warning: "}</b>{message}</div><br/></div>
            },
            Message::Tip(message) => html! {
                <div><div class="tip_message"><b>{"Tip: "}</b>{message}</div><br/></div>
            },
            Message::Stat(message) => html! {
                <div><div class="stat_message">{message}</div><br/></div>
            },
            Message::Info(message) => html! {
                <div><div class="info_message"><b>{"Did you know? "}</b>{message}</div><br/></div>
            },
            Message::Other(message) => html! {
                <div><div class="unknown_message">{message}</div><br/></div>
            },
            Message::Danger(message) => html! {
                <div><div class="danger_message"><b>{"DANGER: "}</b>{message}</div><br/></div>
            },
            Message::Error(message) => html! {
                <div><div class="danger_message"><b>{"ERROR: "}</b>{message}</div><br/></div>
            },
        }
    }
}

impl<U: std::fmt::Debug> std::convert::From<U> for Message<String> {
    fn from(error: U) -> Self {
        Message::Error(format!("{:?}", error))
    }
}