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
    alias: String,
    x: i32,
    y: i32
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
        let x = -1;
        let y = -1;

        Actor {
            label,
            alias,
            x,
            y
        }
    }

    pub fn get_actor_label(&self) -> &String {&self.label}

    pub fn get_actor_alias(&self) -> &String {&self.alias}

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }


    pub fn print(&self) {
        tracing::info!("Actor name: {:?} Actor alias: {:?}",
                 self.label, self.alias);
    }

    pub fn draw(&mut self, svg: &mut SVG, x: i32, y: i32, r: i32) {
        self.x = x; // middle of the head
        self.y = y; // middle of the head
        // r being the size - whole actor has (6r + text_size) height and 3r width
        // (or more if the name is longer).

        let line_weigth = 3; // thickness of lines
        let text_size = 30;

        let text_element = TextElement::new()
            .set("x", x)
            .set("y", y + 5 * r + text_size)
            .set("text-anchor", "middle")
            .set("dominant-baseline", "central")
            .set("fill", "black")
            .set("font-family", "Arial")
            .set("font-size", text_size.to_string())
            .add(Text::new(self.label.clone().as_str()));

        let circle = Circle::new()
            .set("cx", x.to_string())
            .set("cy", y.to_string())
            .set("r", r.to_string())
            .set("fill", "none")
            .set("stroke", "#000")
            .set("stroke-width", line_weigth.to_string());

        let body_line = Line::new()
            .set("x1", x.to_string())
            .set("y1", (y + r).to_string())
            .set("x2", x.to_string())
            .set("y2", (y + 3 * r).to_string())
            .set("stroke", "#000")
            .set("stroke-width", line_weigth.to_string());

        let left_arm = Line::new()
            .set("x1", ((x as f64) - 1.5 * (r as f64)).to_string())
            .set("y1", (y + r).to_string())
            .set("x2", x.to_string())
            .set("y2", (y + 2 * r).to_string())
            .set("stroke", "#000")
            .set("stroke-width", line_weigth.to_string());

        let right_arm = Line::new()
            .set("x1", ((x as f64) + 1.5 * (r as f64)).to_string())
            .set("y1", (y + r).to_string())
            .set("x2", x.to_string())
            .set("y2", (y + 2 * r).to_string())
            .set("stroke", "#000")
            .set("stroke-width", line_weigth.to_string());

        let left_leg = Line::new()
            .set("x1", ((x as f64) - 1.5 * (r as f64)).to_string())
            .set("y1", (y + 5 * r).to_string())
            .set("x2", x.to_string())
            .set("y2", (y + 3 * r).to_string())
            .set("stroke", "#000")
            .set("stroke-width", line_weigth.to_string());

        let right_leg = Line::new()
            .set("x1", ((x as f64) + 1.5 * (r as f64)).to_string())
            .set("y1", (y + 5 * r).to_string())
            .set("x2", x.to_string())
            .set("y2", (y + 3 * r).to_string())
            .set("stroke", "#000")
            .set("stroke-width", line_weigth.to_string());

        *svg = svg.clone().add(text_element);
        *svg = svg.clone().add(circle);
        *svg = svg.clone().add(body_line);
        *svg = svg.clone().add(left_arm);
        *svg = svg.clone().add(right_arm);
        *svg = svg.clone().add(left_leg);
        *svg = svg.clone().add(right_leg);
    }

}
