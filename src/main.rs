use genpdf::{elements, style};
use genpdf::fonts::Font;
use genpdf::Element;

fn main() {

    // Create
    let font_family = genpdf::fonts::from_files("./fonts", "Roboto", None)
            .expect("Failed to load font family");
    let mut doc = genpdf::Document::new(font_family);
    doc.set_title("Demo doc");

    // Customize page
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);

    doc.set_page_decorator(decorator);

    let mut table = elements::TableLayout::new(vec![1,1]);
    table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));
    let mut row = table.row();
    row.push_element(
        elements::Paragraph::default()
            .string("Fullmakts-givare")
            .styled(style::Style::new().with_font_size(8)),
    );
    row.push_element(
        elements::Paragraph::default()
            .string("Namn")
            .styled(style::Style::new().with_font_size(8)),
    );
    row.push().expect("Invalid table row");

    doc.push(table);

    doc.render_to_file("output.pdf").expect("Failed to write PDF file");

    println!("Hello, world!");

}
