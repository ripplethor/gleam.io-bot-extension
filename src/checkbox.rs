
use yew::prelude::*;

pub struct Checkbox<T: Clone + Default + 'static> {
    link: ComponentLink<Self>,
    callback: Callback<(bool, T)>,
    checked: bool,
    label: String,
    id: T,
}

impl<T: Clone + Default> Checkbox<T> {
    pub fn _is_checked(&self) -> bool {
        self.checked
    }
}

impl<T: Clone + Default> Component for Checkbox<T> {
    type Message = ();
    type Properties = CheckboxProp<T>;

    fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self {
        Checkbox {
            link,
            checked: properties.checked,
            label: properties.label,
            id: properties.id,
            callback: properties.onchange
        }
    }

    fn update(&mut self, _: ()) -> bool {
        self.checked = !self.checked;
        self.callback.emit((self.checked, self.id.clone()));

        true
    }

    fn view(&self) -> Html {
        let class = if self.checked {
            "checkbox ng-binding checked"
        } else {
            "checkbox ng-binding"
        };

        html! {
            <label class=class>
                <span class="icon"></span>
                <span class="icon-to-fade"></span>
                <input type="checkbox" class="ng-pristine ng-untouched ng-valid ng-empty" name="check" value="check" onchange=self.link.callback(|_| ()) checked=self.checked />
                {&self.label}
            </label>
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}

#[derive(Clone, Properties)]
pub struct CheckboxProp<T: Clone + Default> {
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub id: T,
    #[prop_or(false)]
    pub checked: bool,
    #[prop_or_default]
    pub onchange: Callback<(bool, T)>,
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