use pest::Parser;
use crate::rules::link::Link;
use crate::rules::structs::Class;
use crate::grammar_parser::{GrammarParser, Rule};
use crate::rules::actor::Actor;
use crate::rules::context::Context;
use svg::{Document, node::element::SVG};

pub struct UmlParser {

}

impl UmlParser {
    pub fn parse(value: &str, file_name: String) {
        let mut svg = SVG::new()
            .set("viewBox", "0 0 1000 1000");
        let program = GrammarParser::parse(Rule::PROGRAM, value)
            .unwrap_or_else(|e| panic!("{}", e))
            .next().unwrap();
        let mut x = 100;
        let mut y = 100;
        for pair in program.into_inner() {
            match pair.as_rule() {
                Rule::CLASS_DIAGRAM => {
                    for inner_pair in pair.into_inner(){
                        match inner_pair.as_rule() {
                            Rule::start_class => println!("{:?}", inner_pair),
                            Rule::CLASS => {
                                Class::new(inner_pair, false).draw(&mut svg, 300, 300);
                            }
                            Rule::INTERFACE => {
                                Class::new(inner_pair, true).draw(&mut svg, 500, 500);
                            }
                            Rule::LINK => {
                                Link::new(inner_pair).draw(&mut svg, x, 20, x + 200, 200);
                                x += 80;
                            }
                            _ => unreachable!()
                        }
                    }
                }
                Rule::USE_CASE_DIAGRAM => {
                    for inner_pair in pair.into_inner(){
                        match inner_pair.as_rule() {
                            Rule::start_use_case => println!("{:?}", inner_pair),

                            Rule::CONTEXT => {
                                //Context::new(inner_pair).print();
                                Context::new(inner_pair).draw(&mut svg, 148, 100, 350, 350);
                            }

                            Rule::ACTOR => {
                                Actor::new(inner_pair).draw(&mut svg, x, y, 20);
                                y += 200;
                            }
                            Rule::LINK => {
                                Link::new(inner_pair).print();
                                //Link::new(inner_pair).draw(&mut svg, x, 20, x + 200, 200);
                                // x += 80;
                            }
                            _ => unreachable!()
                        }
                    }
                }
                Rule::end_uml => println!("{:?}", pair),
                _ => unreachable!()
            }
        }
        let document = Document::new().add(svg);
        svg::save(file_name + ".svg", &document).unwrap();
    }
}
