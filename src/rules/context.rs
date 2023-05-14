use pest::iterators::Pair;
use crate::grammar_parser::Rule;
use crate::rules::use_case::UseCase;
use svg::node::{
    element::{
        SVG, Rectangle, Text as TextElement
    },
    Text
};


#[derive(Debug, PartialEq, Eq)]
pub struct Context {
    label: String,
    use_cases: Vec<UseCase>
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

        Context {
            label,
            use_cases
        }
    }
    pub fn get_context_label(&self) -> &String {&self.label}

    pub fn get_use_cases(&self) -> &Vec<UseCase> {
        &self.use_cases
    }

    pub fn print(&self) {
        println!("Context name {}: ", self.label);
        for use_case in &self.use_cases {
            use_case.print();
        }
    }

    pub fn draw(&self, mut svg: &mut SVG, x: i32, y: i32, width: i32, height: i32) {
        let corner_radius = 10;

        let text_element = TextElement::new()
            .set("x", (x as f64 + 0.5 * (width as f64)).to_string())
            .set("y", y - 20)
            .set("text-anchor", "middle")
            .set("dominant-baseline", "central")
            .set("fill", "black")
            .set("font-size", 28)
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

        let added_y = height / self.use_cases.len() as i32;
        let uc_width = width as f64 / 3.0;
        let uc_height = f64::min(0.8 * (height as f64) / self.use_cases.len() as f64, 0.3 * height as f64);
        let mut mul = 0;
        for use_case in &self.use_cases{
            use_case.draw(
                &mut svg,
                (x as f64 - uc_width / 2.0 + width as f64 / 2.0) as i32,
                (y as f64 + uc_height / 8.0 + added_y as f64 * mul as f64) as i32,
                uc_width as i32,
                uc_height as i32,
            );
            mul += 1;
        }
    }
}
