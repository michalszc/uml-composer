use svg::node::{
    element::{
        SVG, Definitions, Line, Marker, Polygon, Text as TextElement
    },
    Text
};

pub fn draw_line(x1: usize, y1: usize, x2: usize, y2: usize, svg: &mut SVG, label: String) {
    let arrowhead = Marker::new()
        .set("id", "arrowhead")
        .set("markerWidth", "5")
        .set("markerHeight", "5")
        .set("refX", "0")
        .set("refY", "3.5")
        .set("orient", "auto")
        .add(
            Polygon::new()
                .set("points", "-5 1.5, 0 3.5, -5 5.5")
        );

    let mut line = Line::new()
        .set("x1", x1)
        .set("y1", y1)
        .set("x2", x2)
        .set("y2", y2)
        .set("stroke", "#000")
        .set("stroke-width", 8);

    line = line.set("marker-end", "url(#arrowhead)");
    let defs = Definitions::new().add(arrowhead);
    *svg = svg.clone().add(defs);

    // Calculate the angle of the line
    let angle = ((y2 as f32 - y1 as f32) / (x2 as f32 - x1 as f32)).atan();

    // Calculate the center point of the line
    let center_x = (x1 + x2) / 2;
    let center_y = (y1 + y2) / 2;

    let text = Text::new(label.as_str());

    // Create a text element
    let text_element = TextElement::new()
        .set("x", center_x - 20)
        .set("y", center_y - 20)
        .set("text-anchor", "middle")
        .set("dominant-baseline", "central")
        .set("fill", "black")
        .set("font-size", 25)
        .set("transform",
             format!("rotate({} {} {})", angle.to_degrees(), center_x, center_y))
        .add(text);

    *svg = svg.clone().add(text_element);

    *svg = svg.clone().add(line);
}

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Type {
    STEP,
    IF,
    START,
    END
}
