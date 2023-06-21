use pest::iterators::Pair;
use crate::grammar_parser::Rule;
use svg::node::element::{SVG, Rectangle};
use crate::rules::path;
use path::Path;

pub struct Activity {
    path: Path
}

impl Activity {
    pub fn new(value: Pair<Rule>) -> Activity {
        let mut inner = value.clone().into_inner();
        inner.next();
        let p_body =  inner.next().unwrap();

        let path = Path::new(p_body, true);

        return Activity{
            path
        }
    }

    pub fn print(&self) {
        self.path.print()
    }

    pub fn draw(&self, svg: &mut SVG) {
        let left = self.path.max_left()*250;

        let rect = Rectangle::new()
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", "white");
        *svg = svg.clone().add(rect);

        self.path.draw(left+100, 25, svg)
    }

    pub fn width(&self) -> usize {
        let left = self.path.max_left()*250;
        let right = self.path.max_right()*400;
        left+right+200
    }

    pub fn height(&self) -> usize {
        self.path.get_height()+50
    }

    pub fn nodes_count(&self) -> usize {
        self.path.nodes_count()
    }
}
