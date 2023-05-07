use pest::iterators::Pair;
use crate::grammar_parser::Rule;
use crate::rules::use_case::UseCase;

#[derive(Debug, PartialEq, Eq)]
pub struct Context {
    use_cases: Vec<UseCase>,
}

impl Context {
    pub fn new(value: Pair<Rule>) -> Context {
        let mut use_cases = Vec::new();
        let mut inner = value.into_inner();

        while let Some(use_case_pair) = inner.next() {
            let use_case = UseCase::new(use_case_pair);
            use_cases.push(use_case);
        }

        Context {
            use_cases,
        }
    }

    pub fn get_use_cases(&self) -> &Vec<UseCase> {
        &self.use_cases
    }

    pub fn print(&self) {
        println!("Context:");

        for use_case in &self.use_cases {
            use_case.print();
        }
    }
}