#[cfg(test)]
mod link_test {
    use pest::Parser;
    use svg::node::element::SVG;
    use uml_composer::{rules::link::{Link, ArrowType, LinkType}, grammar_parser::{GrammarParser, Rule}};

    const END_SVG: &str = "</svg>";

    #[test]
    fn parse_into_link_solid_line() {
        let input: &str = "a -- b \n";
        let link_pair = GrammarParser::parse(Rule::LINK, input)
            .unwrap().next().unwrap();
        let link = Link::new(link_pair);
        let mut svg = SVG::new();
        let x1 = 20;
        let x2 = 220;
        let y1 = 20;
        let y2 = 220;
        link.draw(&mut svg, x1, y1, x2, y2);
        assert_eq!(link.get_left_id().to_owned(), String::from("a"));
        assert_eq!(link.get_right_id().to_owned(), String::from("b"));
        assert_eq!(*link.get_link_type(), LinkType::SolidLine);
        assert_eq!(link.get_label().to_owned(), String::from(""));
        assert_eq!(*link.get_arrow(), ArrowType::Missing);
        assert!(svg.to_string().contains(&format!(
                "x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>\n{}",
                x1, x2, y1, y2, END_SVG
            )
        ));
    }

    #[test]
    fn parse_into_link_solid_line_with_label() {
        let input: &str = "a -- b : \"lorem ipsum 123\" \n";
        let link_pair = GrammarParser::parse(Rule::LINK, input)
            .unwrap().next().unwrap();
        let link = Link::new(link_pair);
        let mut svg = SVG::new();
        let x1 = 20;
        let x2 = 220;
        let y1 = 20;
        let y2 = 220;
        link.draw(&mut svg, x1, y1, x2, y2);
        assert_eq!(link.get_left_id().to_owned(), String::from("a"));
        assert_eq!(link.get_right_id().to_owned(), String::from("b"));
        assert_eq!(*link.get_link_type(), LinkType::SolidLine);
        assert_eq!(link.get_label().to_owned(), String::from("lorem ipsum 123"));
        assert!(svg.to_string().contains(&format!(
            "x=\"{}\" y=\"{}\">\n{}\n</text>\n<line stroke=\"#000\" stroke-width=\"3\" x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>\n{}",
            (x1+x2)/2 , (y1+y2)/2-5, "lorem ipsum 123", x1, x2, y1, y2, END_SVG
        )
        ));
    }

    #[test]
    fn parse_into_link_solid_line_with_label_and_arrow_left() {
        let input: &str = "a -- b : \"lorem ipsum 123\" < \n";
        let link_pair = GrammarParser::parse(Rule::LINK, input)
            .unwrap().next().unwrap();
        let link = Link::new(link_pair);
        let mut svg = SVG::new();
        let x1 = 20;
        let x2 = 220;
        let y1 = 20;
        let y2 = 220;
        link.draw(&mut svg, x1, y1, x2, y2);
        assert_eq!(link.get_left_id().to_owned(), String::from("a"));
        assert_eq!(link.get_right_id().to_owned(), String::from("b"));
        assert_eq!(*link.get_link_type(), LinkType::SolidLine);
        assert_eq!(link.get_label().to_owned(), String::from("lorem ipsum 123"));
        assert_eq!(*link.get_arrow(), ArrowType::Left);
        assert!(svg.to_string().contains(&format!(
                "x=\"{}\" y=\"{}\">\n{}\n</text>\n<line stroke=\"#000\" stroke-width=\"3\" x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>\n{}",
                (x1+x2)/2 , (y1+y2)/2-5, "lorem ipsum 123◀", x1, x2, y1, y2, END_SVG
            )
        ));
    }

    #[test]
    fn parse_into_link_solid_line_with_label_and_arrow_right() {
        let input: &str = "a -- b : \"lorem ipsum 123\" > \n";
        let link_pair = GrammarParser::parse(Rule::LINK, input)
            .unwrap().next().unwrap();
        let link = Link::new(link_pair);
        let mut svg = SVG::new();
        let x1 = 20;
        let x2 = 220;
        let y1 = 20;
        let y2 = 220;
        link.draw(&mut svg, x1, y1, x2, y2);
        assert_eq!(link.get_left_id().to_owned(), String::from("a"));
        assert_eq!(link.get_right_id().to_owned(), String::from("b"));
        assert_eq!(*link.get_link_type(), LinkType::SolidLine);
        assert_eq!(link.get_label().to_owned(), String::from("lorem ipsum 123"));
        assert_eq!(*link.get_arrow(), ArrowType::Right);
        assert!(svg.to_string().contains(&format!(
                "x=\"{}\" y=\"{}\">\n{}\n</text>\n<line stroke=\"#000\" stroke-width=\"3\" x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>\n{}",
                (x1+x2)/2 , (y1+y2)/2-5, "lorem ipsum 123▶", x1, x2, y1, y2, END_SVG
            )
        ));
    }

    #[test]
    fn parse_into_link_solid_arrow() {
        let input: &str = "a --> b \n";
        let link_pair = GrammarParser::parse(Rule::LINK, input)
            .unwrap().next().unwrap();
        let link = Link::new(link_pair);
        let mut svg = SVG::new();
        let x1 = 20;
        let x2 = 220;
        let y1 = 20;
        let y2 = 220;
        link.draw(&mut svg, x1, y1, x2, y2);
        assert_eq!(link.get_left_id().to_owned(), String::from("a"));
        assert_eq!(link.get_right_id().to_owned(), String::from("b"));
        assert_eq!(*link.get_link_type(), LinkType::SolidArrow);
        assert_eq!(link.get_label().to_owned(), String::from(""));
        assert_eq!(*link.get_arrow(), ArrowType::Missing);
        assert!(svg.to_string().contains(&format!(
                "x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>\n{}",
                x1, x2, y1, y2, END_SVG
            )
        ));
    }

    #[test]
    fn parse_into_link_solid_arrow_with_label() {
        let input: &str = "a --> b : \"lorem ipsum 123\" \n";
        let link_pair = GrammarParser::parse(Rule::LINK, input)
            .unwrap().next().unwrap();
        let link = Link::new(link_pair);
        let mut svg = SVG::new();
        let x1 = 20;
        let x2 = 220;
        let y1 = 20;
        let y2 = 220;
        link.draw(&mut svg, x1, y1, x2, y2);
        assert_eq!(link.get_left_id().to_owned(), String::from("a"));
        assert_eq!(link.get_right_id().to_owned(), String::from("b"));
        assert_eq!(*link.get_link_type(), LinkType::SolidArrow);
        assert_eq!(link.get_label().to_owned(), String::from("lorem ipsum 123"));
        assert_eq!(*link.get_arrow(), ArrowType::Missing);
        assert!(svg.to_string().contains(&format!(
                "x=\"{}\" y=\"{}\">\n{}\n</text>\n",
                (x1+x2)/2 , (y1+y2)/2-5, "lorem ipsum 123"
            )
        ));
    }

    #[test]
    fn parse_into_link_dashed_line() {
        let input: &str = "a .. b \n";
        let link_pair = GrammarParser::parse(Rule::LINK, input)
            .unwrap().next().unwrap();
        let link = Link::new(link_pair);
        let mut svg = SVG::new();
        let x1 = 20;
        let x2 = 220;
        let y1 = 20;
        let y2 = 220;
        link.draw(&mut svg, x1, y1, x2, y2);
        assert_eq!(link.get_left_id().to_owned(), String::from("a"));
        assert_eq!(link.get_right_id().to_owned(), String::from("b"));
        assert_eq!(*link.get_link_type(), LinkType::DashedLine);
        assert_eq!(link.get_label().to_owned(), String::from(""));
        assert_eq!(*link.get_arrow(), ArrowType::Missing);
        assert!(svg.to_string().contains(&format!(
                "x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>\n{}",
                x1, x2, y1, y2, END_SVG
            )
        ));
    }

    #[test]
    fn parse_into_link_dashed_line_with_label() {
        let input: &str = "a .. b : \"lorem ipsum 123\" \n";
        let link_pair = GrammarParser::parse(Rule::LINK, input)
            .unwrap().next().unwrap();
        let link = Link::new(link_pair);
        let mut svg = SVG::new();
        let x1 = 20;
        let x2 = 220;
        let y1 = 20;
        let y2 = 220;
        link.draw(&mut svg, x1, y1, x2, y2);
        assert_eq!(link.get_left_id().to_owned(), String::from("a"));
        assert_eq!(link.get_right_id().to_owned(), String::from("b"));
        assert_eq!(*link.get_link_type(), LinkType::DashedLine);
        assert_eq!(link.get_label().to_owned(), String::from("lorem ipsum 123"));
        assert_eq!(*link.get_arrow(), ArrowType::Missing);
        assert!(svg.to_string().contains(&format!(
                "x=\"{}\" y=\"{}\">\n{}\n</text>\n<line stroke=\"#000\" stroke-dasharray=\"8 8\" stroke-width=\"3\" x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>\n{}",
                (x1+x2)/2 , (y1+y2)/2-5, "lorem ipsum 123", x1, x2, y1, y2, END_SVG
            )
        ));
    }

    #[test]
    fn parse_into_link_dashed_line_with_label_and_arrow_left() {
        let input: &str = "a .. b : \"lorem ipsum 123\" < \n";
        let link_pair = GrammarParser::parse(Rule::LINK, input)
            .unwrap().next().unwrap();
        let link = Link::new(link_pair);
        let mut svg = SVG::new();
        let x1 = 20;
        let x2 = 220;
        let y1 = 20;
        let y2 = 220;
        link.draw(&mut svg, x1, y1, x2, y2);
        assert_eq!(link.get_left_id().to_owned(), String::from("a"));
        assert_eq!(link.get_right_id().to_owned(), String::from("b"));
        assert_eq!(*link.get_link_type(), LinkType::DashedLine);
        assert_eq!(link.get_label().to_owned(), String::from("lorem ipsum 123"));
        assert_eq!(*link.get_arrow(), ArrowType::Left);
        assert!(svg.to_string().contains(&format!(
                "x=\"{}\" y=\"{}\">\n{}\n</text>\n<line stroke=\"#000\" stroke-dasharray=\"8 8\" stroke-width=\"3\" x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>\n{}",
                (x1+x2)/2 , (y1+y2)/2-5, "lorem ipsum 123◀", x1, x2, y1, y2, END_SVG
            )
        ));
    }

    #[test]
    fn parse_into_link_dashed_line_with_label_and_arrow_right() {
        let input: &str = "a .. b : \"lorem ipsum 123\" > \n";
        let link_pair = GrammarParser::parse(Rule::LINK, input)
            .unwrap().next().unwrap();
        let link = Link::new(link_pair);
        let mut svg = SVG::new();
        let x1 = 20;
        let x2 = 220;
        let y1 = 20;
        let y2 = 220;
        link.draw(&mut svg, x1, y1, x2, y2);
        assert_eq!(link.get_left_id().to_owned(), String::from("a"));
        assert_eq!(link.get_right_id().to_owned(), String::from("b"));
        assert_eq!(*link.get_link_type(), LinkType::DashedLine);
        assert_eq!(link.get_label().to_owned(), String::from("lorem ipsum 123"));
        assert_eq!(*link.get_arrow(), ArrowType::Right);
        assert!(svg.to_string().contains(&format!(
                "x=\"{}\" y=\"{}\">\n{}\n</text>\n<line stroke=\"#000\" stroke-dasharray=\"8 8\" stroke-width=\"3\" x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>\n{}",
                (x1+x2)/2 , (y1+y2)/2-5, "lorem ipsum 123▶", x1, x2, y1, y2, END_SVG
            )
        ));
    }

    #[test]
    fn parse_into_link_dashed_arrow() {
        let input: &str = "a ..> b \n";
        let link_pair = GrammarParser::parse(Rule::LINK, input)
            .unwrap().next().unwrap();
        let link = Link::new(link_pair);
        let mut svg = SVG::new();
        let x1 = 20;
        let x2 = 220;
        let y1 = 20;
        let y2 = 220;
        link.draw(&mut svg, x1, y1, x2, y2);
        assert_eq!(link.get_left_id().to_owned(), String::from("a"));
        assert_eq!(link.get_right_id().to_owned(), String::from("b"));
        assert_eq!(*link.get_link_type(), LinkType::DashedArrow);
        assert_eq!(link.get_label().to_owned(), String::from(""));
        assert_eq!(*link.get_arrow(), ArrowType::Missing);
        assert!(svg.to_string().contains(&format!("x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>\n{}", x1, x2, y1, y2, END_SVG)));
    }

    #[test]
    fn parse_into_link_dashed_arrow_with_label() {
        let input: &str = "a ..> b : \"lorem ipsum 123\" \n";
        let link_pair = GrammarParser::parse(Rule::LINK, input)
            .unwrap().next().unwrap();
        let link = Link::new(link_pair);
        let mut svg = SVG::new();
        let x1 = 20;
        let x2 = 220;
        let y1 = 20;
        let y2 = 220;
        link.draw(&mut svg, x1, y1, x2, y2);
        assert_eq!(link.get_left_id().to_owned(), String::from("a"));
        assert_eq!(link.get_right_id().to_owned(), String::from("b"));
        assert_eq!(*link.get_link_type(), LinkType::DashedArrow);
        assert_eq!(link.get_label().to_owned(), String::from("lorem ipsum 123"));
        assert_eq!(*link.get_arrow(), ArrowType::Missing);
        assert!(svg.to_string().contains(&format!(
                "x=\"{}\" y=\"{}\">\n{}\n</text>\n",
                (x1+x2)/2 , (y1+y2)/2-5, "lorem ipsum 123"
            )
        ));
    }

    #[test]
    fn parse_into_link_in_class_diagram(){
        let input: &str = "uc1 --> uc5 : \"Include\" \n";

        let link_pair = GrammarParser::parse(Rule::LINK, input)
            .unwrap().next().unwrap();

        let mut link = Link::new(link_pair);
        let mut svg = SVG::new();
        let x1 = 300;
        let x2 = 1500;
        let y1 = 100;
        let y2 = 400;
        let xs = 900;

        link.draw_class_link(&mut svg, x1, y1, x2, y2, xs);

        // first line
        assert!(svg.to_string().contains(&format!(
            "x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>", x1, xs, y1, y1)));

        // middle line
        assert!(svg.to_string().contains(&format!(
            "x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>", xs, xs, y1, y2)));

        // second line
        assert!(svg.to_string().contains(&format!(
            "x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>", xs, x2, y2, y2)));

    }

}
