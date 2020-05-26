use crate::enums::*;
use wasm_bindgen::JsValue;
use web_sys::*;
use crate::util::*;
use std::time::Duration;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use crate::yew_app::{Model, Msg};
use std::convert::TryFrom;
use std::sync::{Arc, Mutex};

pub fn start(link: Rc<ComponentLink<Model>>, infos: Arc<Mutex<(String, String)>>) {
    let link = Rc::clone(&link);
    let _promise = wasm_bindgen_futures::future_to_promise(run(link, infos));
    /*let ok = Closure::wrap(Box::new(move |_event: JsValue|  {
        eval("console.log('ok');");
    }) as Box<dyn FnMut(_)>);
    let err = Closure::wrap(Box::new(move |_event: JsValue|  {
        eval("console.log('err');");
    }) as Box<dyn FnMut(_)>);
    promise.then2(&ok, &err);
    ok.forget();
    err.forget();*/
}

async fn run(link: Rc<ComponentLink<Model>>, infos: Arc<Mutex<(String, String)>>) -> Result<JsValue, JsValue> {
    let window = window().expect("No window");
    let document = window.document().expect("No document");

    // form asking name, email and birthdate
    if let Some(form) = document.get_elements_by_class_name("contestant compact-box form-compact ng-pristine ng-scope ng-valid-pattern ng-invalid ng-invalid-required ng-valid-email").item(0) {

        let inputs = form.get_elements_by_tag_name("input");
        let mut input_elements = Vec::new();
        for idx in 0..inputs.length() {
            input_elements.push(inputs.item(idx).unwrap());
        }
        for input in input_elements {
            log!("found input {}", input.client_height());
            match input.get_attribute("placeholder") {
                Some(placeholder) if placeholder == "Alice Smith" => {
                    let input: HtmlInputElement = input.dyn_into().unwrap();
                    input.click();
                    input
                        .focus()
                        .unwrap_or_else(|_e| log!("can't focus input"));

                    input.set_value("John Miller");

                    if let Ok(input) = input.dyn_into::<EventTarget>() {
                        if let Ok(event) = InputEvent::new("input") {
                            if input.dispatch_event(&event).is_ok() {
                                sleep(Duration::from_secs(3)).await;
                            }
                        }
                    }
                },
                Some(placeholder) if placeholder == "Alice" => {
                    let input: HtmlInputElement = input.dyn_into().unwrap();
                    input.click();
                    input
                        .focus()
                        .unwrap_or_else(|_e| log!("can't focus input"));

                    input.set_value("John");

                    if let Ok(input) = input.dyn_into::<EventTarget>() {
                        if let Ok(event) = InputEvent::new("input") {
                            if input.dispatch_event(&event).is_ok() {
                                sleep(Duration::from_secs(3)).await;
                            }
                        }
                    }
                },
                Some(placeholder) if placeholder == "Smith" => {
                    let input: HtmlInputElement = input.dyn_into().unwrap();
                    input.click();
                    input
                        .focus()
                        .unwrap_or_else(|_e| log!("can't focus input"));

                    input.set_value("Miller");

                    if let Ok(input) = input.dyn_into::<EventTarget>() {
                        if let Ok(event) = InputEvent::new("input") {
                            if input.dispatch_event(&event).is_ok() {
                                sleep(Duration::from_secs(3)).await;
                            }
                        }
                    }
                },
                Some(placeholder) if placeholder == "alice.smith@example.com" => {
                    let input: HtmlInputElement = input.dyn_into().unwrap();
                    input.click();
                    input
                        .focus()
                        .unwrap_or_else(|_e| log!("can't focus input"));

                    input.set_value(&infos.lock().unwrap().0);

                    if let Ok(input) = input.dyn_into::<EventTarget>() {
                        if let Ok(event) = InputEvent::new("input") {
                            if input.dispatch_event(&event).is_ok() {
                                sleep(Duration::from_secs(3)).await;
                            }
                        }
                    }
                },
                Some(placeholder) if placeholder == "MM/DD/YYYY" || placeholder == "DD/MM/YYYY" => {
                    let input: HtmlInputElement = input.dyn_into().unwrap();
                    input.click();
                    input
                        .focus()
                        .unwrap_or_else(|_e| log!("can't focus input"));

                    input.set_value("02/02/1964");

                    if let Ok(input) = input.dyn_into::<EventTarget>() {
                        if let Ok(event) = InputEvent::new("input") {
                            if input.dispatch_event(&event).is_ok() {
                                sleep(Duration::from_secs(3)).await;
                            }
                        }
                    }
                },
                _ if input.get_attribute("type") == Some(String::from("checkbox")) => if input.get_attribute("ng-required") == Some(String::from("true")) {
                    let input: HtmlElement = input.dyn_into().unwrap();
                    input.click();
                    sleep(Duration::from_secs(3)).await;
                }
                placeholder => log!("Strange input element: placeholder {:?}, type: {:?}", placeholder, input.get_attribute("type"))
            }
        }

        // save button
        if let Some(save_button) = document
            .get_elements_by_class_name("btn btn-primary ng-scope")
            .item(0)
        {
            let save_button: HtmlElement =
                save_button.dyn_into().unwrap();
            save_button.click();

            sleep(Duration::from_secs(2)).await;
        }
    }

    let entries_elements = document.get_elements_by_class_name("entry-method");

    let mut entries = Vec::new();
    for entry_idx in 0..entries_elements.length() {
        let original_entry = entries_elements.item(entry_idx).unwrap();
        let entry = original_entry.first_element_child().unwrap();
        if let Some(class) = entry.get_attribute("class") {
            if class.contains("mandatory") && !class.contains("done") {
                entries.insert(0, (original_entry, entry));
            } else if !class.contains("done") {
                entries.push((original_entry, entry));
            }
        } else {
            entries.push((original_entry, entry));
        }
    }

    for (original_entry, entry) in entries {
        match EntryType::try_from(entry.get_attribute("data-track-event").unwrap()) {
            Ok(entry_type) => {
                console::log_1(&JsValue::from(format!("{:?}", entry_type)));
                let entry: HtmlElement = entry.dyn_into().unwrap();

                match (&entry_type.platform, &entry_type.action_required) {
                    (Platform::Youtube, ActionType::VisitChannel)
                    | (Platform::Instagram, ActionType::VisitProfile) => {
                        entry.click();
                        sleep(Duration::from_secs(2)).await;

                        // form asking name, email and birthdate
                        let inputs = original_entry.get_elements_by_tag_name("input");
                        let mut input_elements = Vec::new();
                        for idx in 0..inputs.length() {
                            input_elements.push(inputs.item(idx).unwrap());
                        }
                        for input in input_elements {
                            match input.get_attribute("placeholder") {
                                Some(placeholder) if placeholder == "Alice Smith" => {
                                    let input: HtmlInputElement = input.dyn_into().unwrap();
                                    input.click();
                                    input
                                        .focus()
                                        .unwrap_or_else(|_e| log!("can't focus input"));

                                    input.set_value("John Miller");

                                    if let Ok(input) = input.dyn_into::<EventTarget>() {
                                        if let Ok(event) = InputEvent::new("input") {
                                            if input.dispatch_event(&event).is_ok() {
                                                sleep(Duration::from_secs(3)).await;
                                            }
                                        }
                                    }
                                },
                                Some(placeholder) if placeholder == "alice.smith@example.com" => {
                                    let input: HtmlInputElement = input.dyn_into().unwrap();
                                    input.click();
                                    input
                                        .focus()
                                        .unwrap_or_else(|_e| log!("can't focus input"));

                                    input.set_value(&infos.lock().unwrap().0);

                                    if let Ok(input) = input.dyn_into::<EventTarget>() {
                                        if let Ok(event) = InputEvent::new("input") {
                                            if input.dispatch_event(&event).is_ok() {
                                                sleep(Duration::from_secs(3)).await;
                                            }
                                        }
                                    }
                                },
                                Some(placeholder) if placeholder == "MM/DD/YYYY" || placeholder == "DD/MM/YYYY" => {
                                    let input: HtmlInputElement = input.dyn_into().unwrap();
                                    input.click();
                                    input
                                        .focus()
                                        .unwrap_or_else(|_e| log!("can't focus input"));

                                    input.set_value("02/02/1964");

                                    if let Ok(input) = input.dyn_into::<EventTarget>() {
                                        if let Ok(event) = InputEvent::new("input") {
                                            if input.dispatch_event(&event).is_ok() {
                                                sleep(Duration::from_secs(3)).await;
                                            }
                                        }
                                    }
                                },
                                _ if input.get_attribute("type") == Some(String::from("checkbox")) => if input.get_attribute("ng-required") == Some(String::from("true")) {
                                    let input: HtmlElement = input.dyn_into().unwrap();
                                    input.click();
                                    sleep(Duration::from_secs(3)).await;
                                }
                                placeholder => log!("Strange input element: placeholder {:?}, type: {:?}", placeholder, input.get_attribute("type"))
                            }
                        }

                        // save button
                        if let Some(save_button) = original_entry
                            .get_elements_by_class_name("btn btn-primary")
                            .item(0)
                        {
                            let save_button: HtmlElement =
                                save_button.dyn_into().unwrap();
                            save_button.click();

                            sleep(Duration::from_secs(2)).await;
                        }

                        let big_button: HtmlElement = original_entry
                            .get_elements_by_class_name(
                                "btn btn-info btn-large btn-embossed ng-binding",
                            )
                            .item(0)
                            .unwrap()
                            .first_element_child()
                            .unwrap()
                            .dyn_into()
                            .unwrap();
                        big_button.click();
                        sleep(Duration::from_secs(2)).await;

                        // select
                        if let Some(option) =
                            original_entry.get_elements_by_tag_name("option").item(1)
                        {
                            if let Some(Ok(select)) = original_entry.get_elements_by_tag_name("select").item(0).map(|e| e.dyn_into::<HtmlElement>()) {
                                if let Ok(event) = Event::new("change") {
                                    if select.dispatch_event(&event).is_ok() {
                                        option.set_attribute("selected", "selected").unwrap();
                                        sleep(Duration::from_secs(2)).await;
                                    }
                                }
                            }
                        }

                        // confirmation button
                        if let Some(confirmation_button) = original_entry
                            .get_elements_by_class_name("btn btn-primary")
                            .item(0)
                        {
                            let confirmation_button: HtmlElement =
                                confirmation_button.dyn_into().unwrap();
                            confirmation_button.click();

                            sleep(Duration::from_secs(2)).await;
                        }
                    }
                    (Platform::Facebook, ActionType::Visit) => {
                        entry.click();
                        sleep(Duration::from_secs(1)).await;

                        // form asking name, email and birthdate
                        let inputs = original_entry.get_elements_by_tag_name("input");
                        let mut input_elements = Vec::new();
                        for idx in 0..inputs.length() {
                            input_elements.push(inputs.item(idx).unwrap());
                        }
                        for input in input_elements {
                            match input.get_attribute("placeholder") {
                                Some(placeholder) if placeholder == "Alice Smith" => {
                                    let input: HtmlInputElement = input.dyn_into().unwrap();
                                    input.click();
                                    input
                                        .focus()
                                        .unwrap_or_else(|_e| log!("can't focus input"));

                                    input.set_value("John Miller");

                                    if let Ok(input) = input.dyn_into::<EventTarget>() {
                                        if let Ok(event) = InputEvent::new("input") {
                                            if input.dispatch_event(&event).is_ok() {
                                                sleep(Duration::from_secs(3)).await;
                                            }
                                        }
                                    }
                                },
                                Some(placeholder) if placeholder == "alice.smith@example.com" => {
                                    let input: HtmlInputElement = input.dyn_into().unwrap();
                                    input.click();
                                    input
                                        .focus()
                                        .unwrap_or_else(|_e| log!("can't focus input"));

                                    input.set_value(&infos.lock().unwrap().0);

                                    if let Ok(input) = input.dyn_into::<EventTarget>() {
                                        if let Ok(event) = InputEvent::new("input") {
                                            if input.dispatch_event(&event).is_ok() {
                                                sleep(Duration::from_secs(3)).await;
                                            }
                                        }
                                    }
                                },
                                Some(placeholder) if placeholder == "MM/DD/YYYY" || placeholder == "DD/MM/YYYY" => {
                                    let input: HtmlInputElement = input.dyn_into().unwrap();
                                    input.click();
                                    input
                                        .focus()
                                        .unwrap_or_else(|_e| log!("can't focus input"));

                                    input.set_value("02/02/1964");

                                    if let Ok(input) = input.dyn_into::<EventTarget>() {
                                        if let Ok(event) = InputEvent::new("input") {
                                            if input.dispatch_event(&event).is_ok() {
                                                sleep(Duration::from_secs(3)).await;
                                            }
                                        }
                                    }
                                },
                                _ if input.get_attribute("type") == Some(String::from("checkbox")) => if input.get_attribute("ng-required") == Some(String::from("true")) {
                                    let input: HtmlElement = input.dyn_into().unwrap();
                                    input.click();
                                    sleep(Duration::from_secs(3)).await;
                                }
                                placeholder => log!("Strange input element: placeholder {:?}, type: {:?}", placeholder, input.get_attribute("type"))
                            }
                        }

                        // save button
                        if let Some(save_button) = original_entry
                            .get_elements_by_class_name("btn btn-primary")
                            .item(0)
                        {
                            let save_button: HtmlElement =
                                save_button.dyn_into().unwrap();
                            save_button.click();

                            sleep(Duration::from_secs(2)).await;
                        }

                        // special facebook link
                        if let Some(facebook_link) = original_entry
                            .get_elements_by_class_name("facebook-heading ng-binding")
                            .item(0)
                        {
                            let facebook_link: HtmlElement = facebook_link.dyn_into().unwrap();
                            facebook_link.click();
                            sleep(Duration::from_secs(2)).await;
                        }

                        // "call to action" button
                        if let Some(big_button) = original_entry
                            .get_elements_by_class_name("btn btn-info btn-large btn-embossed")
                            .item(0)
                        {
                            if let Ok(big_button) = big_button.dyn_into::<HtmlElement>() {
                                big_button.click();
                                sleep(Duration::from_secs(2)).await;
                            }
                        }

                        // confirmation button
                        if let Some(confirmation_button) = original_entry
                            .get_elements_by_class_name("btn btn-primary")
                            .item(0)
                        {
                            let confirmation_button: HtmlElement =
                                confirmation_button.dyn_into().unwrap();
                            confirmation_button.click();

                            sleep(Duration::from_secs(2)).await;
                        }
                    }
                    (_, ActionType::Enter)
                    | (Platform::Twitch, ActionType::Follow)
                    | (Platform::Mixer, ActionType::Follow)
                    | (Platform::Custom, ActionType::Action)
                    | (Platform::Email, ActionType::Subscribe)
                    | (Platform::Youtube, ActionType::Subscribe)
                    | (Platform::Loyalty, ActionType::Loyalty) => {
                        entry.click();
                        sleep(Duration::from_secs(2)).await;

                        // form asking name, email and birthdate
                        let inputs = original_entry.get_elements_by_tag_name("input");
                        let mut input_elements = Vec::new();
                        for idx in 0..inputs.length() {
                            input_elements.push(inputs.item(idx).unwrap());
                        }
                        for input in input_elements {
                            match input.get_attribute("placeholder") {
                                Some(placeholder) if placeholder == "Alice Smith" => {
                                    let input: HtmlInputElement = input.dyn_into().unwrap();
                                    input.click();
                                    input
                                        .focus()
                                        .unwrap_or_else(|_e| log!("can't focus input"));

                                    input.set_value("John Miller");

                                    if let Ok(input) = input.dyn_into::<EventTarget>() {
                                        if let Ok(event) = InputEvent::new("input") {
                                            if input.dispatch_event(&event).is_ok() {
                                                sleep(Duration::from_secs(3)).await;
                                            }
                                        }
                                    }
                                },
                                Some(placeholder) if placeholder == "alice.smith@example.com" => {
                                    let input: HtmlInputElement = input.dyn_into().unwrap();
                                    input.click();
                                    input
                                        .focus()
                                        .unwrap_or_else(|_e| log!("can't focus input"));

                                    input.set_value(&infos.lock().unwrap().0);

                                    if let Ok(input) = input.dyn_into::<EventTarget>() {
                                        if let Ok(event) = InputEvent::new("input") {
                                            if input.dispatch_event(&event).is_ok() {
                                                sleep(Duration::from_secs(3)).await;
                                            }
                                        }
                                    }
                                },
                                Some(placeholder) if placeholder == "MM/DD/YYYY" || placeholder == "DD/MM/YYYY" => {
                                    let input: HtmlInputElement = input.dyn_into().unwrap();
                                    input.click();
                                    input
                                        .focus()
                                        .unwrap_or_else(|_e| log!("can't focus input"));

                                    input.set_value("02/02/1964");

                                    if let Ok(input) = input.dyn_into::<EventTarget>() {
                                        if let Ok(event) = InputEvent::new("input") {
                                            if input.dispatch_event(&event).is_ok() {
                                                sleep(Duration::from_secs(3)).await;
                                            }
                                        }
                                    }
                                },
                                _ if input.get_attribute("type") == Some(String::from("checkbox")) => if input.get_attribute("ng-required") == Some(String::from("true")) {
                                    let input: HtmlElement = input.dyn_into().unwrap();
                                    input.click();
                                    sleep(Duration::from_secs(3)).await;
                                }
                                placeholder => log!("Strange input element: placeholder {:?}, type: {:?}", placeholder, input.get_attribute("type"))
                            }
                        }

                        // save button
                        if let Some(save_button) = original_entry
                            .get_elements_by_class_name("btn btn-primary")
                            .item(0)
                        {
                            let save_button: HtmlElement =
                                save_button.dyn_into().unwrap();
                            save_button.click();

                            sleep(Duration::from_secs(2)).await;
                        }

                        // "call to action" button
                        if let Some(big_button) = original_entry
                            .get_elements_by_class_name("btn btn-info btn-large btn-embossed")
                            .item(0)
                        {
                            if let Ok(big_button) = big_button.dyn_into::<HtmlElement>() {
                                big_button.click();
                                sleep(Duration::from_secs(2)).await;
                            }
                        }

                        // form with checkboxes
                        if let Some(first_ul) =
                            original_entry.get_elements_by_tag_name("label").item(0)
                        {
                            if let Ok(first_ul) = first_ul.dyn_into::<HtmlElement>() {
                                first_ul.click();
                                sleep(Duration::from_secs(2)).await;
                            }
                        }

                        // select
                        if let Some(option) =
                            original_entry.get_elements_by_tag_name("option").item(1)
                        {
                            if let Some(Ok(select)) = original_entry.get_elements_by_tag_name("select").item(0).map(|e| e.dyn_into::<HtmlElement>()) {
                                if let Ok(event) = Event::new("change") {
                                    if select.dispatch_event(&event).is_ok() {
                                        option.set_attribute("selected", "selected").unwrap();
                                        sleep(Duration::from_secs(2)).await;
                                    }
                                }
                            }
                        }

                        // input element
                        if let Some(input) =
                            original_entry.get_elements_by_tag_name("input").item(0)
                        {
                            if let Ok(input) = input.dyn_into::<HtmlInputElement>() {
                                input.click();
                                input
                                    .focus()
                                    .unwrap_or_else(|_e| log!("can't focus input"));

                                input.set_value("我不明白你在问什么");

                                if let Ok(input) = input.dyn_into::<EventTarget>() {
                                    if let Ok(event) = InputEvent::new("input") {
                                        if input.dispatch_event(&event).is_ok() {
                                            sleep(Duration::from_secs(3)).await;
                                        }
                                    }
                                }
                            }
                        }

                        // textarea element
                        if let Some(textarea) =
                            original_entry.get_elements_by_tag_name("textarea").item(0)
                        {
                            if let Ok(textarea) = textarea.dyn_into::<HtmlTextAreaElement>() {
                                textarea.click();
                                textarea
                                    .focus()
                                    .unwrap_or_else(|_e| log!("can't focus textarea"));

                                textarea.set_value("我不明白你在问什么");

                                if let Ok(textarea) = textarea.dyn_into::<EventTarget>() {
                                    if let Ok(event) = InputEvent::new("input") {
                                        if textarea.dispatch_event(&event).is_ok() {
                                            sleep(Duration::from_secs(3)).await;
                                        }
                                    }
                                }
                            }
                        }

                        // confirmation button
                        if let Some(confirmation_button) = original_entry
                            .get_elements_by_class_name("btn btn-primary")
                            .item(0)
                        {
                            let confirmation_button: HtmlElement =
                                confirmation_button.dyn_into().unwrap();
                            confirmation_button.click();

                            sleep(Duration::from_secs(2)).await;
                        }
                    }
                    (Platform::Twitter, ActionType::Retweet)
                    | (Platform::Twitter, ActionType::Tweet)
                    | (Platform::Twitter, ActionType::Follow) => {
                        entry.click();
                        sleep(Duration::from_secs(5)).await;

                        if let Some(follow_button) = original_entry
                            .get_elements_by_class_name("xl twitter-button")
                            .item(0)
                        {
                            let follow_button: HtmlElement = follow_button.dyn_into().unwrap();
                            follow_button.click();
                            sleep(Duration::from_secs(10)).await;

                            // confirmation button
                            if let Some(confirmation_button) = original_entry
                                .get_elements_by_class_name("btn btn-primary")
                                .item(0)
                            {
                                let confirmation_button: HtmlElement =
                                    confirmation_button.dyn_into().unwrap();
                                confirmation_button.click();

                                sleep(Duration::from_secs(2)).await;
                            }

                            sleep(Duration::from_secs(2)).await;
                        }
                    }
                    (Platform::Youtube, ActionType::Video) => {
                        entry.click();
                        sleep(Duration::from_secs(2)).await;

                        // input element
                        if let Some(input) =
                            original_entry.get_elements_by_tag_name("input").item(0)
                        {
                            if let Ok(input) = input.dyn_into::<HtmlInputElement>() {
                                input.click();
                                input
                                    .focus()
                                    .unwrap_or_else(|_e| log!("can't focus button"));

                                input.set_value("https://www.youtube.com/watch?v=dVVZaZ8yO6o");

                                if let Ok(input) = input.dyn_into::<EventTarget>() {
                                    if let Ok(event) = InputEvent::new("input") {
                                        if input.dispatch_event(&event).is_ok() {
                                            sleep(Duration::from_secs(4)).await;
                                        }
                                    }
                                }
                            }
                        }

                        // confirmation button
                        if let Some(confirmation_button) = original_entry
                            .get_elements_by_class_name("btn btn-primary")
                            .item(0)
                        {
                            let confirmation_button: HtmlElement =
                                confirmation_button.dyn_into().unwrap();
                            confirmation_button.click();

                            sleep(Duration::from_secs(2)).await;
                        }
                    }
                    _ => log!("Unsupported action {:?} on the platform {:?}", &entry_type.action_required, &entry_type.platform),
                }
            }
            Err(e) => console::error_1(&JsValue::from(e)),
        }
    }

    link.send_message(Msg::Done);
    Ok(JsValue::null())
}