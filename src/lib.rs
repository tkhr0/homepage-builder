use rand;
use std::convert::From;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::web_sys::HtmlInputElement as InputElement;

type Id = u8;

trait Componentable: PartialEq {
    fn component_name(&self) -> &'static str;
    fn element_data(&self) -> ElementData;
}

trait RawElementData {
    fn id(&self) -> &Id;
    fn render_html(&self) -> Html;
}

fn gen_id() -> Id {
    rand::random()
}

struct ElementData {
    data: Box<dyn RawElementData>,
}

impl ElementData {
    fn render(&self) -> Html {
        self.data.render_html()
    }
}

impl PartialEq<ElementData> for ElementData {
    fn eq(&self, other: &Self) -> bool {
        self.data.id() == other.data.id()
    }
}

#[derive(PartialEq, Clone)]
struct Paragraph {
    value: String,
    style: Style,
    id: Id,
}

impl RawElementData for Paragraph {
    fn id(&self) -> &Id {
        &self.id
    }

    fn render_html(&self) -> Html {
        html! {
            <p style={self.style.to_css()}>
                {self.value.clone()}
            </p>
        }
    }
}

#[derive(PartialEq, Clone)]
struct Anchor {
    text: String,
    href: String,
    style: Style,
    id: Id,
}

impl RawElementData for Anchor {
    fn id(&self) -> &Id {
        &self.id
    }

    fn render_html(&self) -> Html {
        html! {
            <a href={self.href.clone()} style={self.style.to_css()}>
                {self.text.clone()}
            </a>
        }
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

    fn element_data(&self) -> ElementData {
        ElementData {
            data: Box::new(self.clone()),
        }
    }
}

impl Componentable for Anchor {
    fn component_name(&self) -> &'static str {
        "anchor"
    }

    fn element_data(&self) -> ElementData {
        ElementData {
            data: Box::new(self.clone()),
        }
    }
}

impl From<Paragraph> for ElementData {
    fn from(prop: Paragraph) -> Self {
        Self {
            data: Box::new(prop),
        }
    }
}

impl From<Anchor> for ElementData {
    fn from(prop: Anchor) -> Self {
        Self {
            data: Box::new(prop),
        }
    }
}

struct CustomComponent {}

struct Msg {}

#[derive(Properties, PartialEq)]
struct CustomComponentProps {
    elements: Vec<ElementData>,
}

impl Component for CustomComponent {
    type Message = Msg;
    type Properties = CustomComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                {ctx.props().elements.iter().map(|element| element.render()).collect::<Html>()}
            </>
        }
    }
}

#[derive(Properties, PartialEq)]
struct SettingPaneProps {
    oninput: Callback<Option<String>>,
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
        if let Some(color) = &self.color {
            css.push_str(format!("color: {};", color).as_str());
        }

        css
    }
}

#[function_component(App)]
fn app() -> Html {
    let paragraph = Paragraph {
        value: "hoge".to_string(),
        style: Style {
            background_color: Some("#666666".into()),
            color: Some("#222222".into()),
        },
        id: gen_id(),
    };

    let anchor = Anchor {
        text: "hoge".to_string(),
        href: "https://www.google.co.jp".to_string(),
        style: Style {
            background_color: Some("#666666".into()),
            color: Some("#222222".into()),
        },
        id: gen_id(),
    };

    // let oninput = {
    //     let paragraph = paragraph.clone();
    //     Callback::from(move |background_color: Option<String>| {
    //         let mut new = (*paragraph).clone();
    //         new.style.background_color = background_color;
    //         paragraph.set(new);
    //     })
    // };
    let oninput = Callback::from(move |_| {});

    let elements: Vec<ElementData> = vec![paragraph.into(), anchor.into()];

    html! {
        <div>
            <div><CustomComponent elements={elements}/></div>
            <SettingPane oninput={oninput} />
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<App>();
}
