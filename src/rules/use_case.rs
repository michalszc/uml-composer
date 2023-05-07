use pest::iterators::Pair;
use crate::grammar_parser::Rule;

#[derive(Debug, PartialEq, Eq)]
pub struct UseCase {
    label: String,
    use_case_alias: String
}

impl UseCase {
    pub fn new(value: Pair<Rule>) -> UseCase {
        let mut inner = value.into_inner();
        let label = inner.next().unwrap().as_str().to_owned().replace("\"", "");
        println!("____{:?}",label);
        // inner.next(); // skip
        let use_case_alias = inner.next().unwrap().as_str().to_owned();

        UseCase {
            label,
            use_case_alias
        }
    }

    pub fn get_use_case_label(&self) -> &String {&self.label}

    pub fn get_use_case_alias(&self) -> &String {&self.use_case_alias}

    pub fn print(&self) {
        println!("Use Case label: {:?} Use Case alias: {:?}",
                 self.label, self.use_case_alias);
    }
}
