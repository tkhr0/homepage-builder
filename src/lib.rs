use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::web_sys::HtmlInputElement as InputElement;

#[derive(PartialEq, Clone)]
struct Paragraph {
    value: String,
    style: Style,
}

#[derive(Default, PartialEq, Clone)]
struct Style {
    background_color: Option<String>,
}

#[derive(Properties, PartialEq)]
struct ParagraphProps {
    paragraph: Paragraph,
}

#[derive(Properties, PartialEq)]
struct SettingPaneProps {
    oninput: Callback<Option<String>>,
}

#[function_component(ParagraphComponent)]
fn paragraph(ParagraphProps { paragraph }: &ParagraphProps) -> Html {
    html! {
        <p style={paragraph.style.to_css()}>{paragraph.value.as_str()}</p>
    }
}

#[function_component(SettingPane)]
fn setting_pane(SettingPaneProps { oninput }: &SettingPaneProps) -> Html {
    let oninput_value = {
        let oninput = oninput.clone();

        Callback::from(move |event: InputEvent| {
            let input: InputElement = event.target_unchecked_into();
            let value = input.value();

            if value.is_empty() {
                oninput.emit(None)
            } else {
                oninput.emit(Some(value))
            }
        })
    };

    html! {
        <div>
            <section>
                <p>{ "background color" }</p>
                <input type="text"
                       oninput={oninput_value.clone()} />
                <p>{ "value" }</p>
            </section>
        </div>
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

#[function_component(App)]
fn app() -> Html {
    let paragraph = use_state(|| Paragraph {
        value: "hoge".to_string(),
        style: Style {
            background_color: Some("#222222".into()),
        },
    });

    let oninput = {
        let paragraph = paragraph.clone();
        Callback::from(move |background_color: Option<String>| {
            let mut new = (*paragraph).clone();
            new.style.background_color = background_color;
            paragraph.set(new);
        })
    };

    html! {
        <div>
            <div>
                <ParagraphComponent paragraph={(*paragraph).clone()}/>
            </div>
            <SettingPane oninput={oninput} />
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<App>();
}
