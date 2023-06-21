use pest::iterators::Pair;
use crate::grammar_parser::Rule;
use svg::node::{
    element::{
        SVG, Ellipse, Text as TextElement
    },
    Text
};

#[derive(Debug, PartialEq, Eq)]
pub struct UseCase {
    label: String,
    alias: String,
    x: i32,
    y: i32,
    width_number: i32,
    width: i32
}

impl UseCase {
    pub fn new(value: Pair<Rule>) -> UseCase {
        let mut inner = value.into_inner();
        let label;
        let alias;

        inner.next(); // skip 'use case'
        let l = inner.next().unwrap();

        match l.as_rule() {
            Rule::label => {
                label = l.as_str().replace("\"", "").to_owned();
                alias = String::from(l.as_str().replace("\"", "").to_owned());
            },
            Rule::ALIAS => {
                let mut inner2 = l.into_inner();
                label = inner2.next().unwrap().as_str().to_owned().replace("\"", "");

                inner2.next(); // skip 'as'
                alias = inner2.next().unwrap().as_str().to_owned().replace("\"", "");
            }
            _ => unreachable!()
        }
        let x = -1;
        let y = -1;
        let width_number = 1;
        let width = 100;

        UseCase {
            label,
            alias,
            x,
            y,
            width_number,
            width
        }
    }

    pub fn get_use_case_label(&self) -> &String {&self.label}

    pub fn get_use_case_alias(&self) -> &String {&self.alias}

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_width_number(&self) -> i32 {
        self.width_number
    }

    pub fn set_width_number(&mut self, width: i32) {
        self.width_number = width;
    }

    pub fn print(&self) {
        tracing::info!("Use Case label: {:?} Use Case alias: {:?} x: {} y: {} width_number: {}",
                 self.label, self.alias, self.x, self.y, self.width_number);
    }


    pub fn draw(&mut self, svg: &mut SVG, x: i32, y: i32, width: i32, height: i32) {
        self.x = x; // middle of the ellipse
        self.y = y; // middle of the ellipse
        let text_size = 20;
        let text_element = TextElement::new()
            .set("x", (x).to_string())
            .set("y", (y + text_size / 3).to_string())
            .set("text-anchor", "middle")
            .set("dominant-baseline", "central")
            .set("fill", "black")
            .set("font-family", "Arial")
            .set("font-size", text_size)
            .add(Text::new(self.label.clone().as_str()));

        let text_width = self.label.len() as f64 * (text_size as f64 * 0.6); // Adjust the multiplier as needed

        let ellipse_width = f64::max(width as f64, text_width + 20.0); // Add some padding
        self.width = ellipse_width as i32;

        let ellipse = Ellipse::new()
            .set("cx", (x).to_string())
            .set("cy", (y).to_string())
            .set("rx", (ellipse_width / 2.0).to_string()) // Adjusted width
            .set("ry", (height / 2).to_string())
            .set("fill", "blue") // Blue color
            .set("fill-opacity", "0.2") // Very transparent
            .set("stroke", "black")
            .set("stroke-width", "2");

        *svg = svg.clone().add(ellipse);
        *svg = svg.clone().add(text_element);
    }
}
