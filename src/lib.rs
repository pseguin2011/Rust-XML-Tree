use xml::reader::{XmlEvent, Events};
use xml::EventReader;
use std::io::{BufReader, Read};
use std::collections::HashMap;

#[derive(Debug)]
pub struct XMLElement {
    value: Option<String>,
    inner_elements: HashMap<String, XMLElement>,
}

impl XMLElement {
    pub fn get_value(&self) -> Option<String> {
        self.value.clone()
    }
    pub fn get(&self, key: &str) -> Option<&XMLElement> {
        self.inner_elements.get(key)
    }
}

pub fn parse_xml_content(xml_content: &str) -> HashMap<String, XMLElement>{
    let buffer = BufReader::new(xml_content.as_bytes());
    let parser = EventReader::new(buffer);
    let mut xml = parser.into_iter();
    let mut xml_elements: HashMap<String, XMLElement> = HashMap::new();
    while let Some(Ok(current)) = xml.next() {
        if let Some(first_tag) = is_starting_tag(&current) {
            xml_elements.insert(first_tag.into(), get_next_tag(&mut xml, first_tag));
        }
    }
    xml_elements
}

fn get_next_tag<'a, R: Read>(
    mut xml: &mut Events<R>,
    searched_tag: &str,
) -> XMLElement {
    let mut inner_elements: HashMap<String, XMLElement> = HashMap::new();
    let mut value: Option<String> = None;
    while let Some(Ok(current)) = xml.next() {
        if let xml::reader::XmlEvent::Characters(v) = &current {
            value = Some(v.into());
        } else if let Some(item) = is_starting_tag(&current) {
            inner_elements.insert(item.into(), get_next_tag(&mut xml, item));
        }
        if is_ending_tag(searched_tag, &current) {
            break;
        }
    }
    XMLElement {value, inner_elements}
}


fn is_starting_tag(element: &XmlEvent) -> Option<&str> {
    match element {
        XmlEvent::StartElement {
            name: xml::name::OwnedName {
                local_name: tag, ..
            },
            ..
        } => Some(tag),
        _ => None,
    }
}

fn is_ending_tag(tag_name: &str, element: &XmlEvent) -> bool {
    match element {
        XmlEvent::EndElement {
            name: xml::name::OwnedName {
                local_name: tag, ..
            },
            ..
        } => tag == tag_name,
        _ => false,
    }
}