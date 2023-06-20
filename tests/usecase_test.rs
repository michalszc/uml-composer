#[cfg(test)]
mod use_case_test {
    use pest::Parser;
    use svg::node::element::SVG;
    use uml_composer::{rules::use_case::UseCase, grammar_parser::{GrammarParser, Rule}};

    #[test]
    fn use_case_with_alias() {
        let input: &str = "usecase \"Eat food\" as uc1 \n";
        let use_case_pair = GrammarParser::parse(Rule::USE_CASE, input)
            .unwrap().next().unwrap();
        let mut use_case = UseCase::new(use_case_pair);
        assert_eq!(use_case.get_use_case_label().to_owned(), String::from("Eat food"));
        assert_eq!(use_case.get_use_case_alias().to_owned(), String::from("uc1"));
        let mut svg = SVG::new();
        let x = 50;
        let y = 70;
        let height = 50;
        let width = 100;
        use_case.draw(&mut svg, x, y, height, width);
        assert!(svg.to_string().contains(&format!("<ellipse cx=\"{}\" cy=\"{}\"", x, y)));
        assert!(svg.to_string().contains(&format!("{}\n</text>", use_case.get_use_case_label())));
    }

    #[test]
    fn use_case_without_alias() {
        let input: &str = "usecase \"Eat food\"\n";
        let use_case_pair = GrammarParser::parse(Rule::USE_CASE, input)
            .unwrap().next().unwrap();
        let mut use_case = UseCase::new(use_case_pair);
        assert_eq!(use_case.get_use_case_label().to_owned(), String::from("Eat food"));
        assert_eq!(use_case.get_use_case_alias().to_owned(), String::from("Eat food"));

        let mut svg = SVG::new();
        let x = 120;
        let y = 100;
        let height = 100;
        let width = 200;
        use_case.draw(&mut svg, x, y, width, height);

        assert!(svg.to_string().contains(&format!("<ellipse cx=\"{}\" cy=\"{}\"", x, y)));
    }

}


