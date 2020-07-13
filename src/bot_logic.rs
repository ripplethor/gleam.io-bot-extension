use crate::enums::*;
use crate::util::*;
use crate::{
    messages::Message,
    yew_app::{Model, Msg},
};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use string_tools::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;
use yew::prelude::*;

fn dispatch_input_event<T: Into<EventTarget>>(element: T) -> Result<(), JsValue> {
    let element: EventTarget = element.try_into().unwrap();
    let event = InputEvent::new("input")?;
    element.dispatch_event(&event)?;
    Ok(())
}

async fn check_connection_form(
    original_entry: &Element,
    infos: Arc<Mutex<(String, String)>>,
    where_to_find_save_button: &Element,
) -> Result<(), (JsValue, &'static str, u32)> {
    let inputs = original_entry.get_elements_by_tag_name("input");
    let mut input_elements = Vec::new();
    for idx in 0..inputs.length() {
        input_elements.push(inputs.item(idx).unwrap());
    }
    for input in input_elements {
        match input.get_attribute("placeholder") {
            Some(placeholder) if placeholder == "Alice Smith" => {
                let input: HtmlInputElement =
                    input.dyn_into().map_err(|e| (e.into(), file!(), line!()))?;
                input.click();
                input.focus().unwrap_or_else(|_e| log!("can't focus input"));

                input.set_value(&infos.lock().unwrap().1);

                dispatch_input_event(input).map_err(|e| (e, file!(), line!()))?;
                sleep(Duration::from_secs(3)).await;
            }
            Some(placeholder) if placeholder == "Alice" => {
                let input: HtmlInputElement =
                    input.dyn_into().map_err(|e| (e.into(), file!(), line!()))?;
                input.click();
                input.focus().unwrap_or_else(|_e| log!("can't focus input"));

                input.set_value(get_all_before(&infos.lock().unwrap().1, " "));

                dispatch_input_event(input).map_err(|e| (e, file!(), line!()))?;
                sleep(Duration::from_secs(3)).await;
            }
            Some(placeholder) if placeholder == "Smith" => {
                let input: HtmlInputElement = input.dyn_into().unwrap();
                input.click();
                input.focus().unwrap_or_else(|_e| log!("can't focus input"));

                input.set_value(get_all_after(&infos.lock().unwrap().1, " "));

                dispatch_input_event(input).map_err(|e| (e, file!(), line!()))?;
                sleep(Duration::from_secs(3)).await;
            }
            Some(placeholder) if placeholder == "alice.smith@example.com" => {
                let input: HtmlInputElement =
                    input.dyn_into().map_err(|e| (e.into(), file!(), line!()))?;
                input.click();
                input.focus().unwrap_or_else(|_e| log!("can't focus input"));

                input.set_value(&infos.lock().unwrap().0);

                dispatch_input_event(input).map_err(|e| (e, file!(), line!()))?;
                sleep(Duration::from_secs(3)).await;
            }
            Some(placeholder) if placeholder == "MM/DD/YYYY" || placeholder == "DD/MM/YYYY" => {
                let input: HtmlInputElement =
                    input.dyn_into().map_err(|e| (e.into(), file!(), line!()))?;
                input.click();
                input.focus().unwrap_or_else(|_e| log!("can't focus input"));

                input.set_value("02/02/1964");

                dispatch_input_event(input).map_err(|e| (e, file!(), line!()))?;
                sleep(Duration::from_secs(3)).await;
            }
            _ if input.get_attribute("type") == Some(String::from("checkbox")) => {
                if input.get_attribute("ng-required") == Some(String::from("true")) {
                    let input: HtmlElement =
                        input.dyn_into().map_err(|e| (e.into(), file!(), line!()))?;
                    input.click();
                    sleep(Duration::from_secs(3)).await;
                }
            }
            placeholder => log!(
                "Strange input element: placeholder {:?}, type: {:?}",
                placeholder,
                input.get_attribute("type")
            ),
        }
    }

    // save button
    if let Some(save_button) = where_to_find_save_button
        .get_elements_by_class_name("btn btn-primary") // removed ng-scope
        .item(0)
    {
        let save_button: HtmlElement = save_button
            .dyn_into()
            .map_err(|e| (e.into(), file!(), line!()))?;
        save_button.click();

        sleep(Duration::from_secs(2)).await;
    }

    Ok(())
}

fn confirm(container: &Element) -> Result<bool, (JsValue, &'static str, u32)> {
    // confirmation button
    if let Some(confirmation_button) = container
        .get_elements_by_class_name("btn btn-primary")
        .item(0)
    {
        let confirmation_button: HtmlElement = confirmation_button
            .dyn_into()
            .map_err(|e| (e.into(), file!(), line!()))?;
        confirmation_button.click();
        Ok(true)
    } else {
        Ok(false)
    }
}

fn press_action_button(container: &Element) -> Result<(), (JsValue, &'static str, u32)> {
    if let Some(big_button) = container
        .get_elements_by_class_name("btn btn-info btn-large btn-embossed")
        .item(0)
    {
        if let Ok(big_button) = big_button.dyn_into::<HtmlElement>() {
            big_button.click();
        }
    }
    Ok(())
}

pub async fn run(
    link: Rc<ComponentLink<Model>>,
    infos: Arc<Mutex<(String, String)>>,
) -> Result<(), Message<String>> {
    let window = window().expect("No window");
    let document = window.document().expect("No document");

    let entries_before = document
        .get_elements_by_class_name("status ng-binding")
        .item(0)
        .ok_or_else(|| ("None", file!(), line!()))?
        .inner_html()
        .trim()
        .parse::<usize>()
        .map_err(|e| (e, file!(), line!()))?;

    log!("Entering personal information");
    if let Some(form) = document.get_elements_by_class_name("contestant compact-box form-compact ng-pristine ng-scope ng-valid-pattern ng-invalid ng-invalid-required ng-valid-email").item(0) {

        // TODO test if third argument can't be equal to the first
        check_connection_form(&form, Arc::clone(&infos), &document.document_element().unwrap()).await?;
    }

    log!("Getting entry methods");
    let entries_elements = document.get_elements_by_class_name("entry-method");
    let mut entries = Vec::new();
    for entry_idx in 0..entries_elements.length() {
        let original_entry = entries_elements
            .item(entry_idx)
            .ok_or_else(|| ("None", file!(), line!()))?;
        let entry = original_entry
            .first_element_child()
            .ok_or_else(|| ("None", file!(), line!()))?;
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

    log!("Entering main loop");
    let len = entries.len() as f64;
    let mut current = 0.0;
    for (original_entry, entry) in entries {
        match EntryType::try_from(
            entry
                .get_attribute("data-track-event")
                .ok_or_else(|| ("None", file!(), line!()))?,
        ) {
            Ok(entry_type) => {
                log!("{:?}", entry_type);
                let entry: HtmlElement = entry.dyn_into().map_err(|e| (e, file!(), line!()))?;

                match (&entry_type.platform, &entry_type.action_required) {
                    (Platform::Youtube, ActionType::VisitChannel)
                    | (Platform::Instagram, ActionType::VisitProfile) => {
                        entry.click();
                        sleep(Duration::from_secs(2)).await;

                        // form asking name, email and birthdate
                        check_connection_form(&original_entry, Arc::clone(&infos), &original_entry)
                            .await?;

                        let big_button: HtmlElement = original_entry
                            .get_elements_by_class_name(
                                "btn btn-info btn-large btn-embossed ng-binding",
                            )
                            .item(0)
                            .ok_or_else(|| ("None", file!(), line!()))?
                            .first_element_child()
                            .ok_or_else(|| ("None", file!(), line!()))?
                            .dyn_into()
                            .map_err(|e| (e, file!(), line!()))?;
                        big_button.click();
                        sleep(Duration::from_secs(2)).await;

                        // select
                        if let Some(option) =
                            original_entry.get_elements_by_tag_name("option").item(1)
                        {
                            if let Some(Ok(select)) = original_entry
                                .get_elements_by_tag_name("select")
                                .item(0)
                                .map(|e| e.dyn_into::<HtmlElement>())
                            {
                                if let Ok(event) = Event::new("change") {
                                    if select.dispatch_event(&event).is_ok() {
                                        option
                                            .set_attribute("selected", "selected")
                                            .map_err(|e| (e, file!(), line!()))?;
                                        sleep(Duration::from_secs(2)).await;
                                    }
                                }
                            }
                        }

                        confirm(&original_entry);
                        sleep(Duration::from_secs(2)).await;
                    }
                    (Platform::Facebook, ActionType::Visit) => {
                        entry.click();
                        sleep(Duration::from_secs(1)).await;

                        // form asking name, email and birthdate
                        check_connection_form(&original_entry, Arc::clone(&infos), &original_entry)
                            .await?;

                        // special facebook link
                        if let Some(facebook_link) = original_entry
                            .get_elements_by_class_name("facebook-heading ng-binding")
                            .item(0)
                        {
                            let facebook_link: HtmlElement = facebook_link
                                .dyn_into()
                                .map_err(|e| (e, file!(), line!()))?;
                            facebook_link.click();
                            sleep(Duration::from_secs(2)).await;
                        }

                        press_action_button(&original_entry);
                        sleep(Duration::from_secs(3)).await;

                        confirm(&original_entry);
                        sleep(Duration::from_secs(2)).await;
                    }
                    (_, ActionType::Enter)
                    | (_, ActionType::ViewPost)
                    | (Platform::Twitch, ActionType::Follow)
                    | (Platform::Mixer, ActionType::Follow)
                    | (Platform::Custom, ActionType::Action)
                    | (Platform::Email, ActionType::Subscribe)
                    | (Platform::Youtube, ActionType::Subscribe)
                    | (Platform::Loyalty, ActionType::Loyalty) => {
                        entry.click();
                        sleep(Duration::from_secs(2)).await;

                        // form asking name, email and birthdate
                        check_connection_form(&original_entry, Arc::clone(&infos), &original_entry)
                            .await
                            .map_err(|e| (e, file!(), line!()))?;

                        if entry_type.action_required == ActionType::ViewPost {
                            sleep(Duration::from_secs(10)).await;
                        }

                        press_action_button(&original_entry);
                        sleep(Duration::from_secs(2)).await;

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
                            if let Some(Ok(select)) = original_entry
                                .get_elements_by_tag_name("select")
                                .item(0)
                                .map(|e| e.dyn_into::<HtmlElement>())
                            {
                                if let Ok(event) = Event::new("change") {
                                    if select.dispatch_event(&event).is_ok() {
                                        option
                                            .set_attribute("selected", "selected")
                                            .map_err(|e| (e, file!(), line!()))?;
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
                                input.focus().unwrap_or_else(|_e| log!("can't focus input"));

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

                        confirm(&original_entry);
                        sleep(Duration::from_secs(2)).await;
                    }
                    (Platform::Twitter, ActionType::Retweet)
                    | (Platform::Twitter, ActionType::Tweet)
                    | (Platform::Twitter, ActionType::Hashtags)
                    | (Platform::Twitter, ActionType::Follow) => {
                        entry.click();
                        sleep(Duration::from_secs(5)).await;

                        if let Some(follow_button) = original_entry
                            .get_elements_by_class_name("xl twitter-button")
                            .item(0)
                        {
                            let mut url = follow_button
                                .get_attribute("href")
                                .ok_or_else(|| ("None", file!(), line!()))?;
                            url.push_str("&gleambot=true");
                            window
                                .open_with_url(&url)
                                .map_err(|e| (e, file!(), line!()))?;

                            sleep(Duration::from_secs(11)).await;

                            confirm(&original_entry);
                            sleep(Duration::from_secs(2)).await;
                        }
                    }
                    (Platform::Youtube, ActionType::Video)
                    | (Platform::Submit, ActionType::Url) => {
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

                                dispatch_input_event(input).map_err(|e| (e, file!(), line!()))?;
                                sleep(Duration::from_secs(4)).await;
                            }
                        }

                        confirm(&original_entry);
                        sleep(Duration::from_secs(2)).await;
                    }
                    _ => {
                        let msg = format!(
                            "Unsupported action {:?} on the platform {:?}",
                            &entry_type.action_required, &entry_type.platform
                        );
                        elog!("{}", msg);
                        link.send_message(Msg::LogMessage(Message::Warning(msg)));
                    }
                }
            }
            Err(e) => {
                elog!("{}", e);
                link.send_message(Msg::LogMessage(Message::Error(e)));
            }
        }

        current += 1.0;
        link.send_message(Msg::ProgressChange(
            ((100.0 / len) * current).floor() as usize
        ))
    }

    link.send_message(Msg::Done);

    link.send_message(Msg::AddToStats(
        document
            .get_elements_by_class_name("status ng-binding")
            .item(0)
            .ok_or_else(|| ("None", file!(), line!()))?
            .inner_html()
            .trim()
            .parse::<usize>()
            .map_err(|e| (e, file!(), line!()))?
            - entries_before,
    ));

    Ok(())
}
