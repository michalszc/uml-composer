use pest::iterators::Pair;
use crate::grammar_parser::Rule;
use crate::rules::structs::Visibility::{PRIVATE, PROTECTED, PUBLIC};

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
    methods: Vec<Component>
}

impl Class {
    pub fn new(value: Pair<Rule>, interface: bool) -> Class {
        let keyword = if interface {"interface".parse().unwrap()}
        else {"class".parse().unwrap()};
        let mut attributes = Vec::new();
        let mut methods = Vec::new();
        let mut inner = value.into_inner();
        inner.next(); // skip keyword
        let name= inner.next().unwrap().as_str().to_owned();
        let body = inner.next().unwrap();
        for inner_pair in body.into_inner() {
            match inner_pair.as_rule() {
                Rule::ATTRIBUTES => {
                    let mut att = inner_pair.into_inner();
                    att.next(); // skip keyword
                    let component_list = att.next().unwrap().into_inner();
                    for ii_pair in component_list {
                        attributes.push(Component::extract_attribute(ii_pair));
                    }
                }
                Rule::METHODS => {
                    let mut met = inner_pair.into_inner();
                    met.next(); // skip keyword
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
            methods
        }
    }

    pub fn print(&self) {
        println!("{} {}\n\
        attributes:", self.keyword, self.name);
        for attribute in &self.attributes {
            attribute.print();
        }
        println!("methods:");
        for method in &self.methods {
            method.print();
        }
        println!();
    }
}

impl Component {
    fn extract_attribute(value: Pair<Rule>) -> Component {
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

    pub fn print(&self) {
        let v = match self.visibility {
            PRIVATE => "private",
            PUBLIC => "public",
            PROTECTED => "protected"
        };

        println!("\t{} {} {}", v, self.kind, self.name);
    }
}
