use crate::css::cssom::cssom::StylingRule;
use crate::html::dom::dom::{DOMNode, ElementType, NodeType};
use crate::html::dom::elements::elements::HTMLElements;
use crate::paint::font::PaintFont;
use crate::render_tree::pt::fix_unit_to_px;
use crate::render_tree::rectangle::Rectangle;

#[derive(Debug, PartialEq, Clone)]
pub struct _RenderObject {
    pub children: Vec<RenderObject>,
    pub style: Vec<StylingRule>,
    pub rectangle: Rectangle,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TextRenderObject {
    pub text: String,
    pub rectangle: Rectangle,
    pub font: PaintFont,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RenderObject {
    ViewPort(_RenderObject),
    Scroll(_RenderObject),
    Block(_RenderObject),
    Inline(_RenderObject),
    Text(TextRenderObject),
}

impl RenderObject {
    pub fn new() -> Self {
        Self::ViewPort(_RenderObject {
            children: vec![],
            style: vec![],
            rectangle: Rectangle::new(0.0, 0.0, 0.0, 0.0),
        })
    }

    pub fn layouting_node(&mut self, parent_node: Self, big_brother_node: Option<Self>) {
        let big_brother_node = match big_brother_node {
            Some(big_brother_node_) => Some(big_brother_node_),
            None => None,
        };

        let parent_rectangle = match parent_node {
            Self::Text(_) => panic!("TODO"),
            Self::Scroll(parent_node)
            | Self::ViewPort(parent_node)
            | Self::Block(parent_node)
            | Self::Inline(parent_node) => parent_node.rectangle,
        };

        let big_brother_rectangle = match big_brother_node {
            None => None,
            Some(big_brother_node_) => match big_brother_node_ {
                Self::Text(_) => panic!("TODO"),
                Self::Scroll(big_brother)
                | Self::ViewPort(big_brother)
                | Self::Inline(big_brother)
                | Self::Block(big_brother) => Some(big_brother.rectangle),
            },
        };

        self.calc_rectangle(&parent_rectangle, &big_brother_rectangle);

        let parent = self.clone();

        match self {
            Self::Text(_) => return,
            Self::Block(rendering_object)
            | Self::Inline(rendering_object)
            | Self::Scroll(rendering_object)
            | Self::ViewPort(rendering_object) => {
                let mut big_brother_node: Option<Self> = None;

                let mut i = 0;

                while i < rendering_object.children.len() {
                    let child = rendering_object.children.get_mut(i).unwrap();
                    child.layouting_node(parent.clone(), big_brother_node);
                    println!("child: {:?}", child);
                    println!("---------");
                    big_brother_node = Some(child.clone());
                    i += 1;
                }
            }
        }
    }

    pub fn prepare_iterator(&self, iterator: &mut Vec<Self>) {
        iterator.push(self.clone());

        let rendering_object = match self {
            Self::Text(_) => {
                return;
            }
            Self::Block(rendering_object)
            | Self::Inline(rendering_object)
            | Self::Scroll(rendering_object)
            | Self::ViewPort(rendering_object) => rendering_object,
        };

        if rendering_object.children.len() > 0 {
            for child in &rendering_object.children {
                child.prepare_iterator(iterator);
            }
        }
    }

    // TODO position absoluteの時など, big_brotherがparentに入らなさそうな時
    // TODO font size == 高さと見做してするけど, のちになんとかした方が良さそう
    pub fn calc_rectangle(
        &mut self,
        parent_rect: &Rectangle,
        big_brother_rect: &Option<Rectangle>,
    ) {
        println!("rect: {:#?}", parent_rect);
        let width = self.calc_width(&parent_rect.width);
        let height = self.calc_height(&parent_rect.height, &width);

        let rendering_object = match self {
            Self::Text(text_render_object) => {
                text_render_object.rectangle =
                    Rectangle::new(parent_rect.x, parent_rect.y, width, height);
                return;
            }
            Self::Block(rendering_object)
            | Self::Inline(rendering_object)
            | Self::Scroll(rendering_object)
            | Self::ViewPort(rendering_object) => rendering_object,
        };

        // TODO
        rendering_object.rectangle = Rectangle::new(0.0, 0.0, width, height);

        let x = self.calc_x(&parent_rect, &big_brother_rect);
        let y = self.calc_y(&parent_rect, &big_brother_rect);

        let rendering_object = match self {
            Self::Text(_) => {
                return;
            }
            Self::Block(rendering_object)
            | Self::Inline(rendering_object)
            | Self::Scroll(rendering_object)
            | Self::ViewPort(rendering_object) => rendering_object,
        };

        rendering_object.rectangle = Rectangle::new(x, y, width, height);
    }

    fn calc_width(&self, parent_width: &f32) -> f32 {
        let rendering_object = match self {
            // TODO
            Self::Text(_) => {
                return parent_width.clone();
            }
            Self::Block(rendering_object)
            | Self::Inline(rendering_object)
            | Self::Scroll(rendering_object)
            | Self::ViewPort(rendering_object) => rendering_object,
        };

        for style in rendering_object.clone().style {
            if style.declarations.get(&"width".to_string()).is_some() {
                let raw_width = style.declarations.get(&"width".to_string()).unwrap();

                let raw_width = fix_unit_to_px(raw_width.to_string());

                match raw_width {
                    Some(width) => {
                        return width;
                    }
                    None => {
                        panic!("TODO");
                    }
                }
            }
        }

        let width = match self {
            // TODO
            Self::Text(_) => {
                return 0.0;
            }
            Self::Block(_) | Self::Inline(_) | Self::Scroll(_) | Self::ViewPort(_) => parent_width,
        };

        width.clone()
    }

    fn calc_height(&self, _parent_height: &f32, parent_width: &f32) -> f32 {
        println!("www: {:?}", parent_width);
        let rendering_object = match self {
            // TODO
            Self::Text(text) => {
                return text
                    .font
                    .get_font_rendered_size(parent_width.clone(), text.text.clone())
                    .height as f32
            }
            Self::Block(rendering_object)
            | Self::Inline(rendering_object)
            | Self::Scroll(rendering_object)
            | Self::ViewPort(rendering_object) => rendering_object,
        };

        for style in rendering_object.clone().style {
            if style.declarations.get(&"height".to_string()).is_some() {
                let raw_height = style
                    .declarations
                    .get(&"height".to_string())
                    .unwrap()
                    .parse::<f32>();

                match raw_height {
                    Ok(height) => {
                        return height;
                    }
                    Err(e) => {
                        panic!("{:?}", e);
                    }
                }
            }
        }

        let height = match self {
            Self::Text(_) => {
                return 0.0;
            }
            Self::Block(rendering_object)
            | Self::Inline(rendering_object)
            | Self::Scroll(rendering_object)
            | Self::ViewPort(rendering_object) => {
                let mut height = 0.0;
                for child in rendering_object.clone().children {
                    height += child.calc_height(&rendering_object.rectangle.height, &parent_width);
                }
                height
            }
        };

        height
    }

    fn calc_x(&self, parent_rect: &Rectangle, _big_brother_rect: &Option<Rectangle>) -> f32 {
        let x = match self {
            // TODO
            Self::Text(_) => parent_rect.x,
            Self::Block(_) | Self::Inline(_) | Self::Scroll(_) | Self::ViewPort(_) => parent_rect.x,
        };

        x
    }

    fn calc_y(&self, parent_rect: &Rectangle, big_brother_rect: &Option<Rectangle>) -> f32 {
        let big_brother_rect = match big_brother_rect {
            Some(big_brother_rect) => big_brother_rect,
            None => {
                return parent_rect.y;
            }
        };

        let y = match self {
            Self::Text(_) => parent_rect.y,
            Self::Block(_) | Self::Inline(_) | Self::Scroll(_) | Self::ViewPort(_) => {
                big_brother_rect.y + big_brother_rect.height
            }
        };

        y
    }

    pub fn init_with_text(
        txt: String,
        rectangle: Option<Rectangle>,
        font: Option<PaintFont>,
    ) -> Self {
        let rectangle = rectangle.unwrap_or(Rectangle {
            x: 0.0,
            y: 45.0,
            width: 700.0,
            height: 700.0,
        });

        let font = font.unwrap_or(PaintFont::new(None, None));

        Self::Text(TextRenderObject {
            text: txt,
            rectangle,
            font,
        })
    }

    pub fn init_with_element(element_type: ElementType) -> Option<Self> {
        match element_type.tag_name {
            HTMLElements::BodyElement => Some(Self::Scroll(_RenderObject {
                children: vec![],
                style: vec![],
                rectangle: Rectangle::new(0.0, 0.0, 0.0, 0.0),
            })),
            HTMLElements::DivElement | HTMLElements::ParagraphElement | HTMLElements::H1Element => {
                Some(Self::Block(_RenderObject {
                    children: vec![],
                    style: vec![],
                    rectangle: Rectangle::new(0.0, 0.0, 0.0, 0.0),
                }))
            }
            HTMLElements::AnchorElement | HTMLElements::SpanElement => {
                Some(Self::Inline(_RenderObject {
                    children: vec![],
                    style: vec![],
                    rectangle: Rectangle::new(0.0, 0.0, 0.0, 0.0),
                }))
            }
            _ => None,
        }
    }

    pub fn can_init_element(dom_node: &DOMNode) -> bool {
        let element_type = match &dom_node.node_type {
            NodeType::TextNode(_) => return false,
            NodeType::DomNode(element_type) => element_type,
        };
        let tag = &element_type.tag_name;
        tag == &HTMLElements::BodyElement
            || tag == &HTMLElements::DivElement
            || tag == &HTMLElements::ParagraphElement
            || tag == &HTMLElements::AnchorElement
            || tag == &HTMLElements::SpanElement
            || tag == &HTMLElements::H1Element
    }

    pub fn can_init_text(dom_node: &DOMNode) -> bool {
        match &dom_node.node_type {
            NodeType::TextNode(_) => true,
            NodeType::DomNode(_) => false,
        }
    }

    pub fn push_child(&mut self, child: RenderObject) {
        match self {
            Self::Text(_) => {
                panic!("RenderObject::push_shild should not be called with text")
            }
            Self::ViewPort(render_object)
            | Self::Scroll(render_object)
            | Self::Inline(render_object)
            | Self::Block(render_object) => render_object.children.push(child),
        };
    }

    pub fn replace_style(&mut self, rules: Vec<StylingRule>) {
        match self {
            Self::Text(_) => {
                panic!("RenderObject::replace_style should not be called with text")
            }
            Self::ViewPort(render_object)
            | Self::Scroll(render_object)
            | Self::Inline(render_object)
            | Self::Block(render_object) => render_object.style = rules,
        };
    }
}
