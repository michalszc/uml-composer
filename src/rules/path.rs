use std::cmp;
use crate::grammar_parser::Rule;
use activity_utils::{Type, draw_line};
use crate::rules::{activity_utils, node};
use pest::iterators::Pair;
use svg::node::element::SVG;
use node::Node;
use std::collections::LinkedList;

pub struct Condition {
    main_path: Path,
    alternative_path: Path
}

impl Condition {
    pub fn new(if_st: Pair<Rule>, else_st: Pair<Rule>) -> Condition {
        let main_path = Path::new(if_st, false);
        let alternative_path = Path::new(else_st, false);

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

    pub fn draw(&self, x: usize, y: usize, svg: &mut SVG, label: String) {
        let right_x = x+self.main_path.max_left()*250+250;
        let right_width = self.main_path.nodes.front().unwrap().get_name().len()*16;
        self.main_path.draw(right_x, y, svg);
        let arrow_label_r = self.main_path.nodes.front().unwrap().get_arrow_label().to_string();
        draw_line(x+20,y-12,right_x-right_width/2, y-12, svg, arrow_label_r);

        let left_x = x-self.alternative_path.max_right()*250-250;
        let left_width = self.alternative_path.nodes.front().unwrap().get_name().len()*16;
        self.alternative_path.draw(left_x, y, svg);
        let arrow_label_l = self.alternative_path.nodes.front().unwrap().get_arrow_label().to_string();
        draw_line(x-20, y-12, left_x+left_width/2, y-12, svg, arrow_label_l);

        self.bound_last_nodes(x, y, svg, label);
    }

    pub fn bound_last_nodes(&self, x2: usize, y: usize, svg: &mut SVG, label: String) {
        // find last node
        let mut x1 = x2+self.main_path.max_right()*250+250;
        let mut y1 = y+self.main_path.get_height()-110;
        let y2 = y+self.get_height()-25;

        if self.main_path.get_last_node_type() != Type::END {
            draw_line(x1, y1, x2, y2, svg, label.clone());
        }

        x1 = x2-self.alternative_path.max_right()*250-250;
        y1 = y+self.alternative_path.get_height()-110;
        if self.alternative_path.get_last_node_type() != Type::END {
            draw_line(x1, y1, x2, y2, svg, label);
        }
    }

    pub fn get_left_depth(&self) -> usize {
        return self.main_path.get_left_depth()
    }

    pub fn get_right_depth(&self) -> usize {
        return self.alternative_path.get_right_depth()
    }

    pub fn max_right(&self) -> usize {
        let mut n: usize = 1;

        n += self.main_path.max_right();

        return n;
    }

    pub fn max_left(&self) -> usize {
        let mut n: usize = 1;

        n += self.alternative_path.max_left();

        return n;
    }

    pub fn get_height(&self) -> usize {
        return cmp::max(self.main_path.get_height(), self.alternative_path.get_height())
    }

    pub fn nodes_count(&self) -> usize {
        return self.main_path.nodes_count()+self.alternative_path.nodes_count();
    }
}

pub struct Path {
    nodes: LinkedList<Node>,
    alternatives: Vec<Condition>
}

impl Path {
    pub fn new(value: Pair<Rule>, main: bool) -> Path {
        let mut nodes = LinkedList::new();
        let mut alternatives = Vec::new();

        if main {
            nodes.push_back(Node::start_node());
        }

        for inner_pair in value.into_inner() {
            match inner_pair.as_rule() {
                Rule::END_STATE | Rule::ACTIVITY => {
                    nodes.push_back(Node::new(inner_pair))
                }
                Rule::IF => {
                    let mut inner = inner_pair.into_inner();
                    let mut if_st = inner.next().unwrap().into_inner();
                    let mut else_st = inner.next().unwrap().into_inner();

                    let name = if_st.next().unwrap().as_str().to_owned();
                    nodes.push_back(Node::if_node(name));

                    let if_body = if_st.next().unwrap();

                    let else_body = else_st.next().unwrap();

                    alternatives.push(Condition::new(if_body, else_body));
                }
                _ => unreachable!()
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
            match node.get_kind() {
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

    pub fn draw(&self, x: usize, mut y: usize, svg: &mut SVG) {
        let mut i:usize = 0;
        let mut node_num: usize = 1;
        for node in self.nodes.iter() {
            if node_num != 1 && self.nodes.iter().nth(node_num-2).unwrap().get_kind() != Type::IF {
                let arrow_label = node.get_arrow_label().to_string();
                draw_line(x, y-110, x, y-20, svg, arrow_label);
            }
            node.draw(x, y, svg);
            if node.get_kind() == Type::IF {
                let label = self.nodes.iter().nth(node_num).unwrap().get_arrow_label().to_string();
                self.alternatives[i].draw(x, y, svg, label);
                y += self.alternatives[i].get_height();
                i += 1;
            } else if node.get_kind() == Type::STEP {
                y += 130;
            } else {
                y += 130;
            }
            node_num += 1;
        }
    }

    pub fn get_left_depth(&self) -> usize {
        let mut n: usize = 1;

        let mut max_depth: usize = 0;

        for alternative in &self.alternatives {
            if alternative.get_left_depth() > max_depth {
                max_depth = alternative.get_left_depth()
            }
        }

        n += max_depth;

        return n
    }

    pub fn get_right_depth(&self) -> usize {
        let mut n: usize = 1;

        let mut max_depth: usize = 0;

        for alternative in &self.alternatives {
            if alternative.get_right_depth() > max_depth {
                max_depth = alternative.get_right_depth()
            }
        }

        n += max_depth;

        return n
    }

    pub fn max_right(&self) -> usize {
        let mut n: usize = 0;

        for alternative in &self.alternatives {
            if alternative.max_right() > n {
                n = alternative.max_right();
            }
        }

        return n;
    }

    pub fn max_left(&self) -> usize {
        let mut n: usize = 0;

        for alternative in &self.alternatives {
            if alternative.max_left() > n {
                n = alternative.max_left();
            }
        }

        return n;
    }

    pub fn get_height(&self) -> usize {
        let mut n: usize = 0;

        for _node in self.nodes.iter() {
            n += 130;
        }

        for alternative in &self.alternatives {
            n += alternative.get_height();
        }

        n -= 130*self.alternatives.len();

        return n
    }

    pub fn get_last_node_type(&self) -> Type {
        return if self.nodes.back().unwrap().get_kind() == Type::IF {
            self.alternatives.last().unwrap().main_path.get_last_node_type()
        } else {
            self.nodes.back().unwrap().get_kind()
        }
    }

    pub fn nodes_count(&self) -> usize {
        let mut n = 0;
        n += self.nodes.len();

        for alternative in &self.alternatives {
            n += alternative.nodes_count()
        }

        n
    }
}
