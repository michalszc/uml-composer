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
        let actor = Actor::new(actor_pair);
        assert_eq!(actor.get_actor_label().to_owned(), String::from("Guest"));
        assert_eq!(actor.get_actor_alias().to_owned(), String::from("g"));

        const START_SVG: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\">";
        const END_SVG: &str = "</svg>";
        let mut svg = SVG::new();
        let x = 100;
        let y = 100;
        let r = 20;
        actor.draw(&mut svg, x, y, r);
        // println!("{}", svg.to_string());
        let expected_svg = format!(
            "{}\n<text dominant-baseline=\"central\" fill=\"black\" font-size=\"28\" text-anchor=\
            \"middle\" x=\"100\" y=\"60\">\nGuest\n</text>\n<circle cx=\"{}\" cy=\"{}\" fill=\
            \"none\" r=\"{}\" stroke=\"#000\" stroke-width=\"5\"/>\n<line stroke=\"#000\" \
            stroke-width=\"5\" x1=\"100\" x2=\"100\" y1=\"120\" y2=\"160\"/>\n<line stroke=\
            \"#000\" stroke-width=\"5\" x1=\"70\" x2=\"100\" y1=\"120\" y2=\"140\"/>\n<line stroke=\
            \"#000\" stroke-width=\"5\" x1=\"130\" x2=\"100\" y1=\"120\" y2=\"140\"/>\n<line stroke=\
            \"#000\" stroke-width=\"5\" x1=\"70\" x2=\"100\" y1=\"200\" y2=\"160\"/>\n<line stroke=\
            \"#000\" stroke-width=\"5\" x1=\"130\" x2=\"100\" y1=\"200\" y2=\"160\"/>\n{}",
            START_SVG, x, y, r, END_SVG
        );
        assert_eq!(svg.to_string(), expected_svg);
    }

    #[test]
    fn actor_without_alias() {
        let input: &str = "actor Guest2 \n";
        let link_pair = GrammarParser::parse(Rule::ACTOR, input)
            .unwrap().next().unwrap();
        let actor = Actor::new(link_pair);
        assert_eq!(actor.get_actor_label().to_owned(), String::from("Guest2"));
        assert_eq!(actor.get_actor_alias().to_owned(), String::from("Guest2"));

        const START_SVG: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\">";
        const END_SVG: &str = "</svg>";
        let mut svg = SVG::new();
        let x = 100;
        let y = 100;
        let r = 20;
        actor.draw(&mut svg, x, y, r);
        let expected_svg = format!(
            "{}\n<text dominant-baseline=\"central\" fill=\"black\" font-size=\"28\" text-anchor=\
            \"middle\" x=\"100\" y=\"60\">\nGuest2\n</text>\n<circle cx=\"{}\" cy=\"{}\" fill=\
            \"none\" r=\"{}\" stroke=\"#000\" stroke-width=\"5\"/>\n<line stroke=\"#000\" \
            stroke-width=\"5\" x1=\"100\" x2=\"100\" y1=\"120\" y2=\"160\"/>\n<line stroke=\
            \"#000\" stroke-width=\"5\" x1=\"70\" x2=\"100\" y1=\"120\" y2=\"140\"/>\n<line stroke=\
            \"#000\" stroke-width=\"5\" x1=\"130\" x2=\"100\" y1=\"120\" y2=\"140\"/>\n<line stroke=\
            \"#000\" stroke-width=\"5\" x1=\"70\" x2=\"100\" y1=\"200\" y2=\"160\"/>\n<line stroke=\
            \"#000\" stroke-width=\"5\" x1=\"130\" x2=\"100\" y1=\"200\" y2=\"160\"/>\n{}",
            START_SVG, x, y, r, END_SVG
        );
        assert_eq!(svg.to_string(), expected_svg);
    }

}


