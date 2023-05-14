use pest::iterators::Pair;
use crate::grammar_parser::Rule;
use svg::node::{
    element::{
        SVG, Line, Circle, Text as TextElement
    },
    Text
};

#[derive(Debug, PartialEq, Eq)]
pub struct Actor {
    // actor ?
    label: String,
    alias: String
}

impl Actor {
    pub fn new(value: Pair<Rule>) -> Actor {
        let mut inner = value.into_inner();
        let label;
        let alias;

        inner.next(); // skip 'actor'
        let l = inner.next().unwrap();

        match l.as_rule() {
            Rule::label => {
                label = l.as_str().to_owned();
                alias = String::from(l.as_str().to_owned());
            },
            Rule::ALIAS => {
                let mut inner2 = l.into_inner();
                label = inner2.next().unwrap().as_str().to_owned();

                inner2.next(); // skip 'as'
                alias = inner2.next().unwrap().as_str().to_owned();
            }
            _ => unreachable!()
        }

        Actor {
            label,
            alias
        }
    }

    pub fn get_actor_label(&self) -> &String {&self.label}

    pub fn get_actor_alias(&self) -> &String {&self.alias}

    pub fn print(&self) {
        println!("Actor name: {:?} Actor alias: {:?}",
                 self.label, self.alias);
    }

    pub fn draw(&self, svg: &mut SVG, x: i32, y: i32, r: i32) {
        // let r = 20; // size
        let width = 5;

        let text_element = TextElement::new()
            .set("x", x)
            .set("y", y - 40)
            .set("text-anchor", "middle")
            .set("dominant-baseline", "central")
            .set("fill", "black")
            .set("font-size", 28)
            .add(Text::new(self.label.clone().as_str()));

        let circle = Circle::new()
            .set("cx", x.to_string())
            .set("cy", y.to_string())
            .set("r", r.to_string())
            .set("fill", "none")
            .set("stroke", "#000")
            .set("stroke-width", width.to_string());

        let body_line = Line::new()
            .set("x1", x.to_string())
            .set("y1", (y + r).to_string())
            .set("x2", x.to_string())
            .set("y2", (y + 3 * r).to_string())
            .set("stroke", "#000")
            .set("stroke-width", width.to_string());

        let left_arm = Line::new()
            .set("x1", ((x as f64) - 1.5 * (r as f64)).to_string())
            .set("y1", (y + r).to_string())
            .set("x2", x.to_string())
            .set("y2", (y + 2 * r).to_string())
            .set("stroke", "#000")
            .set("stroke-width", width.to_string());

        let right_arm = Line::new()
            .set("x1", ((x as f64) + 1.5 * (r as f64)).to_string())
            .set("y1", (y + r).to_string())
            .set("x2", x.to_string())
            .set("y2", (y + 2 * r).to_string())
            .set("stroke", "#000")
            .set("stroke-width", width.to_string());

        let left_leg = Line::new()
            .set("x1", ((x as f64) - 1.5 * (r as f64)).to_string())
            .set("y1", (y + 5 * r).to_string())
            .set("x2", x.to_string())
            .set("y2", (y + 3 * r).to_string())
            .set("stroke", "#000")
            .set("stroke-width", width.to_string());

        let right_leg = Line::new()
            .set("x1", ((x as f64) + 1.5 * (r as f64)).to_string())
            .set("y1", (y + 5 * r).to_string())
            .set("x2", x.to_string())
            .set("y2", (y + 3 * r).to_string())
            .set("stroke", "#000")
            .set("stroke-width", width.to_string());

        *svg = svg.clone().add(text_element);
        *svg = svg.clone().add(circle);
        *svg = svg.clone().add(body_line);
        *svg = svg.clone().add(left_arm);
        *svg = svg.clone().add(right_arm);
        *svg = svg.clone().add(left_leg);
        *svg = svg.clone().add(right_leg);
    }

}
