use yew::prelude::*;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::*;
use crate::start;
use std::sync::{Arc, Mutex};
use crate::checkbox::Checkbox;

#[derive(PartialEq)]
pub enum ProcessState {
    Waiting,
    Running,
    Done
}

pub struct Model {
    link: Rc<ComponentLink<Self>>,
    infos: Arc<Mutex<(String, String)>>,
    storage: Storage,
}

pub enum Msg {
    Done,
    EmailUpdate(String),
}

#[derive(Clone)]
pub enum CheckboxId {
    Twitter
}

impl Default for CheckboxId {
    fn default() -> CheckboxId {
        CheckboxId::Twitter
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let link = Rc::new(link);
        let window = window().unwrap();
        let storage = window.local_storage().unwrap().unwrap();
        
        let email = if let Ok(Some(email)) = storage.get("gleam_bot_email") {
            email
        } else {
            String::from("unknown@email.com")
        };

        let infos = Arc::new(Mutex::new((email, String::from("Unknown Name"))));

        Self {
            link,
            infos,
            storage
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Done => (),
            Msg::EmailUpdate(email) => {
                self.storage.set("gleam_bot_email", &email).unwrap();

                let mut guard = match self.infos.lock() {
                    Ok(guard) => guard,
                    Err(poisoned) => poisoned.into_inner(),
                };
                guard.0 = email;
            }
        }
        true
    }

    fn view(&self) -> Html {
        let guard = match self.infos.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let email = match guard.0.as_str() {
            "unknown@email.com" => "",
            email => email
        };

        html! {
            <div>
                <input type="text" class="ng-pristine ng-untouched ng-valid ng-not-empty ng-valid-required ng-valid-pattern" placeholder="alice.smith@example.com" oninput=self.link.callback(|e: InputData| Msg::EmailUpdate(e.value)) value=email/><br/>

                <Checkbox<CheckboxId> id=CheckboxId::Twitter label="Follow on Twitch"/>
                <Checkbox<CheckboxId> id=CheckboxId::Twitter label="Tweet"/>
                <Checkbox<CheckboxId> id=CheckboxId::Twitter label="Retweet"/>
                <Checkbox<CheckboxId> id=CheckboxId::Twitter label="Follow on twitter"/>
                {"Other options are not controllable for now"}
            </div>
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}