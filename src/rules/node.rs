use crate::grammar_parser::Rule;
use activity_utils::Type;
use crate::rules::activity_utils;
use pest::iterators::Pair;
use svg::node::{
    element::{
        SVG, Rectangle, Text as TextElement, Circle
    },
    Text
};

pub struct Node {
    kind: Type,
    name: String,
    arrow_label: String
}

impl Node {
    pub fn new(value: Pair<Rule>) -> Node {
        let kind;
        let name;
        let mut arrow_label = "".to_string();

        match value.as_rule() {
            Rule::start_state => {
                kind = Type::START;
                name = "".to_string();
            }
            Rule::END_STATE => {
                kind = Type::END;
                name = "".to_string();
            }
            Rule::ACTIVITY => {
                kind = Type::STEP;
                let mut inner = value.into_inner();
                inner.next(); // skip arrow
                name = inner.next().unwrap().as_str().to_owned();
                if inner.next() != None {
                    arrow_label = inner.next().unwrap().as_str().to_owned();
                }
            }
            _ => unreachable!()
        }

        Node {
            kind,
            name,
            arrow_label
        }
    }

    pub fn if_node(name: String) -> Node {
        Node {
            kind: Type::IF,
            name,
            arrow_label: "".to_string()
        }
    }

    pub fn start_node() -> Node {
        Node {
            kind: Type::START,
            name: "".to_string(),
            arrow_label: "".to_string()
        }
    }

    pub fn print(&self) {
        println!("Activity")
    }

    pub fn draw(&self, x: usize, y: usize, svg: &mut SVG) {
        let width = self.name.len()*16;
        match self.kind {
            Type::IF => {
                let name = Text::new(self.name.as_str());

                let step = Rectangle::new()
                    .set("x", x-25)
                    .set("y", y-25)
                    .set("width", (50.0*std::f64::consts::FRAC_1_SQRT_2) as usize)
                    .set("height", (50.0*std::f64::consts::FRAC_1_SQRT_2) as usize)
                    .set("fill", "white")
                    .set("stroke", "black")
                    .set("stroke-width", 3)
                    .set("transform", format!("rotate({} {} {})", 45, x, y));
                *svg = svg.clone().add(step);

                let caption = TextElement::new()
                    .set("x", x)
                    .set("y", y+39)
                    .set("text-anchor", "middle")
                    .set("dominant-baseline", "central")
                    .set("fill", "black")
                    .set("font-size", 28)
                    .add(name);
                *svg = svg.clone().add(caption);
            }
            Type::STEP => {
                let name = Text::new(self.name.as_str());

                let step = Rectangle::new()
                    .set("x", x-width/2)
                    .set("y", y-32)
                    .set("width", width)
                    .set("height", 50)
                    .set("fill", "white")
                    .set("stroke", "black")
                    .set("stroke-width", 3)
                    .set("rx", 15);
                *svg = svg.clone().add(step);

                let caption = TextElement::new()
                    .set("x", x)
                    .set("y", y)
                    .set("text-anchor", "middle")
                    .set("dominant-baseline", "central")
                    .set("fill", "black")
                    .set("font-size", 28)
                    .add(name);
                *svg = svg.clone().add(caption);
            }
            Type::END => {
                let end = Circle::new()
                    .set("cx", x)
                    .set("cy", y)
                    .set("r", 25)
                    .set("stroke", "black")
                    .set("stroke-width", 2)
                    .set("fill", "none");
                *svg = svg.clone().add(end);

                let center = Circle::new()
                    .set("cx", x)
                    .set("cy", y)
                    .set("r", 20);
                *svg = svg.clone().add(center);
            }
            Type::START => {
                let start = Circle::new()
                    .set("cx", x)
                    .set("cy", y)
                    .set("r", 25);
                *svg = svg.clone().add(start);
            }
        }
    }

    pub fn get_kind(&self) -> Type {
        self.kind.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_arrow_label(&self) -> String {
        self.arrow_label.clone()
    }
}
