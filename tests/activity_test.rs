#[cfg(test)]
mod activity_test {
    use pest::Parser;
    use svg::node::element::SVG;
    use uml_composer::{rules::activity::Activity, grammar_parser::{GrammarParser, Rule}};

    #[test]
    fn nodes_count() {
        let input="@startuml activity\n\n(*) --> step1\n--> step2 : tak\n--> stepnew : opis\nif condition1 {\n\t--> alt1 : tekst\n\t--> alt11\n} else {\n\t-->another1\n}\n--> step3:caption -->(^)\n\n@enduml";
        let diagram = GrammarParser::parse(Rule::PROGRAM, input)
            .unwrap_or_else(|e| panic!("{}", e))
            .next().unwrap();
        for pair in diagram.into_inner() {
            match pair.as_rule() {
                Rule::ACTIVITY_DIAGRAM => {
                    for inner_pair in pair.into_inner(){
                        match inner_pair.as_rule() {
                            Rule::start_activity => {}
                            Rule::ACTIVITY_BODY => {
                                assert_eq!(10, Activity::new(inner_pair).nodes_count());
                            }
                            _ => unreachable!()
                        }
                    }
                }
                Rule::end_uml => {},
                _ => panic!("{}", pair.to_string())
            }
        }
    }
}