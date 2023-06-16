use std::cmp;
use pest::iterators::Pair;
use crate::grammar_parser::Rule;
use svg;
use std::collections::LinkedList;
use svg::node::{
    element::{
        SVG, Definitions, Line, Marker, Polygon, Text as TextElement
    },
    Text
};

fn draw_line(x1: usize, y1: usize, x2: usize, y2: usize, svg: &mut SVG) {
    let arrowhead = Marker::new()
        .set("id", "arrowhead")
        .set("markerWidth", "5")
        .set("markerHeight", "5")
        .set("refX", "0")
        .set("refY", "3.5")
        .set("orient", "auto")
        .add(
            Polygon::new()
                .set("points", "0 1.5, 5 3.5, 0 5.5")
        );

    let mut line = Line::new()
        .set("x1", x1)
        .set("y1", y1)
        .set("x2", x2)
        .set("y2", y2)
        .set("stroke", "#000")
        .set("stroke-width", 8);

    line = line.set("marker-end", "url(#arrowhead)");
    let defs = Definitions::new().add(arrowhead);
    *svg = svg.clone().add(defs);
    *svg = svg.clone().add(line);
}

#[derive(PartialEq)]
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
        let main_path = Path::new(if_st);
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

    pub fn draw(&self, x: usize, y: usize, svg: &mut SVG) {
        let right_x = x+self.main_path.max_left()*250+250;
        let right_width = self.main_path.nodes.front().unwrap().name.len()*16;
        self.main_path.draw(right_x, y, svg);
        draw_line(x+20,y-12,right_x-right_width-5, y-12, svg);

        let left_x = x-self.alternative_path.max_right()*250-250;
        let left_width = self.alternative_path.nodes.front().unwrap().name.len()*16;
        self.alternative_path.draw(left_x, y, svg);
        draw_line(x-20, y-12, left_x+left_width-25, y-12, svg);
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
}

pub struct Node {
    kind: Type,
    name: String,
    arrow_label: String
}

impl Node {
    pub fn new(value: Pair<Rule>) -> Node {
        let kind;
        let name;
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
                name = inner.next().unwrap().as_str().to_owned()
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

    pub fn draw(&self, x: usize, y: usize, svg: &mut SVG) {
        let width = self.name.len()*16;
        match self.kind {
            Type::IF => {
                let name = Text::new(self.name.as_str());

                let step = svg::node::element::Rectangle::new()
                    .set("x", x-25)
                    .set("y", y-25)
                    .set("width", (50.0*std::f64::consts::FRAC_1_SQRT_2) as usize)
                    .set("height", (50.0*std::f64::consts::FRAC_1_SQRT_2) as usize)
                    .set("fill", "white")
                    .set("stroke", "black")
                    .set("stroke-width", 3)
                    .set("transform", format!("rotate({} {} {})", 45, x, y));
                *svg = svg.clone().add(step);

                let caption = svg::node::element::Text::new()
                    .set("x", x-width/2-8)
                    .set("y", y-39)
                    .set("text-anchor", "middle")
                    .set("dominant-baseline", "central")
                    .set("fill", "black")
                    .set("font-size", 28)
                    .add(name);
                *svg = svg.clone().add(caption);
            }
            Type::STEP => {
                let name = Text::new(self.name.as_str());

                let step = svg::node::element::Rectangle::new()
                    .set("x", x-width/2)
                    .set("y", y-32)
                    .set("width", width)
                    .set("height", 50)
                    .set("fill", "white")
                    .set("stroke", "black")
                    .set("stroke-width", 3)
                    .set("rx", 15);
                *svg = svg.clone().add(step);

                let caption = svg::node::element::Text::new()
                    .set("x", x)
                    .set("y", y)
                    .set("text-anchor", "middle")
                    .set("dominant-baseline", "central")
                    .set("fill", "black")
                    .set("font-size", 28)
                    .add(name);
                *svg = svg.clone().add(caption);
            }
            Type::END => {
                let end = svg::node::element::Circle::new()
                    .set("cx", x)
                    .set("cy", y)
                    .set("r", 25)
                    .set("stroke", "black")
                    .set("stroke-width", 2)
                    .set("fill", "none");
                *svg = svg.clone().add(end);

                let center = svg::node::element::Circle::new()
                    .set("cx", x)
                    .set("cy", y)
                    .set("r", 20);
                *svg = svg.clone().add(center);
            }
            Type::START => unreachable!()
        }
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

                    let name = if_st.next().unwrap().as_str().to_owned();
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

    pub fn draw(&self, x: usize, mut y: usize, svg: &mut SVG) {
        let mut i:usize = 0;
        let mut node_num: usize = 1;
        for node in self.nodes.iter() {
            node.draw(x, y, svg);
            if node.kind == Type::IF {
                self.alternatives[i].draw(x,y,svg);
                self.connect_last_node(x, y, i, svg);
                y += self.alternatives[i].get_height();
                i += 1;
            } else if node.kind == Type::STEP {
                if node_num != self.nodes.len(){
                    draw_line(x, y+20, x, y+60, svg);
                }
                y += 130;
            } else {
                y += 130;
            }
            node_num += 1;
        }
    }

    fn connect_last_node(&self, x: usize, y: usize, i: usize, svg: &mut SVG) {
        let y2 = y+self.alternatives[i].get_height();
        let y1 = y2-130;
        draw_line(x, y1, x, y2, svg)
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
}

pub struct Activity {
    path: Path
}

impl Activity {
    pub fn new(value: Pair<Rule>) -> Activity {
        let mut inner = value.clone().into_inner();
        inner.next();
        let p_body =  inner.next().unwrap();

        let path = Path::new(p_body);

        return Activity{
            path
        }
    }

    pub fn print(&self) {
        self.path.print()
    }

    pub fn draw(&self, svg: &mut SVG) {
        println!("{}", self.path.max_left());
        println!("{}", self.path.max_right());
        let left = self.path.max_left()*250;
        let right = self.path.max_right()*400;
        let width = left+right+200;

        let  height = self.path.get_height()+50;

        *svg = svg.clone().set("viewBox", format!("0 0 {} {}", width, height));

        let start = svg::node::element::Circle::new()
            .set("cx", left+100)
            .set("cy", 25)
            .set("r", 25);
        *svg = svg.clone().add(start);

        draw_line(left+100, 50, left+100, 85, svg);
        self.path.draw(left+100, 155, svg)
    }
}
