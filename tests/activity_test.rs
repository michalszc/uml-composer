#[cfg(test)]
mod activity_test {
    use pest::Parser;
    use svg::node::element::SVG;
    use uml_composer::{rules::activity::Activity, grammar_parser::{GrammarParser, Rule}};

    #[test]
    fn nodes_count() {
        let input="@startuml activity

(*) --> step1
--> step2 : tak
--> stepnew : opis
if condition1 {
    --> alt1 : tekst
    --> alt11
} else {
    -->another1
}
--> step3:caption -->(^)

@enduml";
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

    #[test]
    fn draws_step() {
        let mut svg = SVG::new();
        let input="@startuml activity

(*) --> step1
--> step2 : tak
--> stepnew : opis
if condition1 {
    --> alt1 : tekst
    --> alt11
} else {
    -->another1
}
--> step3:caption -->(^)

@enduml";
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
                                Activity::new(inner_pair).draw(&mut svg);
                                let response = svg.to_string();
                                assert_eq!(response.contains("<text dominant-baseline=\"central\" fill=\"black\" font-size=\"28\" text-anchor=\"middle\" x=\"350\" y=\"415\">\nstepnew\n</text>"), true)
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