#[cfg(test)]
mod context_test {
    use pest::Parser;
    use svg::node::element::SVG;
    use uml_composer::{rules::context::Context, grammar_parser::{GrammarParser, Rule}};

    #[test]
    fn context_with_use_case() {
        let input: &str = "context Restaurant { \n usecase \"Eat food\" as uc1 \n usecase \"Pay for food\" as uc2 \n} \n";
        let context_pair = GrammarParser::parse(Rule::CONTEXT, input)
            .unwrap().next().unwrap();
        let mut context = Context::new(context_pair);
        assert_eq!(context.get_context_label().to_owned(), String::from("Restaurant"));
        assert_eq!(context.get_use_cases().len(), 2);
        assert_eq!(context.get_use_cases()[0].get_use_case_alias().to_owned(), String::from("uc1"));
        assert_eq!(context.get_use_cases()[0].get_use_case_label().to_owned(), String::from("Eat food"));
        assert_eq!(context.get_use_cases()[1].get_use_case_alias().to_owned(), String::from("uc2"));
        assert_eq!(context.get_use_cases()[1].get_use_case_label().to_owned(), String::from("Pay for food"));

        let mut svg = SVG::new();
        let x = 50;
        let y = 100;
        let width = 200;
        let height = 200;
        context.draw(&mut svg, x, y, width, height);
        assert!(svg.to_string().contains(&format!("height=\"{}\"", height)));
        assert!(svg.to_string().contains(&format!("width=\"{}\" x=\"{}\" y=\"{}\"/>", width, x, y)));
    }
}



