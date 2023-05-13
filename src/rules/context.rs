use pest::iterators::Pair;
use crate::grammar_parser::Rule;
use crate::rules::use_case::UseCase;

#[derive(Debug, PartialEq, Eq)]
pub struct Context {
    label: String,
    use_cases: Vec<UseCase>
}

impl Context {
    pub fn new(value: Pair<Rule>) -> Context {
        let mut use_cases = Vec::new();
        let mut inner = value.into_inner();

        inner.next(); // skip 'context'
        let label = inner.next().unwrap().as_str().trim().to_owned();

        for use_case in inner.next().unwrap().into_inner(){
            use_cases.push(UseCase::new(use_case));
        }

        Context {
            label,
            use_cases
        }
    }
    pub fn get_context_label(&self) -> &String {&self.label}

    pub fn get_use_cases(&self) -> &Vec<UseCase> {
        &self.use_cases
    }

    pub fn print(&self) {
        println!("Context name {}: ", self.label);
        for use_case in &self.use_cases {
            use_case.print();
        }
    }
}
