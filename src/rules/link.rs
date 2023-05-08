use pest::iterators::Pair;
use crate::grammar_parser::Rule;
use svg::node::{
    element::{
        SVG, Definitions, Line, Marker, Polygon, Text as TextElement
    }, 
    Text
};

#[derive(Debug, PartialEq, Eq)]
pub enum ArrowType {
    Left,
    Right,
    Missing
}

#[derive(Debug, PartialEq, Eq)]
pub enum LinkType {
    SolidLine,
    DashedLine,
    SolidArrow,
    DashedArrow
}

pub struct Link {
    left_id: String,
    right_id: String,
    link_type: LinkType,
    label: String,
    arrow: ArrowType
}

impl Link {
    pub fn new(value: Pair<Rule>) -> Link {
        let mut inner = value.into_inner();
        let left_id = inner.next().unwrap().as_str().to_owned();
  
        let link_rule = inner.next().unwrap().as_rule();
        let link_type: LinkType = match link_rule {
            Rule::solid_line => LinkType::SolidLine,
            Rule::dashed_line => LinkType::DashedLine,
            Rule::solid_arrow => LinkType::SolidArrow,
            Rule::dashed_arrow => LinkType::DashedArrow,
            _ => unreachable!()
        };

        let right_id = inner.next().unwrap().as_str().to_owned();

        inner.next(); // skip colon

        let label = match inner.next() {
            Some(l) => {
                l.as_str().to_owned().replace("\"", "")
            }
            None => String::new()
        };

        let arrow: ArrowType = match inner.next() {
            Some(a) => {
                match a.as_rule() {
                    Rule::left_arrow => ArrowType::Left,
                    Rule::right_arrow => ArrowType::Right,
                    _ => unreachable!()
                }
            }
            None => ArrowType::Missing
        };

        Link {
            left_id,
            right_id,
            link_type,
            label,
            arrow
        }
    }
    
    pub fn get_left_id(&self) -> &String {
        &self.left_id
    }

    pub fn get_right_id(&self) -> &String {
        &self.right_id
    }

    pub fn get_link_type(&self) -> &LinkType {
        &self.link_type
    }

    pub fn get_label(&self) -> &String {
        &self.label
    }

    pub fn get_arrow(&self) -> &ArrowType {
        &self.arrow
    }

    pub fn print(&self) {
        println!("Left: {:?} Link: {:?} Right: {:?} Label: {:?} Arrow: {:?}",
            self.left_id, self.link_type, self.right_id, self.label, self.arrow);
    }

    pub fn draw(&self, svg: &mut SVG, x1: i32, y1: i32, x2: i32, y2: i32) {
        let arrowhead = Marker::new()
            .set("id", "arrowhead")
            .set("markerWidth", "10")
            .set("markerHeight", "7")
            .set("refX", "0")
            .set("refY", "3.5")
            .set("orient", "auto")
            .add(
                Polygon::new()
                    .set("points", "0 0, 10 3.5, 0 7")
            );

        let mut line = Line::new()
            .set("x1", x1.to_string())
            .set("y1", y1.to_string())
            .set("x2", x2.to_string())
            .set("y2", y2.to_string())
            .set("stroke", "#000")
            .set("stroke-width", "8");
        
        if self.link_type == LinkType::DashedLine || self.link_type == LinkType::DashedArrow {
            line = line.set("stroke-dasharray", "8 8");
        }

        if self.link_type == LinkType::SolidArrow || self.link_type == LinkType::DashedArrow {
            line = line.set("marker-end", "url(#arrowhead)");
            let defs = Definitions::new().add(arrowhead);
            *svg = svg.clone().add(defs);
        }

        if !self.label.is_empty() {
            // Calculate the angle of the line
            let angle = ((y2 - y1) as f32 / (x2 - x1) as f32).atan();

            // Calculate the center point of the line
            let center_x = (x1 + x2) / 2;
            let center_y = (y1 + y2) / 2;

            let text = match self.arrow {
                ArrowType::Left => {
                    Text::new((self.label.clone() + "◀").as_str())
                },
                ArrowType::Right => {
                    Text::new((self.label.clone() + "▶").as_str())
                },
                ArrowType::Missing => Text::new(self.label.as_str())
            };

            // Create a text element
            let text_element = TextElement::new()
                .set("x", center_x)
                .set("y", center_y - 20)
                .set("text-anchor", "middle")
                .set("dominant-baseline", "central")
                .set("fill", "black")
                .set("font-size", 28)
                .set("transform", 
                    format!("rotate({} {} {})", angle.to_degrees(), center_x, center_y))
                .add(text);
            
            *svg = svg.clone().add(text_element);
        }

        *svg = svg.clone().add(line);
    }
}
