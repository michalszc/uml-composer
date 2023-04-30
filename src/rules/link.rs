use pest::iterators::Pair;
use crate::grammar_parser::Rule;

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
            None => String::from("")
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
}
