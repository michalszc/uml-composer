use pest::iterators::Pair;
use crate::grammar_parser::Rule;
use crate::rules::use_case::UseCase;
use std::collections::HashMap;
use svg::node::{
    element::{
        SVG, Rectangle, Text as TextElement
    },
    Text
};


#[derive(Debug, PartialEq, Eq)]
pub struct Context {
    label: String,
    use_cases: Vec<UseCase>,
    width_number: i32
}

impl Context {
    pub fn new(value: Pair<Rule>) -> Context {
        let mut use_cases = Vec::new();
        let mut inner = value.into_inner();

        inner.next(); // skip 'context'
        let label = inner.next().unwrap().as_str().trim().to_owned();

        for use_case in inner.next().unwrap().into_inner(){
            use_cases.push(UseCase::new(use_case));
        }
        let width_number = 1;

        Context {
            label,
            use_cases,
            width_number
        }
    }
    pub fn get_context_label(&self) -> &String {&self.label}

    pub fn get_use_cases(&self) -> &Vec<UseCase> {
        &self.use_cases
    }

    pub fn get_use_cases_mut(&mut self) -> &mut Vec<UseCase> {
        &mut self.use_cases
    }

    pub fn print(&self) {
        println!("Context name {}. Width: {}. Use cases: ", self.label, self.width_number);
        for use_case in &self.use_cases {
            use_case.print();
        }
    }

    pub fn get_width_number(&self) -> i32 {
        self.width_number
    }

    pub fn set_width_number(&mut self, width: i32) {
        self.width_number = width;
    }

    pub fn draw(&mut self, mut svg: &mut SVG, x: i32, y: i32, width: i32, height: i32) {
        let corner_radius = 10;
        let text_size = 20;
        let mut heights: HashMap<i32, i32> = (1..=self.width_number).map(|key| (key, 1)).collect();
        let mut max_heights: HashMap<i32, i32> = (1..=self.width_number).map(|key| (key, 0)).collect();

        let text_element = TextElement::new()
            .set("x", (x as f64 + 0.5 * (width as f64)).to_string())
            .set("y", y - text_size / 2)
            .set("text-anchor", "middle")
            .set("dominant-baseline", "central")
            .set("fill", "black")
            .set("font-family", "Arial")
            .set("font-size", text_size)
            .add(Text::new(self.label.clone().as_str()));

        let rectangle = Rectangle::new()
            .set("x", x)
            .set("y", y)
            .set("width", width)
            .set("height", height)
            .set("rx", corner_radius)
            .set("ry", corner_radius)
            .set("fill", "transparent")
            .set("fill-opacity", "0.3")
            .set("stroke", "gray")
            .set("stroke-width", "2");

        *svg = svg.clone().add(text_element);
        *svg = svg.clone().add(rectangle);

        let uc_width = 100;
        let uc_height = f64::min(0.8 * (height as f64) / self.use_cases.len() as f64, 50.0);
        let mut y_in_column = y;
        let _use_cases_length = self.get_use_cases().len();

        for use_case in &mut self.use_cases {
            if let Some(value) = max_heights.get_mut(&use_case.get_width_number()) {
                *value += 1;
            }
        }

        for use_case in &mut self.use_cases {
            if let Some(value) = heights.get(&use_case.get_width_number()) {
                if let Some(value2) = max_heights.get(&use_case.get_width_number()) {
                    y_in_column = y + (*value as i32) * std::cmp::min(100, height / (value2 + 1) as i32);
                }
            }
            if let Some(value) = heights.get_mut(&use_case.get_width_number()) {
                *value += 1;
            }
            use_case.draw(
                &mut svg,
                (x + 350 / 2 + (use_case.get_width_number() - 1) * 350) as i32,
                (y_in_column) as i32,
                uc_width as i32,
                uc_height as i32,
            );
        }
    }
}
