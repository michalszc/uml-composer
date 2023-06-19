use pest::iterators::Pair;
use crate::grammar_parser::Rule;
use svg;
use svg::node::element::SVG;
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
        let right = self.path.max_right()*400;
        let width = left+right+200;

        let  height = self.path.get_height()+50;

        *svg = svg.clone().set("viewBox", format!("0 0 {} {}", width, height));

        self.path.draw(left+100, 25, svg)
    }
}
