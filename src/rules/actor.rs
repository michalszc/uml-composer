use pest::iterators::Pair;
use crate::grammar_parser::Rule;

#[derive(Debug, PartialEq, Eq)]
pub struct Actor {
    // actor ?
    actor_id: String,
    actor_alias: String
}

impl Actor {
    pub fn new(value: Pair<Rule>) -> Actor {
        let mut inner = value.into_inner();
        println!("____{:?}",inner.as_str());

        let actor_id = inner.next().unwrap().as_str().trim().to_owned();

        let actor_alias = inner.next().unwrap().as_str().trim().to_owned();

        Actor {
            actor_id,
            actor_alias
        }
    }

    pub fn get_actor_id(&self) -> &String {&self.actor_id}

    pub fn get_actor_alias(&self) -> &String {&self.actor_alias}

    pub fn print(&self) {
        println!("Actor name: {:?} Actor alias: {:?}",
                 self.actor_id, self.actor_alias);
    }
}
