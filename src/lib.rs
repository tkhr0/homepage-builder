use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {}

struct Paragraph {
    value: String,
    style: Style,
}

struct SettingPane {
    link: ComponentLink<Self>,
    value: String,
}

#[derive(Default)]
struct Style {
    background_color: Option<String>,
}

enum Msg {
    Update(String),
}

impl Component for Paragraph {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            value: "paragraph".to_string(),
            style: Style::default(),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <p style=self.style()>{&self.value}</p>
        }
    }
}

impl Paragraph {
    fn style(&self) -> String {
        self.style.to_css()
    }
}

impl Component for SettingPane {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: "foo".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(value) => self.value = value,
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <section>
                    <p>{ "background color" }</p>
                    <input type="text"
                           value=self.value.clone()
                           oninput=self.link.callback(|e: InputData| Msg::Update(e.value))/>
                    <p>{&self.value}</p>
                </section>
            </div>
        }
    }
}

impl Style {
    fn to_css(&self) -> String {
        let mut css = String::from("");

        if let Some(background_color) = &self.background_color {
            css.push_str(format!("background-color: {};", background_color).as_str());
        };

        css
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div>
                    <Paragraph/>
                </div>
                <SettingPane/>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
