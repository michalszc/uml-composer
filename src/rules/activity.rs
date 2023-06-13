use pest::iterators::Pair;
use crate::grammar_parser::Rule;
use svg;
use std::collections::LinkedList;

enum Type {
    STEP,
    IF,
    START,
    END
}

pub struct Condition {
    main_path: Path,
    alternative_path: Path
}

impl Condition {
    pub fn new(if_st: Pair<Rule>, else_st: Pair<Rule>) -> Condition {
        let  main_path = Path::new(if_st);
        let alternative_path = Path::new(else_st);

        Condition {
            main_path,
            alternative_path
        }
    }

    pub fn print(&self) {
        println!("If {{");
        self.main_path.print();
        println!("}}");
        println!("else {{");
        self.alternative_path.print();
        println!("}}");
    }
}

pub struct Node {
    kind: Type,
    name: String,
    arrow_label: String
}

impl Node {
    pub fn new(value: Pair<Rule>) -> Node {
        let kind;
        let mut name;
        let mut arrow_label = "".to_string();

        match value.as_rule() {
            Rule::start_state => {
                kind = Type::START;
                name = "".to_string();
            }
            Rule::END_STATE => {
                kind = Type::END;
                name = "".to_string();
            }
            Rule::ACTIVITY => {
                kind = Type::STEP;
                let mut inner = value.into_inner();
                inner.next(); // skip arrow
                name = inner.next().unwrap().to_string();
            }
            _ => unreachable!()
        }

        Node {
            kind,
            name,
            arrow_label
        }
    }

    pub fn if_node(name: String) -> Node {
        Node {
            kind: Type::IF,
            name,
            arrow_label: "".to_string()
        }
    }

    pub fn print(&self) {
        println!("Activity")
    }
}

pub struct Path {
    nodes: LinkedList<Node>,
    alternatives: Vec<Condition>
}

impl Path {
    pub fn new(value: Pair<Rule>) -> Path {
        let mut nodes = LinkedList::new();
        let mut alternatives = Vec::new();

        for inner_pair in value.into_inner() {
            match inner_pair.as_rule() {
                Rule::END_STATE | Rule::ACTIVITY => {
                    nodes.push_back(Node::new(inner_pair))
                }
                Rule::IF => {
                    let mut inner = inner_pair.into_inner();
                    let mut if_st = inner.next().unwrap().into_inner();
                    let mut else_st = inner.next().unwrap().into_inner();

                    let name = if_st.next().unwrap().to_string();
                    nodes.push_back(Node::if_node(name));

                    let if_body = if_st.next().unwrap();

                    let else_body = else_st.next().unwrap();

                    alternatives.push(Condition::new(if_body, else_body));
                }
                _ => {}
            }
        }

        Path {
            nodes,
            alternatives
        }
    }

    pub fn print(&self) {
        let mut i: usize = 0;
        for node in self.nodes.iter() {
            match node.kind {
                Type::IF => {
                    self.alternatives[i].print();
                    i += 1;
                }
                Type::START => {
                    println!("Start")
                }
                Type::END => {
                    println!("Stop")
                }
                Type::STEP => {
                    node.print()
                }
            }
        }
    }
}

pub struct Activity {
    path: Path
}

impl Activity {
    pub fn new(value: Pair<Rule>) -> Activity {
        let mut inner = value.clone().into_inner();
        let start = inner.next().unwrap();
        let p_body =  inner.next().unwrap();

        let path = Path::new(p_body);

        return Activity{
            path
        }
    }

    pub fn print(&self) {
        self.path.print()
    }
}
