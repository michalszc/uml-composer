use pest::Parser;
use crate::rules::link::Link;
use crate::grammar_parser::{GrammarParser, Rule};
use crate::rules::actor::Actor;
use crate::rules::context::Context;

pub struct UmlParser {

}

impl UmlParser {
    pub fn parse(value: &str) {
        let program = GrammarParser::parse(Rule::PROGRAM, value)
            .unwrap_or_else(|e| panic!("{}", e))
            .next().unwrap();
        for pair in program.into_inner() {
            match pair.as_rule() {
                Rule::CLASS_DIAGRAM => {
                    for inner_pair in pair.into_inner(){
                        match inner_pair.as_rule() {
                            Rule::start_class => println!("{:?}", inner_pair),
                            Rule::CLASS => {
                                for ii_pair in inner_pair.into_inner(){
                                    println!("{:?}",ii_pair);
                                }
                            }
                            Rule::INTERFACE => {
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
                Rule::USE_CASE_DIAGRAM => {
                    for inner_pair in pair.into_inner(){
                        match inner_pair.as_rule() {
                            Rule::start_use_case => println!("{:?}", inner_pair),
                            // Rule::ACTOR => {
                            //     for ii_pair in inner_pair.into_inner(){
                            //         println!("{:?}",ii_pair);
                            //     }
                            // }
                            Rule::ACTOR => {
                                Actor::new(inner_pair).print();
                            }
                            Rule::CONTEXT => {
                                Context::new(inner_pair).print();
                                // for ii_pair in inner_pair.into_inner(){
                                //     println!("{:?}",ii_pair);
                                // }

                                // Context::new(inner_pair).print();
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
    }
}
