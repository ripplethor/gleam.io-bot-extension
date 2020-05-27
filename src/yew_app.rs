use yew::prelude::*;
use std::rc::Rc;
use web_sys::*;
use std::sync::{Arc, Mutex};
use crate::checkbox::*;
use crate::bot_logic::run;
use wasm_bindgen_futures::*;

pub enum Tab {
    Main,
    Settings,
    Stats,
}

pub struct Model {
    link: Rc<ComponentLink<Self>>,
    infos: Arc<Mutex<(String, String)>>,
    storage: Storage,
    tab: Tab,
    progress: usize,
    progress_state: BotState,
}

pub enum Msg {
    Done,
    ProgressChange(usize),
    EmailUpdate(String),
    NameUpdate(String),
    ChangeTab(Tab),
    AddToStats(usize),
    Launch,
}

#[derive(PartialEq)]
pub enum BotState {
    Waiting,
    Running,
    Ended
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

        let infos = Arc::new(Mutex::new((email, storage.get("gleam_bot_name").ok().flatten().unwrap_or_else(|| String::from("Undefined Random")))));

        Self {
            link,
            infos,
            storage,
            tab: Tab::Main,
            progress: 0,
            progress_state: BotState::Waiting,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Done => self.progress_state = BotState::Ended,
            Msg::EmailUpdate(email) => {
                self.storage.set("gleam_bot_email", &email).unwrap();

                let mut guard = match self.infos.lock() {
                    Ok(guard) => guard,
                    Err(poisoned) => poisoned.into_inner(),
                };
                guard.0 = email;
            }
            Msg::NameUpdate(name) => {
                self.storage.set("gleam_bot_name", &name).unwrap();

                let mut guard = match self.infos.lock() {
                    Ok(guard) => guard,
                    Err(poisoned) => poisoned.into_inner(),
                };
                guard.1 = name;
            }
            Msg::ChangeTab(tab) => {
                self.tab = tab;
            }
            Msg::ProgressChange(p) => {
                self.progress = p;
            },
            Msg::Launch => {
                if self.progress_state == BotState::Waiting {
                    let link2 = Rc::clone(&self.link);
                    let infos2 = Arc::clone(&self.infos);
                    self.progress = 0;
                    self.progress_state = BotState::Running;
                    spawn_local(async move {
                        run(link2, infos2).await;
                    })
                }
            },
            Msg::AddToStats(new_entries) => {
                let mut total_entries = match self.storage.get("stats_total_entries").map(|t| t.map(|t| t.parse::<usize>())) {
                    Ok(Some(Ok(total_entries))) => total_entries,
                    _ => {
                        elog!("Failed to read stats");
                        0
                    },
                };
                total_entries += new_entries;
                self.storage.set("stats_total_entries", &total_entries.to_string()).unwrap_or_else(|e| elog!("Failed to store stats: {:?}", e));
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
        let name = match guard.1.as_str() {
            "Undefined Random" => "",
            name => name
        };

        match self.tab {
            Tab::Main => {
                html! {
                    <div>
                        <h2>{"Top Secret Control Panel"}</h2>
                        <p>
                            {"Thank you for using the bot!"}
                        </p>
                        <br/>
                        <div class=if self.progress_state != BotState::Waiting {"progress_bar in_progress"}else{"progress_bar"} >
                            <div style=format!("width: {}%", self.progress)>
                            </div>
                        </div>
                        <br/>
                        {
                            match self.progress_state {
                                BotState::Waiting => html! { <button class="btn btn-primary ng-binding" onclick=self.link.callback(|e: _| Msg::Launch)>{"Launch"}</button> },
                                BotState::Running => html! {
                                    { "The bot is running." }
                                },
                                BotState::Ended => html! {
                                    { "The bot has finished." }
                                },
                            }
                        }<br/><br/>
                        <button class="btn btn-primary ng-binding" onclick=self.link.callback(|e: _| Msg::ChangeTab(Tab::Settings))>{"Settings"}</button><br/><br/>
                        <button class="btn btn-primary ng-binding" onclick=self.link.callback(|e: _| Msg::ChangeTab(Tab::Stats))>{"Stats"}</button>
                    </div>
                }
            }
            Tab::Settings => {
                html! {
                    <div>
                        <label>
                            {"Your email: "}
                            <input type="text" class="ng-pristine ng-untouched ng-valid ng-not-empty ng-valid-required ng-valid-pattern" placeholder="alice.smith@example.com" oninput=self.link.callback(|e: InputData| Msg::EmailUpdate(e.value)) value=email/>
                        </label><br/>

                        <label>
                            {"Your name: "}
                            <input type="text" class="ng-pristine ng-untouched ng-valid ng-not-empty ng-valid-required ng-valid-pattern" placeholder="Alice Smith" oninput=self.link.callback(|e: InputData| Msg::NameUpdate(e.value)) value=name/>
                        </label><br/>
                    
                        {"INFO: These options are a preview of the next update. For now it is not working at all."}<br/>
                        <br/>
                        <Checkbox<CheckboxId> id=CheckboxId::Twitter label="Follow on Twitch"/>
                        <Checkbox<CheckboxId> id=CheckboxId::Twitter label="Tweet"/>
                        <Checkbox<CheckboxId> id=CheckboxId::Twitter label="Retweet"/>
                        <Checkbox<CheckboxId> id=CheckboxId::Twitter label="Follow on twitter"/>

                        <button class="btn btn-primary ng-binding" onclick=self.link.callback(|e: _| Msg::ChangeTab(Tab::Main))>{"Save"}</button>
                    </div>
                }
            },
            Tab::Stats => {
                let total_entries = match self.storage.get("stats_total_entries").map(|t| t.map(|t| t.parse::<usize>())) {
                    Ok(Some(Ok(total_entries))) => total_entries,
                    _ => {
                        elog!("Failed to read stats");
                        0
                    },
                };

                html! {
                    <div>
                        {format!("Total entries: {}", total_entries)}<br/>
                        {"More stats will be available in the future."}<br/>
                        <br/>
                        <button class="btn btn-primary ng-binding" onclick=self.link.callback(|e: _| Msg::ChangeTab(Tab::Main))>{"Go back"}</button>
                    </div>
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}