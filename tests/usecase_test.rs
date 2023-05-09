#[cfg(test)]
mod link_test {
    use pest::Parser;
    use uml_composer::{rules::use_case::UseCase, grammar_parser::{GrammarParser, Rule}};

    #[test]
    fn use_case_with_alias() {
        let input: &str = "usecase \"Eat food\" as uc1 \n";
        let use_case_pair = GrammarParser::parse(Rule::USE_CASE, input)
            .unwrap().next().unwrap();
        let use_case = UseCase::new(use_case_pair);
        assert_eq!(use_case.get_use_case_label().to_owned(), String::from("Eat food"));
        assert_eq!(use_case.get_use_case_alias().to_owned(), String::from("uc1"));
    }

    #[test]
    fn use_case_without_alias() {
        let input: &str = "usecase \"Eat food\"\n";
        let use_case_pair = GrammarParser::parse(Rule::USE_CASE, input)
            .unwrap().next().unwrap();
        let use_case = UseCase::new(use_case_pair);
        assert_eq!(use_case.get_use_case_label().to_owned(), String::from("Eat food"));
        assert_eq!(use_case.get_use_case_alias().to_owned(), String::from("Eat food"));
    }

}


