use std::fs;
use std::path::Path;
use uml_composer::uml_parser::UmlParser;

fn main() {
    let file_path = "./testWN.uml";
    let file_name = Path::new(file_path)
        .file_stem().unwrap()
        .to_str().unwrap()
        .to_string();
    let contents = fs::read_to_string(file_path).unwrap();
    let contents_str = contents.as_str();
    UmlParser::parse(contents_str, file_name);
}
