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
        let use_case = UseCase::new(use_case_pair);
        assert_eq!(use_case.get_use_case_label().to_owned(), String::from("Eat food"));
        assert_eq!(use_case.get_use_case_alias().to_owned(), String::from("uc1"));
        const START_SVG: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\">";
        const END_SVG: &str = "</svg>";
        let mut svg = SVG::new();
        let x = 50;
        let y = 70;
        let height = 50;
        let width = 100;
        use_case.draw(&mut svg, x, y, height, width);

        // println!("{}", svg.to_string());

        let expected_svg = format!(
            "{}\n\
        <ellipse cx=\"75\" cy=\"120\" fill=\"blue\" fill-opacity=\"0.2\" rx=\"58\" ry=\"50\" stroke=\"black\" stroke-width=\"2\"/>\n\
        <text dominant-baseline=\"central\" fill=\"black\" font-size=\"20\" text-anchor=\"middle\" x=\"75\" y=\"126\">\n\
        {}\n\
        </text>\n\
        {}",
            START_SVG, use_case.get_use_case_label(), END_SVG
        );

        assert_eq!(svg.to_string(), expected_svg);
    }

    #[test]
    fn use_case_without_alias() {
        let input: &str = "usecase \"Eat food\"\n";
        let use_case_pair = GrammarParser::parse(Rule::USE_CASE, input)
            .unwrap().next().unwrap();
        let use_case = UseCase::new(use_case_pair);
        assert_eq!(use_case.get_use_case_label().to_owned(), String::from("Eat food"));
        assert_eq!(use_case.get_use_case_alias().to_owned(), String::from("Eat food"));

        const START_SVG: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\">";
        const END_SVG: &str = "</svg>";
        let mut svg = SVG::new();
        let x = 120;
        let y = 100;
        let height = 100;
        let width = 200;
        use_case.draw(&mut svg, x, y, width, height);

        let expected_svg = format!(
            "{}\n\
        <ellipse cx=\"{}\" cy=\"{}\" fill=\"blue\" fill-opacity=\"0.2\" rx=\"{}\" ry=\"50\" stroke=\"black\" stroke-width=\"2\"/>\n\
        <text dominant-baseline=\"central\" fill=\"black\" font-size=\"20\" text-anchor=\"middle\" x=\"{}\" y=\"156\">\n\
        Eat food\n\
        </text>\n\
        {}",
            START_SVG, x + width / 2, y + height / 2, width / 2, x + width / 2, END_SVG
        );


        assert_eq!(svg.to_string(), expected_svg);
    }

}


