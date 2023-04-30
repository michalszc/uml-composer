#[cfg(test)]
mod link_test {
    use pest::Parser;
    use uml_composer::{rules::link::{Link, ArrowType, LinkType}, grammar_parser::{GrammarParser, Rule}};

    #[test]
    fn parse_into_link_solid_line() {
        let input: &str = "a -- b \n";
        let link_pair = GrammarParser::parse(Rule::LINK, input)
            .unwrap().next().unwrap();
        let link = Link::new(link_pair);
        assert_eq!(link.get_left_id().to_owned(), String::from("a"));
        assert_eq!(link.get_right_id().to_owned(), String::from("b"));
        assert_eq!(*link.get_link_type(), LinkType::SolidLine);
        assert_eq!(link.get_label().to_owned(), String::from(""));
        assert_eq!(*link.get_arrow(), ArrowType::Missing);
    }

    #[test]
    fn parse_into_link_solid_line_with_label() {
        let input: &str = "a -- b : \"lorem ipsum 123\" \n";
        let link_pair = GrammarParser::parse(Rule::LINK, input)
            .unwrap().next().unwrap();
        let link = Link::new(link_pair);
        assert_eq!(link.get_left_id().to_owned(), String::from("a"));
        assert_eq!(link.get_right_id().to_owned(), String::from("b"));
        assert_eq!(*link.get_link_type(), LinkType::SolidLine);
        assert_eq!(link.get_label().to_owned(), String::from("lorem ipsum 123"));
    }

    #[test]
    fn parse_into_link_solid_line_with_label_and_arrow_left() {
        let input: &str = "a -- b : \"lorem ipsum 123\" < \n";
        let link_pair = GrammarParser::parse(Rule::LINK, input)
            .unwrap().next().unwrap();
        let link = Link::new(link_pair);
        assert_eq!(link.get_left_id().to_owned(), String::from("a"));
        assert_eq!(link.get_right_id().to_owned(), String::from("b"));
        assert_eq!(*link.get_link_type(), LinkType::SolidLine);
        assert_eq!(link.get_label().to_owned(), String::from("lorem ipsum 123"));
        assert_eq!(*link.get_arrow(), ArrowType::Left);
    }

    #[test]
    fn parse_into_link_solid_line_with_label_and_arrow_right() {
        let input: &str = "a -- b : \"lorem ipsum 123\" > \n";
        let link_pair = GrammarParser::parse(Rule::LINK, input)
            .unwrap().next().unwrap();
        let link = Link::new(link_pair);
        assert_eq!(link.get_left_id().to_owned(), String::from("a"));
        assert_eq!(link.get_right_id().to_owned(), String::from("b"));
        assert_eq!(*link.get_link_type(), LinkType::SolidLine);
        assert_eq!(link.get_label().to_owned(), String::from("lorem ipsum 123"));
        assert_eq!(*link.get_arrow(), ArrowType::Right);
    }

    #[test]
    fn parse_into_link_solid_arrow() {
        let input: &str = "a --> b \n";
        let link_pair = GrammarParser::parse(Rule::LINK, input)
            .unwrap().next().unwrap();
        let link = Link::new(link_pair);
        assert_eq!(link.get_left_id().to_owned(), String::from("a"));
        assert_eq!(link.get_right_id().to_owned(), String::from("b"));
        assert_eq!(*link.get_link_type(), LinkType::SolidArrow);
        assert_eq!(link.get_label().to_owned(), String::from(""));
        assert_eq!(*link.get_arrow(), ArrowType::Missing);
    }

    #[test]
    fn parse_into_link_solid_arrow_with_label() {
        let input: &str = "a --> b : \"lorem ipsum 123\" \n";
        let link_pair = GrammarParser::parse(Rule::LINK, input)
            .unwrap().next().unwrap();
        let link = Link::new(link_pair);
        assert_eq!(link.get_left_id().to_owned(), String::from("a"));
        assert_eq!(link.get_right_id().to_owned(), String::from("b"));
        assert_eq!(*link.get_link_type(), LinkType::SolidArrow);
        assert_eq!(link.get_label().to_owned(), String::from("lorem ipsum 123"));
        assert_eq!(*link.get_arrow(), ArrowType::Missing);
    }

    #[test]
    fn parse_into_link_dashed_line() {
        let input: &str = "a .. b \n";
        let link_pair = GrammarParser::parse(Rule::LINK, input)
            .unwrap().next().unwrap();
        let link = Link::new(link_pair);
        assert_eq!(link.get_left_id().to_owned(), String::from("a"));
        assert_eq!(link.get_right_id().to_owned(), String::from("b"));
        assert_eq!(*link.get_link_type(), LinkType::DashedLine);
        assert_eq!(link.get_label().to_owned(), String::from(""));
        assert_eq!(*link.get_arrow(), ArrowType::Missing);
    }

    #[test]
    fn parse_into_link_dashed_line_with_label() {
        let input: &str = "a .. b : \"lorem ipsum 123\" \n";
        let link_pair = GrammarParser::parse(Rule::LINK, input)
            .unwrap().next().unwrap();
        let link = Link::new(link_pair);
        assert_eq!(link.get_left_id().to_owned(), String::from("a"));
        assert_eq!(link.get_right_id().to_owned(), String::from("b"));
        assert_eq!(*link.get_link_type(), LinkType::DashedLine);
        assert_eq!(link.get_label().to_owned(), String::from("lorem ipsum 123"));
        assert_eq!(*link.get_arrow(), ArrowType::Missing);
    }

    #[test]
    fn parse_into_link_dashed_line_with_label_and_arrow_left() {
        let input: &str = "a .. b : \"lorem ipsum 123\" < \n";
        let link_pair = GrammarParser::parse(Rule::LINK, input)
            .unwrap().next().unwrap();
        let link = Link::new(link_pair);
        assert_eq!(link.get_left_id().to_owned(), String::from("a"));
        assert_eq!(link.get_right_id().to_owned(), String::from("b"));
        assert_eq!(*link.get_link_type(), LinkType::DashedLine);
        assert_eq!(link.get_label().to_owned(), String::from("lorem ipsum 123"));
        assert_eq!(*link.get_arrow(), ArrowType::Left);
    }

    #[test]
    fn parse_into_link_dashed_line_with_label_and_arrow_right() {
        let input: &str = "a .. b : \"lorem ipsum 123\" > \n";
        let link_pair = GrammarParser::parse(Rule::LINK, input)
            .unwrap().next().unwrap();
        let link = Link::new(link_pair);
        assert_eq!(link.get_left_id().to_owned(), String::from("a"));
        assert_eq!(link.get_right_id().to_owned(), String::from("b"));
        assert_eq!(*link.get_link_type(), LinkType::DashedLine);
        assert_eq!(link.get_label().to_owned(), String::from("lorem ipsum 123"));
        assert_eq!(*link.get_arrow(), ArrowType::Right);
    }

    #[test]
    fn parse_into_link_dashed_arrow() {
        let input: &str = "a ..> b \n";
        let link_pair = GrammarParser::parse(Rule::LINK, input)
            .unwrap().next().unwrap();
        let link = Link::new(link_pair);
        assert_eq!(link.get_left_id().to_owned(), String::from("a"));
        assert_eq!(link.get_right_id().to_owned(), String::from("b"));
        assert_eq!(*link.get_link_type(), LinkType::DashedArrow);
        assert_eq!(link.get_label().to_owned(), String::from(""));
        assert_eq!(*link.get_arrow(), ArrowType::Missing);
    }

    #[test]
    fn parse_into_link_dashed_arrow_with_label() {
        let input: &str = "a ..> b : \"lorem ipsum 123\" \n";
        let link_pair = GrammarParser::parse(Rule::LINK, input)
            .unwrap().next().unwrap();
        let link = Link::new(link_pair);
        assert_eq!(link.get_left_id().to_owned(), String::from("a"));
        assert_eq!(link.get_right_id().to_owned(), String::from("b"));
        assert_eq!(*link.get_link_type(), LinkType::DashedArrow);
        assert_eq!(link.get_label().to_owned(), String::from("lorem ipsum 123"));
        assert_eq!(*link.get_arrow(), ArrowType::Missing);
    }

}
