use std::convert::From;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::web_sys::HtmlInputElement as InputElement;

trait Componentable: PartialEq {
    fn component_name(&self) -> &'static str;
}

trait ComponentPropable: PartialEq {
    fn element_data(&self) -> Box<dyn ElementData>;
}

trait ElementData {
    fn value(&self) -> &String;
    fn style(&self) -> &Style;
    fn href(&self) -> Option<String> {
        None
    }
}

#[derive(PartialEq, Clone)]
struct Paragraph {
    value: String,
    style: Style,
}

impl ElementData for Paragraph {
    fn value(&self) -> &String {
        &self.value
    }

    fn style(&self) -> &Style {
        &self.style
    }
}

#[derive(PartialEq, Clone)]
struct Anchor {
    text: String,
    href: String,
    style: Style,
}

impl ElementData for Anchor {
    fn value(&self) -> &String {
        &self.text
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn href(&self) -> Option<String> {
        Some(self.href.clone())
    }
}

#[derive(Default, PartialEq, Clone)]
struct Style {
    color: Option<String>,
    background_color: Option<String>,
}

impl Componentable for Paragraph {
    fn component_name(&self) -> &'static str {
        "paragraph"
    }
}

impl Componentable for Anchor {
    fn component_name(&self) -> &'static str {
        "anchor"
    }
}

#[derive(Properties, PartialEq)]
struct ParagraphProps {
    paragraph: Paragraph,
}

#[derive(Properties, PartialEq)]
struct AnchorProps {
    anchor: Anchor,
}

impl ComponentPropable for ParagraphProps {
    fn element_data(&self) -> Box<dyn ElementData> {
        Box::new(self.paragraph.clone())
    }
}
impl ComponentPropable for AnchorProps {
    fn element_data(&self) -> Box<dyn ElementData> {
        Box::new(self.anchor.clone())
    }
}

impl From<Box<dyn ElementData>> for Paragraph {
    fn from(prop: Box<dyn ElementData>) -> Self {
        Self {
            value: (*prop.value()).clone(),
            style: (*prop.style()).clone(),
        }
    }
}

impl From<Box<dyn ElementData>> for Anchor {
    fn from(prop: Box<dyn ElementData>) -> Self {
        Self {
            text: (*prop.value()).clone(),
            href: prop.href().clone().or(Some("".to_string())).unwrap(),
            style: (*prop.style()).clone(),
        }
    }
}

#[derive(Properties, PartialEq)]
struct CustomComponentProps<T, U>
where
    T: Componentable,
    U: ComponentPropable,
{
    component: T,
    component_props: U,
}

#[derive(Properties, PartialEq)]
struct SettingPaneProps {
    oninput: Callback<Option<String>>,
}

// #[function_component(ParagraphComponent)]
fn paragraph(ParagraphProps { paragraph }: &ParagraphProps) -> Html {
    html! {
        <p style={paragraph.style.to_css()}>{paragraph.value.as_str()}</p>
    }
}

// #[function_component(AnchorComponent)]
fn anchor(AnchorProps { anchor }: &AnchorProps) -> Html {
    html! {
        <a href={anchor.href.clone()} style={anchor.style.to_css()}>{anchor.text.as_str()}</a>
    }
}

#[function_component(CustomComponent)]
fn custom_component<T, U>(
    CustomComponentProps {
        component,
        component_props,
    }: &CustomComponentProps<T, U>,
) -> Html
where
    T: Componentable,
    U: ComponentPropable,
{
    match component.component_name() {
        "paragraph" => paragraph(&ParagraphProps {
            paragraph: component_props.element_data().into(),
        }),
        "anchor" => anchor(&AnchorProps {
            anchor: component_props.element_data().into(),
        }),
        _ => html! {},
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
            background_color: Some("#666666".into()),
            color: Some("#222222".into()),
        },
    });

    let anchor = use_state(|| Anchor {
        text: "hoge".to_string(),
        href: "https://www.google.co.jp".to_string(),
        style: Style {
            background_color: Some("#666666".into()),
            color: Some("#222222".into()),
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
            // <div>
            //     <ParagraphComponent paragraph={(*paragraph).clone()}/>
            // </div>
            // <div>
            //     <AnchorComponent anchor={(*anchor).clone()}/>
            // </div>
            // TODO: component がいらない. component_props に含まれている
            <div><CustomComponent component_props={paragraph}/></div>
            <div><CustomComponent /></div>
            <SettingPane oninput={oninput} />
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<App>();
}
