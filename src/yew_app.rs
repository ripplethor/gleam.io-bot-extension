
use yew::prelude::*;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::*;
use crate::start;
use std::sync::{Arc, Mutex};

#[derive(PartialEq)]
pub enum ProcessState {
    Waiting,
    Running,
    Done
}

pub struct Model {
    link: Rc<ComponentLink<Self>>,
    infos: Arc<Mutex<(String, String)>>,
    process_state: ProcessState,
    storage: Storage,
}

pub enum Msg {
    StorageChange,
    Unload,
    Done,
    EmailUpdate(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let link = Rc::new(link);
        let window = window().unwrap();
        let storage = window.local_storage().unwrap().unwrap();

        let link2 = Rc::clone(&link);
        let closure = Closure::wrap(Box::new(move |_event: Event| {
            let link3= Rc::clone(&link2);
            let window = web_sys::window().unwrap();
            let crypto = window.crypto().unwrap();
            let mut random = [0;1];
            crypto.get_random_values_with_u8_array(&mut random).unwrap();
            let closure2 = Closure::wrap(Box::new(move |_event: Event| {
                link3.send_message(Msg::StorageChange);
            }) as Box<dyn FnMut(_)>);
            let random: i32 = 100 + random[0] as i32 * 100;
            window.set_timeout_with_callback_and_timeout_and_arguments_0(closure2.as_ref().unchecked_ref(), random).unwrap();
            closure2.forget();
        }) as Box<dyn FnMut(_)>);
        window.set_onstorage(Some(closure.as_ref().unchecked_ref()));
        closure.forget();

        let link2 = Rc::clone(&link);
        let closure = Closure::wrap(Box::new(move |_event: Event| {
            link2.send_message(Msg::Unload);
        }) as Box<dyn FnMut(_)>);
        window.set_onbeforeunload(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
        
        let email = if let Ok(Some(email)) = storage.get("gleam_bot_email") {
            email
        } else {
            String::from("unknown@email.com")
        };
        let infos = Arc::new(Mutex::new((email, String::from("Unknown Name"))));
        let link2 = Rc::clone(&link);
        let process_state: ProcessState = match storage.get("gleam_bot_lock") {
            Ok(Some(state)) if state == "busy" => ProcessState::Waiting,
            _ => {
                storage.set("gleam_bot_lock", "busy").unwrap();
                start(link2, Arc::clone(&infos));
                ProcessState::Running
            }
        };

        Self {
            link,
            infos,
            process_state,
            storage
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StorageChange => if self.process_state == ProcessState::Waiting {
                self.process_state = match self.storage.get("gleam_bot_lock") {
                    Ok(Some(state)) if state == "busy" => ProcessState::Waiting,
                    _state => {
                        self.storage.set("gleam_bot_lock", "busy").unwrap();
                        let link2 = Rc::clone(&self.link);
                        start(link2, Arc::clone(&self.infos));
                        ProcessState::Running
                    }
                };
            },
            Msg::Unload => if self.process_state == ProcessState::Running {
                self.storage.set("gleam_bot_lock", "ready").unwrap();
            }
            Msg::Done => if self.process_state == ProcessState::Running {
                self.process_state = ProcessState::Done;
                self.storage.set("gleam_bot_lock", "ready").unwrap();
            }
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

        match self.process_state {
            ProcessState::Waiting => html! {
                <div>
                    <input type="text" class="ng-pristine ng-untouched ng-valid ng-not-empty ng-valid-required ng-valid-pattern" placeholder="alice.smith@example.com" oninput=self.link.callback(|e: InputData| Msg::EmailUpdate(e.value)) value=email/><br/>
                    {"Waiting for another process to finish."}
                </div>
            },
            ProcessState::Running => html! {
                <div>
                    <input type="text" class="ng-pristine ng-untouched ng-valid ng-not-empty ng-valid-required ng-valid-pattern" placeholder="alice.smith@example.com" oninput=self.link.callback(|e: InputData| Msg::EmailUpdate(e.value)) value=email/><br/>
                    {"The process is running."}
                </div>
            },
            ProcessState::Done => html! {
                <div>
                    <input type="text" class="ng-pristine ng-untouched ng-valid ng-not-empty ng-valid-required ng-valid-pattern" placeholder="alice.smith@example.com" oninput=self.link.callback(|e: InputData| Msg::EmailUpdate(e.value)) value=email/><br/>
                    {"The process has ended."}
                </div>
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        true
    }
}