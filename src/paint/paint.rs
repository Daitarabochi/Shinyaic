// TODO

use crate::css::parser::parser::Parser as CSSParser;
use crate::html::parser::parser::Parser;
use crate::paint::window_canvas::{create_block, create_text};
use crate::paint::wrapper::Wrapper;
use crate::render_tree::render_object::RenderObject;
use crate::render_tree::render_tree::RenderTree;
use iced::{
    scrollable, text_input, Color, Container, Element, Length, Sandbox, Scrollable, Settings,
    TextInput,
};
use konnnyaku_client::Client;

pub fn paint() {
    let mut settings = Settings::default();
    settings.window.size = (700, 700);
    Window::run(settings).unwrap();
}

// TODO
pub struct Window {
    scroll: scrollable::State,
    pub debug: bool,
    render_tree: Vec<RenderObject>,
    url_search_bar_text_value: text_input::State,
    url_searchbar_text: String,
}

fn prepare() -> RenderTree {
    // let mut parser = Parser {
    //     pos: 0,
    //     input: "<html><head></head><body><p id=\"id1\">hello browser!</p><p id=\"id2\">asdf</p><p id=\"id3\"></p></body></html>".to_string(),
    // };

    let mut parser = Parser {
        pos: 0,
        input: "<!doctype html>
<html>
<head>
<title>Example Domain</title>
<meta charset=\"utf-8\" />
<meta http-equiv=\"Content-type\" content=\"text/html; charset=utf-8\" />
<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />  
<style type=\"text/css\">
body {
    background-color: #f0f0f2;
    margin: 0;
    padding: 0;
    font-family: -apple-system, system-ui, BlinkMacSystemFont, \"Segoe UI\", \"Open Sans\", \"Helvetica Neue\", Helvetica, Arial, sans-serif;
}
div {
    width: 600px;
    margin: 5em auto;
    padding: 2em;
    background-color: #fdfdff;
    border-radius: 0.5em;
    box-shadow: 2px 3px 7px 2px rgba(0,0,0,0.02);
}
a:link, a:visited {
    color: #38488f;
    text-decoration: none;
}
@media (max-width: 700px) {
    div {
        margin: 0 auto;
        width: auto;
    }
}
</style>
</head>
<body>
<div>
    <h1>Example Domain</h1>
    <p>This domain is for use in illustrative examples in documents. You may use this domain in literature without prior coordination or asking for permission.</p>
    <p><a href=\"https://www.iana.org/domains/example\">More information...</a></p>
</div>
</body>
</html>".to_string()
    };

    let dom = parser.parse();
    println!("------");
    println!("{:?}", dom);

    let mut parser = CSSParser {
        pos: 0,
        input: "#id1 { width: 120; background: rgba(255, 0, 0, 1); margin: 0; }
        #id2 { width: 120; height: 90; background: rgba(0, 255, 0, 1); margin: 0; }
        #id3 { width: 120; height: 90; background: rgba(0, 0, 255, 1); margin: 0; }"
            .to_string(),
    };

    let cssom = parser.parse();
    println!("------");
    println!("{:?}", cssom);

    let mut render_tree = RenderTree::new(dom, cssom);
    render_tree.constructor();
    println!("------");
    println!("{:#?}", render_tree);

    render_tree
}

impl Sandbox for Window {
    type Message = Message;
    fn new() -> Window {
        let render_tree = prepare();
        let rendering_objects = render_tree.prepare_iterator();
        Window {
            scroll: scrollable::State::new(),
            debug: false,
            render_tree: rendering_objects,
            url_search_bar_text_value: text_input::State::new(),
            url_searchbar_text: "".to_string(),
        }
    }

    fn title(&self) -> String {
        String::from("shinyaic")
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::UrlSearchBarTextInputChanged(text) => {
                self.url_searchbar_text = text;
            }
            Message::UrlSearchBarSubmit(url) => {
                println!("{}", url);
                let response = Client::get(url);
                let body = response.body;
                let mut parser = Parser {
                    pos: 0,
                    input: body,
                };
                let dom = parser.parse();
                println!("------");
                println!("{:?}", dom);
                // TODO link の css をちゃんと読む
                let mut parser = CSSParser {
                    pos: 0,
                    input:
                        "#id1 { width: 120; height: 90; background: rgba(255, 0, 0, 1); margin: 0; }
                    #id2 { width: 120; height: 90; background: rgba(0, 255, 0, 1); margin: 0; }
                    #id3 { width: 120; height: 90; background: rgba(0, 0, 255, 1); margin: 0; }"
                            .to_string(),
                };
                let cssom = parser.parse();
                println!("------");
                println!("{:?}", cssom);
                let mut render_tree = RenderTree::new(dom, cssom);
                render_tree.constructor();

                self.render_tree = render_tree.prepare_iterator();
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let Window {
            scroll,
            url_searchbar_text,
            url_search_bar_text_value,
            ..
        } = self;

        let mut wrapper = Wrapper::new(700.0, 700.0);

        let url_search_bar = TextInput::new(
            url_search_bar_text_value,
            "url",
            url_searchbar_text,
            Message::UrlSearchBarTextInputChanged,
        )
        .padding(10)
        .on_submit(Message::UrlSearchBarSubmit(url_searchbar_text.to_string()));

        for item in &self.render_tree {
            match item {
                RenderObject::Text(text) => {
                    wrapper.items.push(create_text(
                        text.text.to_string(),
                        Color::new(0.0, 0.0, 0.0, 1.0),
                        text.rectangle.clone(),
                        text.font.clone(),
                    ));
                }
                RenderObject::ViewPort(rendering_object)
                | RenderObject::Block(rendering_object)
                | RenderObject::Inline(rendering_object)
                | RenderObject::Scroll(rendering_object) => {
                    let mut background_color = Color::new(1.0, 1.0, 1.0, 1.0);
                    for style in &rendering_object.style {
                        let background_color_value = {
                            if style.declarations.get(&"background".to_string()).is_some() {
                                style.declarations.get(&"background".to_string())
                            } else if style
                                .declarations
                                .get(&"background-color".to_string())
                                .is_some()
                            {
                                style.declarations.get(&"background-color".to_string())
                            } else {
                                None
                            }
                        };
                        if background_color_value.is_some() {
                            let mut raw_background_color = background_color_value.unwrap().clone();
                            let colors = if raw_background_color.starts_with("#") {
                                // TODO other module
                                let mut colors = vec![];

                                let mut color =
                                    raw_background_color.strip_prefix("#").unwrap().to_string();
                                if color.len() == 3 {
                                    let c = &color[0..1];
                                    let mut c = c.to_string();
                                    c.push_str(&c.clone());
                                    let z = u8::from_str_radix(&c, 16).unwrap();
                                    colors.push(z);

                                    let c = &color[1..2];
                                    let mut c = c.to_string();
                                    c.push_str(&c.clone());
                                    let z = u8::from_str_radix(&c, 16).unwrap();
                                    colors.push(z);

                                    let c = &color[2..3];
                                    let mut c = c.to_string();
                                    c.push_str(&c.clone());
                                    let z = u8::from_str_radix(&c, 16).unwrap();
                                    colors.push(z);
                                } else if color.len() == 6 {
                                    let c = &color[0..2];
                                    let z = u8::from_str_radix(&c, 16).unwrap();
                                    colors.push(z);
                                    let c = &color[2..4];
                                    let z = u8::from_str_radix(&c, 16).unwrap();
                                    colors.push(z);
                                    let c = &color[4..6];
                                    let z = u8::from_str_radix(&c, 16).unwrap();
                                    colors.push(z);
                                } else {
                                    panic!("invalid color");
                                }

                                (colors[0], colors[1], colors[2], 1.0)
                            } else {
                                raw_background_color.retain(|c| {
                                    c == ','
                                        || c == '.'
                                        || c == '1'
                                        || c == '0'
                                        || c == '2'
                                        || c == '3'
                                        || c == '4'
                                        || c == '5'
                                        || c == '6'
                                        || c == '7'
                                        || c == '8'
                                        || c == '9'
                                });

                                let colors = &raw_background_color;
                                let colors: Vec<&str> = colors.split(",").collect();
                                let colors = (
                                    colors[0].parse::<u8>().unwrap(),
                                    colors[1].parse::<u8>().unwrap(),
                                    colors[2].parse::<u8>().unwrap(),
                                    colors[3].parse::<f32>().unwrap(),
                                );
                                colors
                            };
                            background_color =
                                Color::from_rgba8(colors.0, colors.1, colors.2, colors.3);
                        }
                    }
                    wrapper.items.push(create_block(
                        background_color,
                        rendering_object.rectangle.clone(),
                    ));
                }
            };
        }

        let scrollable = Scrollable::new(scroll).push(url_search_bar).push(wrapper);

        Container::new(scrollable)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    UrlSearchBarTextInputChanged(String),
    UrlSearchBarSubmit(String),
}
