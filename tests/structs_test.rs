#[cfg(test)]
mod structs_test {
    use pest::Parser;
    use uml_composer::{rules::structs::{Class, Visibility, Component}, grammar_parser::{GrammarParser, Rule}};

    #[test]
    fn parse_public_component() {
        let input: &str = "+name\n";
        let component = GrammarParser::parse(Rule::COMPONENT, input)
            .unwrap().next().unwrap();
        let new_component = Component::extract_attribute(component);
        assert_eq!(new_component.get_name().to_owned(),String::from("name"));
        assert_eq!(*new_component.get_visibility(), Visibility::PUBLIC);
        assert_eq!(new_component.get_kind().to_owned(), String::from(""));
    }

    #[test]
    fn parse_private_component() {
        let input: &str = "-name\n";
        let component = GrammarParser::parse(Rule::COMPONENT, input)
            .unwrap().next().unwrap();
        let new_component = Component::extract_attribute(component);
        assert_eq!(new_component.get_name().to_owned(),String::from("name"));
        assert_eq!(*new_component.get_visibility(), Visibility::PRIVATE);
        assert_eq!(new_component.get_kind().to_owned(), String::from(""));
    }

    #[test]
    fn parse_protected_component() {
        let input: &str = "#name\n";
        let component = GrammarParser::parse(Rule::COMPONENT, input)
            .unwrap().next().unwrap();
        let new_component = Component::extract_attribute(component);
        assert_eq!(new_component.get_name().to_owned(),String::from("name"));
        assert_eq!(*new_component.get_visibility(), Visibility::PROTECTED);
        assert_eq!(new_component.get_kind().to_owned(), String::from(""));
    }

    #[test]
    fn parse_public_component_with_type() {
        let input: &str = "+cost:double\n";
        let component = GrammarParser::parse(Rule::COMPONENT, input)
            .unwrap().next().unwrap();
        let new_component = Component::extract_attribute(component);
        assert_eq!(new_component.get_name().to_owned(),String::from("cost"));
        assert_eq!(*new_component.get_visibility(), Visibility::PUBLIC);
        assert_eq!(new_component.get_kind().to_owned(), String::from("double"));
    }

    #[test]
    fn parse_private_component_with_type() {
        let input: &str = "-cost:double\n";
        let component = GrammarParser::parse(Rule::COMPONENT, input)
            .unwrap().next().unwrap();
        let new_component = Component::extract_attribute(component);
        assert_eq!(new_component.get_name().to_owned(),String::from("cost"));
        assert_eq!(*new_component.get_visibility(), Visibility::PRIVATE);
        assert_eq!(new_component.get_kind().to_owned(), String::from("double"));
    }

    #[test]
    fn parse_protected_component_with_type() {
        let input: &str = "#cost:double\n";
        let component = GrammarParser::parse(Rule::COMPONENT, input)
            .unwrap().next().unwrap();
        let new_component = Component::extract_attribute(component);
        assert_eq!(new_component.get_name().to_owned(),String::from("cost"));
        assert_eq!(*new_component.get_visibility(), Visibility::PROTECTED);
        assert_eq!(new_component.get_kind().to_owned(), String::from("double"));
    }

    #[test]
    fn parse_class_with_only_components() {
        let input: &str = "class klasa1 {\n\tattributes {\n\t\t- atrybut_prywatny : typ\n\t\t+ atrybut_publiczny : typ\n\t\t# atrybut_chroniony : typ\n\t}\n}\n";
        let class = GrammarParser::parse(Rule::CLASS, input)
            .unwrap().next().unwrap();
        let new_class = Class::new(class.clone(), false);
        assert_eq!(new_class.get_keyword().to_owned(), String::from("class"));
        assert_eq!(new_class.get_name().to_owned(), String::from("klasa1"));
        assert_eq!(new_class.get_attributes().len(), 3);
        assert_eq!(new_class.get_methods().len(), 0)
    }

    #[test]
    fn parse_class_with_only_methods() {
        let input: &str = "class klasa1 {\n\tmethods {\n\t\t- metoda_prywatna : typ\n\t\t+ metoda_publiczna : typ\n\t\t# metoda_chroniona\n\t}\n}\n";
        let class = GrammarParser::parse(Rule::CLASS, input)
            .unwrap().next().unwrap();
        let new_class = Class::new(class.clone(), false);
        assert_eq!(new_class.get_keyword().to_owned(), String::from("class"));
        assert_eq!(new_class.get_name().to_owned(), String::from("klasa1"));
        assert_eq!(new_class.get_attributes().len(), 0);
        assert_eq!(new_class.get_methods().len(), 3);
    }

    #[test]
    fn parse_class_with_methods_and_attributes() {
        let input: &str = "class klasa1 {\n\tattributes {\n\t\t- atrybut_prywatny : typ\n\t\t+ atrybut_publiczny : typ\n\t\t# atrybut_chroniony : typ\n\t}\n\tmethods {\n\t\t- metoda_prywatna : typ\n\t\t+ metoda_publiczna : typ\n\t\t# metoda_chroniona\n\t}\n}\n";
        let class = GrammarParser::parse(Rule::CLASS, input)
            .unwrap().next().unwrap();
        let new_class = Class::new(class.clone(), false);
        assert_eq!(new_class.get_keyword().to_owned(), String::from("class"));
        assert_eq!(new_class.get_name().to_owned(), String::from("klasa1"));
        assert_eq!(new_class.get_attributes().len(), 3);
        assert_eq!(new_class.get_methods().len(), 3);
    }

    #[test]
    fn parse_interface_with_only_components() {
        let input: &str = "interface klasa1 {\n\tattributes {\n\t\t- atrybut_prywatny : typ\n\t\t+ atrybut_publiczny : typ\n\t\t# atrybut_chroniony : typ\n\t}\n}\n";
        let class = GrammarParser::parse(Rule::INTERFACE, input)
            .unwrap().next().unwrap();
        let new_class = Class::new(class.clone(), true);
        assert_eq!(new_class.get_keyword().to_owned(), String::from("interface"));
        assert_eq!(new_class.get_name().to_owned(), String::from("klasa1"));
        assert_eq!(new_class.get_attributes().len(), 3);
        assert_eq!(new_class.get_methods().len(), 0)
    }

    #[test]
    fn parse_interface_with_only_methods() {
        let input: &str = "interface klasa1 {\n\tmethods {\n\t\t- metoda_prywatna : typ\n\t\t+ metoda_publiczna : typ\n\t\t# metoda_chroniona\n\t}\n}\n";
        let class = GrammarParser::parse(Rule::INTERFACE, input)
            .unwrap().next().unwrap();
        let new_class = Class::new(class.clone(), true);
        assert_eq!(new_class.get_keyword().to_owned(), String::from("interface"));
        assert_eq!(new_class.get_name().to_owned(), String::from("klasa1"));
        assert_eq!(new_class.get_attributes().len(), 0);
        assert_eq!(new_class.get_methods().len(), 3);
    }

    #[test]
    fn parse_interface_with_methods_and_attributes() {
        let input: &str = "interface klasa1 {\n\tattributes {\n\t\t- atrybut_prywatny : typ\n\t\t+ atrybut_publiczny : typ\n\t\t# atrybut_chroniony : typ\n\t}\n\tmethods {\n\t\t- metoda_prywatna : typ\n\t\t+ metoda_publiczna : typ\n\t\t# metoda_chroniona\n\t}\n}\n";
        let class = GrammarParser::parse(Rule::INTERFACE, input)
            .unwrap().next().unwrap();
        let new_class = Class::new(class.clone(), true);
        assert_eq!(new_class.get_keyword().to_owned(), String::from("interface"));
        assert_eq!(new_class.get_name().to_owned(), String::from("klasa1"));
        assert_eq!(new_class.get_attributes().len(), 3);
        assert_eq!(new_class.get_methods().len(), 3);
    }
}
