use rust_xml_tree::parse_xml_content;

#[test]
fn verify_xml_structure_maintained() {
    let xml = "\
    <I>\
        <J>\
            <L>Q</L>\
            <H>B</H>\
        </J>\
    </I>";
    let result = parse_xml_content(xml);
    assert_eq!(
        Some(String::from("Q")),
        result
            .get("I")
            .unwrap()
            .get("J")
            .unwrap()
            .get("L")
            .unwrap()
            .get_value()
    );
    assert_eq!(
        Some(String::from("B")),
        result
            .get("I")
            .unwrap()
            .get("J")
            .unwrap()
            .get("H")
            .unwrap()
            .get_value());
}

#[test]
fn verify_confusing_xml() {
    let xml = "\
    <I>\
        <I>\
            <I>\
                <J>Q</J>
            </I>\
            <J>B</J>\
        </I>\
    </J>";
    let result = parse_xml_content(xml);
    assert_eq!(
        Some(String::from("Q")),
        result
            .get("I")
            .unwrap()
            .get("I")
            .unwrap()
            .get("I")
            .unwrap()
            .get("J")
            .unwrap()
            .get_value()
    );
    assert_eq!(
        Some(String::from("B")),
        result
            .get("I")
            .unwrap()
            .get("I")
            .unwrap()
            .get("J")
            .unwrap()
            .get_value());
}