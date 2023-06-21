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

    pub fn set_arrow(&mut self, arrow: ArrowType) { self.arrow = arrow; }

    pub fn set_link_type(&mut self, link_type: LinkType) { self.link_type = link_type; }

    pub fn print(&self) {
        tracing::info!("Left: {:?} Link: {:?} Right: {:?} Label: {:?} Arrow: {:?}",
            self.left_id, self.link_type, self.right_id, self.label, self.arrow);
    }

    pub fn draw(&self, svg: &mut SVG, x1: i32, y1: i32, x2: i32, y2: i32) {
        let line_weight = 3;
        let text_size = line_weight * 6;

        let mut line = self.draw_line(x1, y1, x2, y2, line_weight);
        
        if self.link_type == LinkType::DashedLine || self.link_type == LinkType::DashedArrow {
            line = line.set("stroke-dasharray", "8 8");
        }

        // Calculate the angle of the line
        let angle = ((y2 - y1) as f32 / (x2 - x1) as f32).atan();

        if self.link_type == LinkType::SolidArrow || self.link_type == LinkType::DashedArrow{
            let line_length = 30.0;
            let angle_offset:f32 = 7.0;

            let angle_left = angle + angle_offset.to_radians();
            let angle_right = angle - angle_offset.to_radians();

            let left_x1 = x2;
            let left_y1 = y2;
            let left_x2 = x2 - (line_length * angle_left.cos()) as i32;
            let left_y2 = y2 - (line_length * angle_left.sin()) as i32;

            let right_x1 = x2;
            let right_y1 = y2;
            let right_x2 = x2 - (line_length * angle_right.cos()) as i32;
            let right_y2 = y2 - (line_length * angle_right.sin()) as i32;

            let line_left = self.draw_line(left_x1, left_y1, left_x2, left_y2, line_weight);
            let line_right = self.draw_line(right_x1, right_y1, right_x2, right_y2, line_weight);
            let line_base = self.draw_line(left_x2, left_y2, right_x2, right_y2, line_weight);

            *svg = svg.clone().add(line_left);
            *svg = svg.clone().add(line_right);
            *svg = svg.clone().add(line_base);
        }


        if !self.label.is_empty() {
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
                .set("y", center_y - 5)
                .set("text-anchor", "middle")
                .set("dominant-baseline", "central")
                .set("fill", "black")
                .set("font-family", "Arial")
                .set("font-size", text_size)
                .set("transform", 
                    format!("rotate({} {} {})", angle.to_degrees(), center_x, center_y))
                .add(text);
            
            *svg = svg.clone().add(text_element);
        }

        *svg = svg.clone().add(line);
    }
    pub fn draw_class_link(&mut self, svg: &mut SVG, x1: i32, y1: i32, x2: i32, y2: i32, xs: i32) {
        let line_weight = 3; // thickness of the line

        if *self.get_link_type() == LinkType::DashedArrow {
            self.set_link_type(LinkType::DashedLine);
            self.draw(svg, xs, y1, xs, y2);
            self.set_link_type(LinkType::DashedArrow);
        }

        else if *self.get_link_type() == LinkType::SolidArrow {
            self.set_link_type(LinkType::SolidLine);
            self.draw(svg, xs, y1, xs, y2);
            self.set_link_type(LinkType::SolidArrow);
        }
        else {
            self.draw(svg, xs, y1, xs, y2); // middle line
        }

        let mut line1 = self.draw_line(x1, y1, xs, y1, line_weight);

        if self.link_type == LinkType::DashedLine || self.link_type == LinkType::DashedArrow {
            line1 = line1.set("stroke-dasharray", "8 8");
        }
        let mut line2 = self.draw_line(xs, y2, x2, y2, line_weight);

        if self.link_type == LinkType::DashedLine || self.link_type == LinkType::DashedArrow {
            line2 = line2.set("stroke-dasharray", "8 8");
        }
        let arrowhead = Marker::new()
            .set("id", "arrowhead")
            .set("markerWidth", "10")   // Adjust the width to make it smaller
            .set("markerHeight", "7")  // Adjust the height to make it smaller
            .set("refX", "0")
            .set("refY", "1.75")  // Adjust the reference point to center the arrowhead
            .set("orient", "auto")
            .add(
                Polygon::new()
                    .set("points", "-10 -5.25, 0 1.75, -10 8.75")  // Adjust the points to fit the new dimensions
            );

        if self.link_type == LinkType::SolidArrow || self.link_type == LinkType::DashedArrow{
            line2 = line2.set("marker-end", "url(#arrowhead)");
            let defs = Definitions::new().add(arrowhead);
            *svg = svg.clone().add(defs);
        }

        *svg = svg.clone().add(line1);
        *svg = svg.clone().add(line2);
    }

    pub fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32, line_weight: i32) -> Line {
        let line = Line::new()
            .set("x1", x1.to_string())
            .set("y1", y1.to_string())
            .set("x2", x2.to_string())
            .set("y2", y2.to_string())
            .set("stroke", "#000")
            .set("stroke-width", line_weight.to_string());

        return line;
    }

}
