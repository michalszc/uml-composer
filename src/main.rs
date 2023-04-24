use std::fs;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct GrammarParser;

fn main() {
    let contents = fs::read_to_string("usecase.uml").unwrap();
    let contents_str = contents.as_str();
    let program = GrammarParser::parse(Rule::PROGRAM, contents_str)
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
                            for ii_pair in inner_pair.into_inner(){
                                println!("{:?}",ii_pair);
                            }
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
                            for ii_pair in inner_pair.into_inner(){
                                println!("{:?}",ii_pair);
                            }
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
