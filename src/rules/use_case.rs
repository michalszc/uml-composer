use pest::iterators::Pair;
use crate::grammar_parser::Rule;

#[derive(Debug, PartialEq, Eq)]
pub struct UseCase {
    label: String,
    alias: String
}

impl UseCase {
    pub fn new(value: Pair<Rule>) -> UseCase {
        let mut inner = value.into_inner();
        let label;
        let alias;

        inner.next(); // skip 'usecase'
        let l = inner.next().unwrap();

        match l.as_rule() {
            Rule::label => {
                label = l.as_str().replace("\"", "").to_owned();
                alias = String::from(l.as_str().replace("\"", "").to_owned());
            },
            Rule::ALIAS => {
                let mut inner2 = l.into_inner();
                label = inner2.next().unwrap().as_str().to_owned().replace("\"", "");

                inner2.next(); // skip 'as'
                alias = inner2.next().unwrap().as_str().to_owned().replace("\"", "");
            }
            _ => unreachable!()
        }

        UseCase {
            label,
            alias
        }
    }

    pub fn get_use_case_label(&self) -> &String {&self.label}

    pub fn get_use_case_alias(&self) -> &String {&self.alias}

    pub fn print(&self) {
        println!("Use Case label: {:?} Use Case alias: {:?}",
                 self.label, self.alias);
    }
}
