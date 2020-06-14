use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Result;

use crate::elements;

pub struct Dom {
    pub title: String,
    pub root: elements::Element
}

#[derive(Serialize, Deserialize)]
pub struct Document {
    pub title: String,
    pub root: DocumentElement 
}

#[derive(Serialize, Deserialize)]
pub struct DocumentElement {
    pub elType: String,
    pub value: String,
    pub children: Vec<DocumentElement>
}

impl Dom {

    pub fn create(document: Document) -> Dom {

        // Load the document 

        let mut sections = Vec::new();

        // Add each section, 

        // Do this recursively.
        for elem in document.root.children {
            
            let section = create_child(elem);

            //let section = elements::Element::Section(elem.value, children);

            sections.push(section);
        }

        let root = elements::Element::Root(sections);

        let title = format!("{}","Hello World!");

        Dom { title, root }
    }
}

fn create_child(child: DocumentElement) -> elements::Element {

    // Create the children
    let mut children = Vec::new();

    for c in child.children {
        let cc = create_child(c);
        children.push(cc);
    }
    
    match &child.elType[..] {
        "Root" => create_header_1(),
        "Section" => create_header_1(),
        "Header1" => create_header_1(),
        "Header2" => create_header_2(),
        "Header3" => create_header_3(),
        "Header4" => create_header_4(),
        "Header5" => create_header_5(),
        "Header6" => create_header_6(),
        "Paragraph" => create_paragraph(),
        "Span" => create_span(),
        "OrderedList" => create_ordered_list(),
        "UnorderedList" => create_unordered_list(),
        "ListItem" => create_list_item(),
        "Image" => create_image(),
        "Link" => create_link(),
        "Table" => create_table(),
        "TableColumn" => create_table_column(),
        "TableRow" => create_table_row(),
        "TableCell" => create_table_cell(),
        _ => panic!("Error: Unknown type!")
    }
} 

fn create_header_1() -> elements::Element {
    elements::Element::Header1
}

fn create_header_2() -> elements::Element {
    elements::Element::Header2
}

fn create_header_3() -> elements::Element {
    elements::Element::Header3
}

fn create_header_4() -> elements::Element {
    elements::Element::Header4
}

fn create_header_5() -> elements::Element {
    elements::Element::Header5
}

fn create_header_6() -> elements::Element {
    elements::Element::Header6
}

fn create_paragraph() -> elements::Element {
    elements::Element::Paragraph
}

fn create_span() -> elements::Element {
    elements::Element::Span
}

fn create_ordered_list() -> elements::Element {
    elements::Element::OrderedList
}

fn create_unordered_list() -> elements::Element {
    elements::Element::UnorderedList
}

fn create_list_item() -> elements::Element {
    elements::Element::ListItem
}

fn create_image() -> elements::Element {
    elements::Element::Image
}

fn create_link() -> elements::Element {
    elements::Element::Link
}

fn create_table() -> elements::Element {
    elements::Element::Table
}

fn create_table_column() -> elements::Element {
    elements::Element::TableColumn
}

fn create_table_row() -> elements::Element {
    elements::Element::TableRow
}

fn create_table_cell() -> elements::Element {
    elements::Element::TableCell
}

impl Document {
    pub fn load(path: String) -> Document {
        let mut file = File::open(path).expect("Couldn't read file");
        let mut contents = String::new();

        file.read_to_string(&mut contents);

        let document: Document = serde_json::from_str(&mut contents).unwrap();

        document
    }
}