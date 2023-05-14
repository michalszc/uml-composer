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
        let context = Context::new(context_pair);
        assert_eq!(context.get_context_label().to_owned(), String::from("Restaurant"));
        assert_eq!(context.get_use_cases().len(), 2);
        assert_eq!(context.get_use_cases()[0].get_use_case_alias().to_owned(), String::from("uc1"));
        assert_eq!(context.get_use_cases()[0].get_use_case_label().to_owned(), String::from("Eat food"));
        assert_eq!(context.get_use_cases()[1].get_use_case_alias().to_owned(), String::from("uc2"));
        assert_eq!(context.get_use_cases()[1].get_use_case_label().to_owned(), String::from("Pay for food"));

        const START_SVG: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\">";
        const END_SVG: &str = "</svg>";
        let mut svg = SVG::new();
        let x = 50;
        let y = 100;
        let width = 200;
        let height = 200;
        context.draw(&mut svg, x, y, width, height);
        let expected_svg = format!(
            "{}\n\
        <text dominant-baseline=\"central\" fill=\"black\" font-size=\"28\" text-anchor=\"middle\" x=\"150\" y=\"80\">\n\
        {}\n\
        </text>\n\
        <rect fill=\"transparent\" fill-opacity=\"0.3\" height=\"{}\" rx=\"10\" ry=\"10\" stroke=\"gray\" stroke-width=\"2\" width=\"{}\" x=\"{}\" y=\"{}\"/>\n\
        <ellipse cx=\"149\" cy=\"137\" fill=\"blue\" fill-opacity=\"0.2\" rx=\"58\" ry=\"30\" stroke=\"black\" stroke-width=\"2\"/>\n\
        <text dominant-baseline=\"central\" fill=\"black\" font-size=\"20\" text-anchor=\"middle\" x=\"149\" y=\"143\">\n\
        Eat food\n\
        </text>\n\
        <ellipse cx=\"149\" cy=\"237\" fill=\"blue\" fill-opacity=\"0.2\" rx=\"82\" ry=\"30\" stroke=\"black\" stroke-width=\"2\"/>\n\
        <text dominant-baseline=\"central\" fill=\"black\" font-size=\"20\" text-anchor=\"middle\" x=\"149\" y=\"243\">\n\
        Pay for food\n\
        </text>\n\
    {}",
            START_SVG, context.get_context_label(), height, width, x, y, END_SVG
        );

        assert_eq!(svg.to_string(), expected_svg);
    }
}



