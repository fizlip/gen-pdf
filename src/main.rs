use genpdf::{elements, style};
use std::convert::TryFrom;
use genpdf::fonts::Font;
use genpdf::Element;
use genpdf::Alignment;
use std::fs;
use serde::Deserialize;
use genpdf::Document;
use regex::Regex;
use genpdf::style::Color;
use genpdf::elements::Image;

use std::path::Path;
use std::process;
use aws_sdk_s3::{ByteStream, Client};
use std::fs::File;
use std::io::prelude::*;

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
struct SpekterDocument{
    Rows: Vec<Vec<Block>>
}

fn read_json() -> SpekterDocument {
    let json_file = fs::read_to_string("./test-real.json").expect("Could not read file");

    let json: SpekterDocument = serde_json::from_str(&json_file)
        .expect("JSON not well formatted");

    json

}

fn get_color(color: &str) -> Color {
    match color {
        "red" =>  return Color::Rgb(255, 0, 0),
        "blue" => return Color::Rgb(0, 0, 255),
        "black" => return Color::Rgb(0,0,0),
        "white" => return Color::Rgb(255, 255, 255),
        "green" => return Color::Rgb(0, 255, 0),
        "yellow" => return Color::Rgb(255, 255, 0),
        "purple" => return Color::Rgb(170, 0, 255),
        "pink" => return Color::Rgb(255, 0, 120),
        _ => return Color::Rgb(0, 0, 0)
    }
}

/**
 * apply_css_styling will return a paragraph object that is styled according 
 * to relevant css rules
 *
**/
fn apply_css_styling(mut p: elements::Paragraph, attrs: Vec<&str>, vals: Vec<&str>) -> elements::StyledElement<elements::Paragraph> {

    let mut style = style::Style::new();

    let mut i = 0;

    for attr in attrs {
        match attr {
            "font-size" => style.set_font_size(vals[i][0..vals[i].len()-2].parse::<u8>().unwrap()-4),
            "color" => style.set_color(get_color(vals[i])),
            "font-style" => style.set_italic(),
            "font-weight" => style.set_bold(),
            "text-align" => p.set_alignment(genpdf::Alignment::Center),
            _ => (),
        };
        i += 1;

    }

    let paragraph = p.styled(style);

    paragraph
}

fn parse_blocks(mut doc: Document, json:SpekterDocument) -> Document {
    let attrs_regex = Regex::new(r"(?<attr>[a-zA-Z0-9_%-]*?):").unwrap();
    let vals_regex  = Regex::new(r"(?<val>[a-zA-Z0-9_%-]*?);").unwrap();

    for r in json.Rows.iter() {
        let column_weights: Vec<usize> = r.iter().map(|_| 1).collect();
        let mut table = elements::TableLayout::new(column_weights);
        let mut has_border:bool = false;

        let mut row = table.row();

        for c in r.iter() {
            
            if c.t == "text" {

                let mut column_content = elements::TableLayout::new(vec![1]);

                for content in c.content.to_string().split("\n") {

                    let mut row_col = column_content.row();

                    let mut paragraph:elements::Paragraph = elements::Paragraph::default()
                                    .string(" ".to_owned() + content);

                    if c.content.to_string() == "<br>" {
                        paragraph = elements::Paragraph::default()
                            .string("");
                    }


                    let html = &c.raw;

                    let attrs: Vec<&str> = attrs_regex
                        .find_iter(html)
                        .map(|m| { 
                            let s = m.as_str();
                            let s = &s[0.. s.len()-1];
                            s
                        }).collect();

                    let vals: Vec<&str>  = vals_regex
                        .find_iter(html)
                        .map(|m| {
                            let s = m.as_str();
                            let s = &s[0.. s.len()-1];
                            s
                        }).collect();

                    let paragraph = apply_css_styling(paragraph, attrs.clone(), vals);

                    if(content.len() > 10) {
                        has_border = attrs.contains(&"border")
                            || attrs.contains(&"border-bottom")
                            || attrs.contains(&"border-top")
                            || attrs.contains(&"border-right")
                            || attrs.contains(&"border-left");
                    }

                    row_col.push_element(paragraph);
                    row_col.push();
                }
                row.push_element(column_content);
            }

            if c.t == "image" {
                let image = elements::Image::from_path("/home/filip/Dokument/lawgpt/gen-pdf/tmp/sofie.png")
                .expect("Failed to load image")
                .with_alignment(genpdf::Alignment::Center)
                .with_scale(genpdf::Scale::new(0.25, 0.25));

                row.push_element(image);

            }

        }

        row.push().expect("Invalid table row");

        table.set_cell_decorator(
            elements::FrameCellDecorator::new(has_border, has_border, false)
        );

        let b = genpdf::elements::Break::new(5);

        doc.push(table);
    }

    doc
}

async fn generate_pdf() {
    // Create
    let font_family = genpdf::fonts::from_files("./fonts", "Roboto", None)
            .expect("Failed to load font family");
    let mut doc = genpdf::Document::new(font_family);
    doc.set_title("Demo doc");

    //// Customize page
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(20);

    doc.set_page_decorator(decorator);


    let json:SpekterDocument = read_json();
    doc = parse_blocks(doc, json);

    doc.render_to_file("output.pdf").expect("Failed to write PDF file");

    let config = aws_config::load_from_env().await;
    println!("{:?}", config);
    let client = Client::new(&config);
    let file = ByteStream::from_path(Path::new("./output.pdf")).await;
    let bucket = "f4-public";
    let key = "output.pdf";

    let mut resp;

    match file {
        Ok(f) => {
            resp = client
                .put_object()
                .bucket(bucket)
                .key(key)
                .body(f)
                .send()
                .await;
        },
        Err(e) => {
            println!("Error uploading file {:?}", e);
        }
    };

    println!("File rendered to output.pdf");

}

#[tokio::main]
async fn main() {
    let res = generate_pdf().await;
}
