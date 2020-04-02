use xml::reader::{XmlEvent, Events};
use xml::EventReader;
use std::io::{BufReader, Read};
use std::collections::HashMap;

fn main() {
    let xml_str = "<Placemark><name>Dynamic Funds Tower</name><address>1 Adelaide St E, Toronto, ON M5C 2V9, Kanada</address><ExtendedData><Data name=\"Email\"><value>avsec05@gmail.com</value></Data><Data name=\"Category\"><value>Pisarna podjetja</value></Data><Data name=\"Distance\"><value>0</value></Data></ExtendedData><description>Pisarna podjetja from 2020-01-20T19:26:39.819Z to 2020-01-21T03:25:13.151Z. Distance 0m</description><Point><coordinates>-79.3778805,43.650278899999996,0</coordinates></Point><TimeSpan><begin>2020-01-20T19:26:39.819Z</begin><end>2020-01-21T03:25:13.151Z</end></TimeSpan></Placemark><Placemark><name>Walking</name><address></address><ExtendedData><Data name=\"Email\"><value>avsec05@gmail.com</value></Data><Data name=\"Category\"><value>Walking</value></Data><Data name=\"Distance\"><value>1619</value></Data></ExtendedData><description>Walking from 2020-01-21T03:25:13.151Z to 2020-01-21T03:49:27.566Z. Distance 1619m</description><LineString><altitudeMode>clampToGround</altitudeMode><extrude>1</extrude><tesselate>1</tesselate><coordinates>-79.3778805,43.650278899999996,0 -79.3778805,43.650278899999996,0 -79.3780702,43.6500914,0 -79.3792992,43.6497463,0 -79.3817811,43.659222,0 -79.378977,43.6597555,0 -79.3782056,43.6608412,0 -79.3777604,43.661101099999996,0 -79.3777604,43.661101099999996,0</coordinates></LineString><TimeSpan><begin>2020-01-21T03:25:13.151Z</begin><end>2020-01-21T03:49:27.566Z</end></TimeSpan></Placemark>";
    let buffer = BufReader::new(xml_str.as_bytes());
    let parser = EventReader::new(buffer);
    let mut xml = parser.into_iter();
    let mut xml_elements: HashMap<String, XMLElement> = HashMap::new();

    while let Some(Ok(current)) = xml.next() {
        if let Some(first_tag) = is_starting_tag(&current) {
            xml_elements.insert(first_tag.into(), get_next_tag(&mut xml, first_tag));
        }
    }

    // let element = xml_elements.get("Placemark").unwrap().get("name").unwrap().value;
    println!("{:?}", xml_elements);
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
        self.inner_elements.get(key.into())
    }
}


pub fn get_next_tag<'a, R: Read>(
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