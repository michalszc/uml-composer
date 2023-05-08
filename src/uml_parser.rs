use pest::Parser;
use crate::rules::link::Link;
use crate::rules::structs::Class;
use crate::grammar_parser::{GrammarParser, Rule};
use svg::{Document, node::element::SVG};

pub struct UmlParser {

}

impl UmlParser {
    pub fn parse(value: &str, file_name: String) {
        let mut svg = SVG::new()
            .set("viewBox", "0 0 500 500");
        let program = GrammarParser::parse(Rule::PROGRAM, value)
            .unwrap_or_else(|e| panic!("{}", e))
            .next().unwrap();
        let mut x = 20;
        for pair in program.into_inner() {
            match pair.as_rule() {
                Rule::CLASS_DIAGRAM => {
                    for inner_pair in pair.into_inner(){
                        match inner_pair.as_rule() {
                            Rule::start_class => println!("{:?}", inner_pair),
                            Rule::CLASS => {
                                Class::new(inner_pair, false).print();
                            }
                            Rule::INTERFACE => {
                                Class::new(inner_pair, true).print();
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
                            Rule::ACTOR => {
                                for ii_pair in inner_pair.into_inner(){
                                    println!("{:?}",ii_pair);
                                }
                            }
                            Rule::CONTEXT => {
                                for ii_pair in inner_pair.into_inner(){
                                    println!("{:?}",ii_pair);
                                }
                            }
                            Rule::LINK => {
                                Link::new(inner_pair).print();
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
