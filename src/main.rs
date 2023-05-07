use std::fs;
use uml_composer::uml_parser::UmlParser;

fn main() {
    let contents = fs::read_to_string("testWN.uml").unwrap();
    let contents_str = contents.as_str();
    UmlParser::parse(contents_str);
}
