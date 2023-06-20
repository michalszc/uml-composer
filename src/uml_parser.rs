use std::collections::{HashMap, HashSet};
use std::{str, fs};
use std::process::Command;
use pest::Parser;
use crate::rules::link::Link;
use crate::rules::structs::Class;
use crate::grammar_parser::{GrammarParser, Rule};
use crate::rules::actor::Actor;
use crate::rules::context::Context;
use crate::rules::activity::Activity;
use svg::{Document, node::element::SVG};

pub struct UmlParser {

}

enum DiagramType {
    ClassDiagram,
    UseCaseDiagram,
}


impl UmlParser {
    fn add_alias(list: &mut HashSet<String>, alias: String) {
        if list.contains(&alias) {
            panic!("Duplicate name/alias found: {}", alias);
        } else {
            list.insert(alias);
        }
    }
    fn check_alias(list: &HashSet<String>, alias: String) {
        if !list.contains(&alias) {
            panic!("Alias not found: {}", alias);
        }
    }
    pub fn parse(value: &str) {
        let mut svg;
        let mut diagram = DiagramType::UseCaseDiagram;

        let mut contexts: Vec<Context> = Vec::new();
        let mut actors: Vec<Actor> = Vec::new();
        let mut links: Vec<Link> = Vec::new();
        let mut classes: Vec<Class> = Vec::new();
        let mut aliases: HashSet<String> = HashSet::new();

        let _actor_length;
        let _context_length;
        let _links_length;
        let mut _use_cases_len = 0;
        let _classes_length;

        let program = GrammarParser::parse(Rule::PROGRAM, value)
            .unwrap_or_else(|e| panic!("{}", e))
            .next().unwrap();

        for pair in program.into_inner() {
            match pair.as_rule() {
                Rule::CLASS_DIAGRAM => {
                    diagram = DiagramType::ClassDiagram;
                    for inner_pair in pair.into_inner() {
                        match inner_pair.as_rule() {
                            Rule::start_class => {}
                            Rule::CLASS => {
                                let class = Class::new(inner_pair, false);
                                UmlParser::add_alias(&mut aliases, class.get_name().clone());
                                classes.push(class);
                            }
                            Rule::INTERFACE => {
                                let interface = Class::new(inner_pair, true);
                                UmlParser::add_alias(&mut aliases, interface.get_name().clone());
                                classes.push(interface);
                            }
                            Rule::LINK => {
                                let link = Link::new(inner_pair);
                                links.push(link);
                            }
                            _ => unreachable!()
                        }
                    }
                }
                Rule::USE_CASE_DIAGRAM => {
                    diagram = DiagramType::UseCaseDiagram;
                    for inner_pair in pair.into_inner() {
                        match inner_pair.as_rule() {
                            Rule::start_use_case => {}

                            Rule::CONTEXT => {
                                let context = Context::new(inner_pair);
                                UmlParser::add_alias(&mut aliases, context.get_context_label().clone());
                                _use_cases_len += context.get_use_cases().len();
                                contexts.push(context);
                            }

                            Rule::ACTOR => {
                                let actor = Actor::new(inner_pair);
                                UmlParser::add_alias(&mut aliases, actor.get_actor_alias().clone());
                                actors.push(actor);
                            }
                            Rule::LINK => {
                                let link = Link::new(inner_pair);
                                links.push(link);
                            }
                            _ => unreachable!()
                        }
                    }
                }
                Rule::ACTIVITY_DIAGRAM => {
                    for inner_pair in pair.into_inner(){
                        match inner_pair.as_rule() {
                            Rule::start_activity => println!("{:?}", inner_pair),
                            Rule::ACTIVITY_BODY => {
                                Activity::new(inner_pair).draw(&mut svg);
                            }
                            _ => unreachable!()
                        }
                    }
                }
                Rule::end_uml => {}
                _ => unreachable!()
            }
        }
        let initial_height:usize = 500;
        let initial_width:usize = 500;
        let mut width = initial_width as usize;
        let height;

        match diagram {
            DiagramType::ClassDiagram => {
                _classes_length = classes.len();
                _links_length = links.len();
                let class_size:usize = 600; // default class box size
                let mut index = 0; // used to generate X,Y coordinates of the class
                width = class_size * 5;
                height = class_size * ((_classes_length + 1) / 2);

                svg = SVG::new().set("viewBox", format!("0 0 {} {}", width, height));
                let mut y_column1 = 25; // Y coordinate of classes that appear in the first column
                let mut y_column2 = 50; // Y coordinate of classes that appear in the second column

                let mut index_class_links: HashMap<String, i32> = HashMap::new(); // number of a link in a list of linked links to the class
                let mut max_index_class_links: HashMap<String, i32> = HashMap::new(); // number of links linked to the class
                for class in &mut classes {
                    max_index_class_links.insert(class.get_name().to_string(), 0);
                    index_class_links.insert(class.get_name().to_string(), 0);
                }

                // count the number of links linked to the classes
                for link in &links {
                    if let Some(value) = max_index_class_links.get_mut(link.get_left_id()) {
                        *value += 1;
                    }
                    if let Some(value) = max_index_class_links.get_mut(link.get_right_id()) {
                        *value += 1;
                    }
                }

                // generate X,Y coordinates of classes and draw them
                for class in &mut classes {
                    if index % 2 == 0 {
                        class.draw(&mut svg, 50, y_column1);
                        y_column1 = y_column1 + (100 + class.get_height());
                    }
                    else {
                        class.draw(&mut svg, 3 * class_size + 50, y_column2);
                        y_column2 = y_column2 + (100 + class.get_height());
                    }
                    index += 1;
                }
                let mut link_index= 0; // used to generate X coordinate of middle of the link

                // generate connections of links and their X,Y coordinates
                for link in &mut links {
                    let mut x1 = 0;
                    let mut y1 = 0;
                    let mut x2 = 0;
                    let mut y2 = 0;
                    for class in &mut classes {
                        if class.get_name() == link.get_left_id(){
                            x1 = class.get_x().clone() as i32;
                            if x1 < 900 {x1 = x1 + class.get_width().clone() as i32;}
                            if let Some(value) = index_class_links.get_mut(class.get_name()) {
                                if let Some(value2) = max_index_class_links.get_mut(class.get_name()) {
                                    *value += 1;
                                    y1 = class.get_y().clone() as i32 + *value * (class.get_height() - 10) as i32 / *value2;
                                }
                            }
                        }
                        if class.get_name() == link.get_right_id(){
                            x2 = class.get_x().clone() as i32;
                            if x2 < 900 {x2 = x2 + class.get_width().clone() as i32;}

                            if let Some(value) = index_class_links.get_mut(class.get_name()) {
                                if let Some(value2) = max_index_class_links.get_mut(class.get_name()) {
                                    *value += 1;
                                    y2 = class.get_y().clone() as i32 + *value * (class.get_height() - 10) as i32 / *value2;
                                }
                            }
                        }
                    }
                    if y1 == 0 || y2 == 0 {continue;} // non-existent link
                    link.draw_class_link(&mut svg, x1, y1, x2, y2, (900 + 600 * link_index / _links_length) as i32);

                    link_index += 1;
                }
            }
            DiagramType::UseCaseDiagram => {
                _context_length = contexts.len();
                _actor_length = actors.len();

                let actor_size: i32 = 20;
                let contest_height = 350;
                let mut contest_width ;
                height = std::cmp::max((contest_height + 100) * _context_length,
                                           std::cmp::max(initial_height as usize, (_actor_length * actor_size as usize * 8) as usize));

                let all_actors_height = (8 * actor_size as i32 * _actor_length as i32) as i32;
                let x_actor: i32 = 75;
                let mut y_actor = (initial_height as i32 - all_actors_height) / 2 + actor_size as i32;
                let mut y_context = std::cmp::min(y_actor - actor_size, 100); // display first context a little above first actor

                let mut _left_id: String; // left id in link
                let mut _right_id: String; // right id in link
                let mut left_x: i32 = -1; // x coordinate of left side of link
                let mut left_y: i32 = -1; // y coordinate of left side of link
                let mut right_x: i32 = -1; // x coordinate of right side of link
                let mut right_y: i32 = -1; // y coordinate of right side of link

                let mut max_width; // number of dependent use_cases in a context
                let mut change; // auxiliary variable
                let mut index;
                let mut used_links: HashMap<i32, i32> = (1..=links.len()).map(|key| (key as i32, 0)).collect();

                // calculate the maximum width of connected use_cases and column of dependent use_cases
                // context
                // |----------------------------------|
                // u1 ---link1---> u2 ---link2---> u3
                // u4 ---link3---> u5
                // u6
                // |----------------------------------|

                for context in &mut contexts{
                    max_width = 1;
                    let mut modifications = Vec::new();
                    for use_case in context.get_use_cases(){
                        UmlParser::add_alias(&mut aliases, use_case.get_use_case_alias().clone());
                        contest_width = 1;
                        _left_id = use_case.get_use_case_alias().clone();
                        loop {
                            change = false;
                            index = 0;
                            for link in &mut links{
                                if let Some(value) = used_links.get_mut(&index) {
                                    if link.get_left_id().as_str() == _left_id && *value == 0{
                                        contest_width += 1;
                                        *value = 1;
                                        modifications.push((link.get_right_id().clone(), contest_width));
                                        change = true;
                                        _left_id = link.get_right_id().clone();
                                    }
                                }
                                index += 1;
                            }
                            if change{
                                if contest_width > max_width{
                                    max_width = contest_width;
                                }
                                _left_id = use_case.get_use_case_alias().clone();
                                contest_width = 1;
                                continue;
                            }
                            break;
                        }
                    }
                    // assign column to the use cases
                    for use_case2 in context.get_use_cases_mut() {
                        for (right_id, width) in &modifications {
                            if use_case2.get_use_case_alias().as_str() == right_id && use_case2.get_width_number() < *width {
                                use_case2.set_width_number(*width);
                            }
                        }
                    }
                    width = std::cmp::max(width, (initial_width + 350 * (max_width - 1) as usize) as usize); // set width of svg viewBox according to widest context
                    // change the width according to the width of use_cases in the context, 200 value is temporary
                    context.set_width_number(max_width);
                }
                for link in &mut links {
                    UmlParser::check_alias(&aliases, link.get_left_id().clone());
                    UmlParser::check_alias(&aliases, link.get_right_id().clone());
                }


                // create ready svg
                svg = SVG::new().set("viewBox", format!("0 0 {} {}", width + 10, height))
                    .set("style", "background-color: green");

                for context in &mut contexts {
                    context.draw(&mut svg, 2 * x_actor, y_context, 350 * context.get_width_number(), 350);
                    y_context += 50 + contest_height as i32;
                }

                for actor in &mut actors {
                    actor.draw(&mut svg, x_actor, y_actor, actor_size);
                    y_actor += 8 * actor_size;
                }

                for link in links {
                    let _left_id = link.get_left_id().clone();
                    let _right_id = link.get_right_id().clone();

                    // match _left_id to possible actor
                    for actor in &actors {
                        if actor.get_actor_alias().as_str() == _left_id {
                            left_x = actor.get_x() + actor_size;
                            left_y = actor.get_y() + 2 * actor_size;
                        }
                    }
                    // match _left_id or _right_id to possible use_case
                    for context in &contexts {
                        for use_case in context.get_use_cases() {
                            if use_case.get_use_case_alias().as_str() == _left_id {
                                left_x = use_case.get_x() + use_case.get_width() / 2;
                                left_y = use_case.get_y();
                            }
                            if use_case.get_use_case_alias().as_str() == _right_id {
                                right_x = use_case.get_x() - use_case.get_width() / 2;
                                right_y = use_case.get_y();
                            }
                        }
                    }
                    // check if _left_id and _right_id exists
                    if !(left_x == -1 || left_y == -1 || right_x == -1 || right_y == -1) {
                        link.draw(&mut svg, left_x, left_y, right_x, right_y);
                    }
                }
            }
        }
        svg::save("image.svg", &svg).unwrap();
        let output = Command::new("rsvg-convert")
            .arg("-w")
            .arg(format!("{width}"))
            .arg("-h")
            .arg(format!("{height}"))
            .arg("-f")
            .arg("png")
            .arg("-o")
            .arg("output.png")
            .arg("image.svg")
            .output()
            .expect("Failed to execute command.");
        match fs::remove_file("image.svg") { // removed unnecessary file
            Ok(()) => tracing::info!("File image.svg removed successfully."),
            Err(err) => tracing::info!("Failed to remove the file: {err}"),
        }
        if output.status.success() {
            tracing::info!("Command executed successfully!");
        } else {
            let error_message = str::from_utf8(&output.stderr).unwrap_or("Unknown error");
            tracing::error!("Command failed with error code: {}", output.status);
            tracing::error!("Error message: {}", error_message);
        }
    }
}
