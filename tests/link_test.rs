#[cfg(test)]
mod link_test {
    use pest::Parser;
    use svg::node::element::SVG;
    use uml_composer::{rules::link::{Link, ArrowType, LinkType}, grammar_parser::{GrammarParser, Rule}};

    const START_SVG: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\">";
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
        assert_eq!(svg.to_string(),
            format!(
                "{}\n<line stroke=\"#000\" stroke-width=\"8\" x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>\n{}",
                START_SVG, x1, x2, y1, y2, END_SVG
            )
        );
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
        assert_eq!(svg.to_string(),
            format!(
                "{}\n<text dominant-baseline=\"central\" fill=\"black\" font-size=\"28\" text-anchor=\"middle\" transform=\"rotate(45 120 120)\" x=\"{}\" y=\"{}\">\n{}\n</text>\n<line stroke=\"#000\" stroke-width=\"8\" x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>\n{}",
                START_SVG, (x1+x2)/2 , (y1+y2)/2-20, "lorem ipsum 123", x1, x2, y1, y2, END_SVG
            )
        );
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
        assert_eq!(svg.to_string(),
            format!(
                "{}\n<text dominant-baseline=\"central\" fill=\"black\" font-size=\"28\" text-anchor=\"middle\" transform=\"rotate(45 120 120)\" x=\"{}\" y=\"{}\">\n{}\n</text>\n<line stroke=\"#000\" stroke-width=\"8\" x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>\n{}",
                START_SVG, (x1+x2)/2 , (y1+y2)/2-20, "lorem ipsum 123◀", x1, x2, y1, y2, END_SVG
            )
        );
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
        assert_eq!(svg.to_string(),
            format!(
                "{}\n<text dominant-baseline=\"central\" fill=\"black\" font-size=\"28\" text-anchor=\"middle\" transform=\"rotate(45 120 120)\" x=\"{}\" y=\"{}\">\n{}\n</text>\n<line stroke=\"#000\" stroke-width=\"8\" x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>\n{}",
                START_SVG, (x1+x2)/2 , (y1+y2)/2-20, "lorem ipsum 123▶", x1, x2, y1, y2, END_SVG
            )
        );
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
        assert_eq!(svg.to_string(),
            format!(
                "{}\n<defs>\n<marker id=\"arrowhead\" markerHeight=\"7\" markerWidth=\"10\" orient=\"auto\" refX=\"0\" refY=\"3.5\">\n<polygon points=\"0 0, 10 3.5, 0 7\"/>\n</marker>\n</defs>\n<line marker-end=\"url(#arrowhead)\" stroke=\"#000\" stroke-width=\"8\" x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>\n{}",
                START_SVG, x1, x2, y1, y2, END_SVG
            )
        );
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
        assert_eq!(svg.to_string(),
            format!(
                "{}\n<defs>\n<marker id=\"arrowhead\" markerHeight=\"7\" markerWidth=\"10\" orient=\"auto\" refX=\"0\" refY=\"3.5\">\n<polygon points=\"0 0, 10 3.5, 0 7\"/>\n</marker>\n</defs>\n<text dominant-baseline=\"central\" fill=\"black\" font-size=\"28\" text-anchor=\"middle\" transform=\"rotate(45 120 120)\" x=\"{}\" y=\"{}\">\n{}\n</text>\n<line marker-end=\"url(#arrowhead)\" stroke=\"#000\" stroke-width=\"8\" x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>\n{}",
                START_SVG, (x1+x2)/2 , (y1+y2)/2-20, "lorem ipsum 123", x1, x2, y1, y2, END_SVG
            )
        );
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
        assert_eq!(svg.to_string(),
            format!(
                "{}\n<line stroke=\"#000\" stroke-dasharray=\"8 8\" stroke-width=\"8\" x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>\n{}",
                START_SVG, x1, x2, y1, y2, END_SVG
            )
        );
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
        assert_eq!(svg.to_string(),
            format!(
                "{}\n<text dominant-baseline=\"central\" fill=\"black\" font-size=\"28\" text-anchor=\"middle\" transform=\"rotate(45 120 120)\" x=\"{}\" y=\"{}\">\n{}\n</text>\n<line stroke=\"#000\" stroke-dasharray=\"8 8\" stroke-width=\"8\" x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>\n{}",
                START_SVG, (x1+x2)/2 , (y1+y2)/2-20, "lorem ipsum 123", x1, x2, y1, y2, END_SVG
            )
        );
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
        assert_eq!(svg.to_string(),
            format!(
                "{}\n<text dominant-baseline=\"central\" fill=\"black\" font-size=\"28\" text-anchor=\"middle\" transform=\"rotate(45 120 120)\" x=\"{}\" y=\"{}\">\n{}\n</text>\n<line stroke=\"#000\" stroke-dasharray=\"8 8\" stroke-width=\"8\" x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>\n{}",
                START_SVG, (x1+x2)/2 , (y1+y2)/2-20, "lorem ipsum 123◀", x1, x2, y1, y2, END_SVG
            )
        );
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
        assert_eq!(svg.to_string(),
            format!(
                "{}\n<text dominant-baseline=\"central\" fill=\"black\" font-size=\"28\" text-anchor=\"middle\" transform=\"rotate(45 120 120)\" x=\"{}\" y=\"{}\">\n{}\n</text>\n<line stroke=\"#000\" stroke-dasharray=\"8 8\" stroke-width=\"8\" x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>\n{}",
                START_SVG, (x1+x2)/2 , (y1+y2)/2-20, "lorem ipsum 123▶", x1, x2, y1, y2, END_SVG
            )
        );
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
        assert_eq!(svg.to_string(),
            format!(
                "{}\n<defs>\n<marker id=\"arrowhead\" markerHeight=\"7\" markerWidth=\"10\" orient=\"auto\" refX=\"0\" refY=\"3.5\">\n<polygon points=\"0 0, 10 3.5, 0 7\"/>\n</marker>\n</defs>\n<line marker-end=\"url(#arrowhead)\" stroke=\"#000\" stroke-dasharray=\"8 8\" stroke-width=\"8\" x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>\n{}",
                START_SVG, x1, x2, y1, y2, END_SVG
            )
        );
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
        assert_eq!(svg.to_string(),
            format!(
                "{}\n<defs>\n<marker id=\"arrowhead\" markerHeight=\"7\" markerWidth=\"10\" orient=\"auto\" refX=\"0\" refY=\"3.5\">\n<polygon points=\"0 0, 10 3.5, 0 7\"/>\n</marker>\n</defs>\n<text dominant-baseline=\"central\" fill=\"black\" font-size=\"28\" text-anchor=\"middle\" transform=\"rotate(45 120 120)\" x=\"{}\" y=\"{}\">\n{}\n</text>\n<line marker-end=\"url(#arrowhead)\" stroke=\"#000\" stroke-dasharray=\"8 8\" stroke-width=\"8\" x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>\n{}",
                START_SVG, (x1+x2)/2 , (y1+y2)/2-20, "lorem ipsum 123", x1, x2, y1, y2, END_SVG
            )
        );
    }

}
