use genpdf::{elements, style};
use genpdf::fonts::Font;
use genpdf::Element;
use genpdf::Alignment;
use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct StyleElement {
    cssName: String,
    value: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DocStyle {
    font_size: StyleElement,
    padding_left: StyleElement,
    padding_right: StyleElement,
    font_family: StyleElement,
    color: StyleElement,
    width: StyleElement,
    background: StyleElement
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Block {
    content: String,
    media: String,
    id: i64,
    t: String,
    render: String,
    raw: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Row {
    Blocks: Vec<Block>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Document {
    Rows: Vec<Row>
}

fn read_json() -> Document {
    let json_file = fs::read_to_string("./test2.json").expect("Could not read file");

    let json: Document = serde_json::from_str(&json_file)
        .expect("JSON not well formatted");

    json

}

fn parse_blocks(json:Document) {
}

fn main() {

    let doc:Document = read_json();

    // Create
    //let font_family = genpdf::fonts::from_files("./fonts", "Roboto", None)
    //        .expect("Failed to load font family");
    //let mut doc = genpdf::Document::new(font_family);
    //doc.set_title("Demo doc");

    //// Customize page
    //let mut decorator = genpdf::SimplePageDecorator::new();
    //decorator.set_margins(50);

    //doc.set_page_decorator(decorator);

    //let mut table = elements::TableLayout::new(vec![1,1]);
    //table.set_cell_decorator(elements::FrameCellDecorator::new(false, false, false));
    //let mut row = table.row();
    //row.push_element(
    //    elements::Paragraph::default()
    //        .string("Fullmakts-givare")
    //        .styled(style::Style::new().with_font_size(8)),
    //);
    //row.push_element(
    //    elements::Paragraph::default()
    //        .aligned(Alignment::Right)
    //        .string("Namn")
    //        .styled(style::Style::new().with_font_size(12)),
    //);
    //row.push().expect("Invalid table row");

    //doc.push(table);

    //doc.render_to_file("output.pdf").expect("Failed to write PDF file");

    println!("Hello, world!");

}
