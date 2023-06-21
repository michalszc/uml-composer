#[cfg(test)]
mod actor_test {
    use pest::Parser;
    use svg::node::element::SVG;
    use uml_composer::{rules::actor::Actor, grammar_parser::{GrammarParser, Rule}};

    #[test]
    fn actor_with_alias() {
        let input: &str = "actor Guest as g \n";
        let actor_pair = GrammarParser::parse(Rule::ACTOR, input)
            .unwrap().next().unwrap();
        let mut actor = Actor::new(actor_pair);
        assert_eq!(actor.get_actor_label().to_owned(), String::from("Guest"));
        assert_eq!(actor.get_actor_alias().to_owned(), String::from("g"));

        let mut svg = SVG::new();
        let x = 100;
        let y = 100;
        let r = 20;
        actor.draw(&mut svg, x, y, r);
        assert!(svg.to_string().contains(&format!("<circle cx=\"{}\" cy=\"{}\" fill=\"none\" r=\"{}\"", x, y, r)));
    }

    #[test]
    fn actor_without_alias() {
        let input: &str = "actor Guest2 \n";
        let link_pair = GrammarParser::parse(Rule::ACTOR, input)
            .unwrap().next().unwrap();
        let mut actor = Actor::new(link_pair);
        assert_eq!(actor.get_actor_label().to_owned(), String::from("Guest2"));
        assert_eq!(actor.get_actor_alias().to_owned(), String::from("Guest2"));

        let mut svg = SVG::new();
        let x = 100;
        let y = 100;
        let r = 20;
        actor.draw(&mut svg, x, y, r);

        assert!(svg.to_string().contains(&format!("<circle cx=\"{}\" cy=\"{}\" fill=\"none\" r=\"{}\"", x, y, r)));
    }

}

