use pest::iterators::Pair;
use crate::grammar_parser::Rule;
use crate::rules::structs::Visibility::{PRIVATE, PROTECTED, PUBLIC};
use svg;

#[derive(Debug, PartialEq, Eq)]
pub enum Visibility{
    PRIVATE,
    PROTECTED,
    PUBLIC
}

pub struct Component {
    name: String,
    visibility: Visibility,
    kind: String
}

pub struct Class {
    keyword: String,
    name: String,
    attributes: Vec<Component>,
    methods: Vec<Component>,
    x: usize,
    y: usize,
    height: usize,
    width: usize
}

fn add_text(text : svg::node::Text, i: usize, x: usize, y: usize) -> svg::node::element::Text {
    svg::node::element::Text::new()
        .set("x", x + 15)
        .set("y", y + 87 + i*50)
        .set("dominant-baseline", "central")
        .set("fill", "black")
        .set("font-size", 28)
        .add(text)
}

impl Class {
    pub fn new(value: Pair<Rule>, interface: bool) -> Class {
        let keyword = if interface {"interface".parse().unwrap()}
        else {"class".parse().unwrap()};
        let mut attributes = Vec::new();
        let mut methods = Vec::new();
        let mut inner = value.into_inner();
        let name= inner.next().unwrap().as_str().to_owned();
        let body = inner.next().unwrap();
        for inner_pair in body.into_inner() {
            match inner_pair.as_rule() {
                Rule::ATTRIBUTES => {
                    let mut att = inner_pair.into_inner();
                    let component_list = att.next().unwrap().into_inner();
                    for ii_pair in component_list {
                        attributes.push(Component::extract_attribute(ii_pair));
                    }
                }
                Rule::METHODS => {
                    let mut met = inner_pair.into_inner();
                    let component_list = met.next().unwrap().into_inner();
                    for ii_pair in component_list {
                        methods.push(Component::extract_attribute(ii_pair));
                    }
                }
                _ => unreachable!()
            }
        }

        Class {
            keyword,
            name,
            attributes,
            methods,
            x: 0,
            y: 0,
            height: 0,
            width: 0
        }
    }

    pub fn get_keyword(&self) -> &String {&self.keyword}
    pub fn get_name(&self) -> &String {&self.name}
    pub fn get_attributes(&self) -> &Vec<Component> {&self.attributes}
    pub fn get_methods(&self) -> &Vec<Component> {&self.methods}
    pub fn get_x(&self) -> &usize {&self.x}
    pub fn get_y(&self) -> &usize {&self.y}
    pub fn get_height(&self) -> &usize {&self.height}
    pub fn get_width(&self) -> &usize {&self.width}

    pub fn print(&self) {
        tracing::info!("{} {}\n\
        attributes:", self.keyword, self.name);
        for attribute in &self.attributes {
            attribute.print();
        }
        tracing::info!("methods:");
        for method in &self.methods {
            method.print();
        }
        tracing::info!("");
    }

    pub fn draw(&mut self, svg: &mut svg::node::element::SVG, x: usize, y: usize) {

        let additional = if self.keyword != "class".to_owned() { 50 } else { 0 };

        let name = svg::node::Text::new(self.name.as_str());

        let mut i:usize = 0;
        let mut texts = Vec::new();

        let mut width:usize = 0;

        for component in &self.attributes {
            let vis = match component.visibility {
                PRIVATE => "-",
                PROTECTED => "#",
                PUBLIC => "+"
            };
            let kind = if component.kind.is_empty() {"".to_owned()} else {
                " : ".to_owned() + component.kind.as_str()
            };
            let content = vis.to_string() + " " + component.get_name() + &kind;
            let new_width:usize = content.len()*15;
            if new_width > width {
                width = new_width;
            }
            let text = svg::node::Text::new(content);
            let attrib = add_text(text, i, x ,y + additional);
            texts.push(attrib);

            i += 1;
        }

        for method in &self.methods {
            let vis = match method.visibility {
                PRIVATE => "-",
                PROTECTED => "#",
                PUBLIC => "+"
            };
            let kind = if method.kind.is_empty() {"".to_owned()} else {
                " : ".to_owned() + method.kind.as_str()
            };
            let content = vis.to_string() + " " + method.get_name() + "()" + &kind;
            let new_width:usize = content.len()*15;
            if new_width > width {
                width = new_width;
            }
            let text = svg::node::Text::new(content);
            let meth = add_text(text, i, x, y + additional);
            texts.push(meth);
            i += 1;
        }

        let height = (i+1)*50+additional;
        let rect = svg::node::element::Rectangle::new()
            .set("x", x)
            .set("y", y)
            .set("width", width)
            .set("height", height)
            .set("fill", "white")
            .set("stroke", "black")
            .set("stroke-width", 10);
        *svg = svg.clone().add(rect);

        if self.keyword == "interface".to_owned() {
            let text = svg::node::Text::new("(interface)");
            let keyword = svg::node::element::Text::new()
                .set("x", x + 200)
                .set("y", y + 37)
                .set("text-anchor", "middle")
                .set("dominant-baseline", "central")
                .set("fill", "black")
                .set("font-size", 28)
                .add(text);
            *svg = svg.clone().add(keyword);
        }

        let name_label = svg::node::element::Text::new()
            .set("x", x + 200)
            .set("y", y + 37+additional)
            .set("text-anchor", "middle")
            .set("dominant-baseline", "central")
            .set("fill", "black")
            .set("font-size", 28)
            .add(name);
        *svg = svg.clone().add(name_label);

        let line = svg::node::element::Line::new()
            .set("x1", x)
            .set("y1", y + 50 + additional)
            .set("x2", x + width)
            .set("y2", y + 50 + additional)
            .set("stroke", "#000")
            .set("stroke-width", 5);
        *svg = svg.clone().add(line);

        let line_comp = svg::node::element::Line::new()
            .set("x1", x)
            .set("y1", y + (i-self.attributes.len()+1)*50 + additional)
            .set("x2", x + width)
            .set("y2", y + ((i-self.attributes.len())+1)*50 + additional)
            .set("stroke", "#000")
            .set("stroke-width", 5);
        *svg = svg.clone().add(line_comp);

        for attrib in texts {
            *svg = svg.clone().add(attrib);
        }

        self.x = x;
        self.y = y;
        self.height = height;
        self.width = width;
    }
}

impl Component {
    pub fn extract_attribute(value: Pair<Rule>) -> Component {
        let mut attribute = value.into_inner();
        let visibility: Visibility = match attribute.next().unwrap().as_str() {
            "-" => PRIVATE,
            "#" => PROTECTED,
            "+" => PUBLIC,
            _ => unreachable!()
        };
        let name: String = attribute.next().unwrap().as_str().to_owned();
        let colon = attribute.next();
        let mut kind: String = "".parse().unwrap();
        if !colon.is_none() {
            kind = attribute.next().unwrap().as_str().to_owned();
        }

        Component {
            name,
            visibility,
            kind
        }
    }

    pub fn get_name(&self) -> &String {&self.name}
    pub fn get_visibility(&self) -> &Visibility {&self.visibility}
    pub fn get_kind(&self) -> &String {&self.kind}

    pub fn print(&self) {
        let v = match self.visibility {
            PRIVATE => "private",
            PUBLIC => "public",
            PROTECTED => "protected"
        };

        tracing::info!("\t{} {} {}", v, self.kind, self.name);
    }
}
