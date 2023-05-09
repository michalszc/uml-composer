use pest::iterators::Pair;
use crate::grammar_parser::Rule;

#[derive(Debug, PartialEq, Eq)]
pub struct Actor {
    // actor ?
    label: String,
    alias: String
}

impl Actor {
    pub fn new(value: Pair<Rule>) -> Actor {
        let mut inner = value.into_inner();
        let label;
        let alias;

        inner.next(); // skip 'actor'
        let l = inner.next().unwrap();

        match l.as_rule() {
            Rule::label => {
                label = l.as_str().to_owned();
                alias = String::from(l.as_str().to_owned());
            },
            Rule::ALIAS => {
                let mut inner2 = l.into_inner();
                label = inner2.next().unwrap().as_str().to_owned();

                inner2.next(); // skip 'as'
                alias = inner2.next().unwrap().as_str().to_owned();
            }
            _ => unreachable!()
        }

        Actor {
            label,
            alias
        }
    }

    pub fn get_actor_label(&self) -> &String {&self.label}

    pub fn get_actor_alias(&self) -> &String {&self.alias}

    pub fn print(&self) {
        println!("Actor name: {:?} Actor alias: {:?}",
                 self.label, self.alias);
    }
}
