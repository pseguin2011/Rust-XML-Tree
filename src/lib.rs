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
    /// Retrieves the value of the XMLElement in String format
    /// 
    /// ## Returns
    /// The value of the XMLElement if one exists
    pub fn get_value(&self) -> Option<String> {
        self.value.clone()
    }

    /// Searches through the inner elements of the current one and
    /// returns the `XMLElement` that matches the tag if one exists.
    /// 
    /// ## Arguments
    /// `tag` The XMLElement tag name that is being searched
    /// 
    /// ## Returns
    /// The `XMLElement` matching the tag name
    pub fn get(&self, tag: &str) -> Option<&XMLElement> {
        self.inner_elements.get(tag)
    }
}

/// When provided a String to parse, this function will create a buffered stream and
/// iterate through it parsing each found tag elements individually. until the end of the
/// content.
/// 
/// ## Arguments
/// `xml_content` - A valid string of XML tags
/// 
/// ## Returns
/// A HashMap of xml elements representing a tree of elements
pub fn parse_xml_content(xml_content: &str) -> HashMap<String, XMLElement>{
    let buffer = BufReader::new(xml_content.as_bytes());
    let parser = EventReader::new(buffer);
    let mut xml = parser.into_iter();
    let mut xml_elements: HashMap<String, XMLElement> = HashMap::new();
    while let Some(Ok(current)) = xml.next() {
        if let Some(first_tag) = get_starting_tag(&current) {
            xml_elements.insert(first_tag.into(), get_next_tag(&mut xml, first_tag));
        }
    }
    xml_elements
}

/// Iterates through the inner elements of the XML content:
/// If a new starting element is found it will recursively call this function and parse that tag's elements.
/// If the element is actually a character value, we store it in the value of the element returned.
/// If the ending element is found for the current element being parsed, we have successfully parsed the element and
/// can break out of it.
/// 
/// ## Arguments
/// `xml` A mutable iterator containing XML elements
/// `searched_tag` The tag we are currently searching in
/// 
/// ## Returns
/// The currently being searched XML element
fn get_next_tag<R: Read>(
    mut xml: &mut Events<R>,
    searched_tag: &str,
) -> XMLElement {
    let mut inner_elements: HashMap<String, XMLElement> = HashMap::new();
    let mut value: Option<String> = None;
    while let Some(Ok(current)) = xml.next() {
        if let xml::reader::XmlEvent::Characters(v) = &current {
            value = Some(v.into());
        } else if let Some(item) = get_starting_tag(&current) {
            inner_elements.insert(item.into(), get_next_tag(&mut xml, item));
        }
        if is_ending_tag(searched_tag, &current) {
            break;
        }
    }
    XMLElement {value, inner_elements}
}

/// Verifies that the element provided is a starting tag
/// If it is, the function will return the tag name
/// Otherwise the `None` option will be returned
fn get_starting_tag(element: &XmlEvent) -> Option<&str> {
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

/// Verifies that the element provided is the ending element of the provided tag
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