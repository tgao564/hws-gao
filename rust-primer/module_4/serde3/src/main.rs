use solution::{University,
               serialize_struct_to_jsonstring,
               deserialize_jsonstring_to_struct,
               serialize_struct_to_cbor,
               deserialize_struct_from_cbor};


fn main() {
    let json_string = r#"
        {
            "name": "University of Chicago",
            "undergraduate_enrollment": 7559,
            "graduate_enrollment": 10893,
            "schools": [
                "Biological Sciences Division",
                "Chicago Booth School of Business",
                "Crown Family School of Social Work, Policy, and Practice",
                "Divinity School",
                "Graham School of Continuing Liberal and Professional Studies",
                "Harris School of Public Policy",
                "Humanities Division",
                "Law School",
                "Physical Sciences Division",
                "Pritzker School of Medicine",
                "Pritzker School of Molecular Engineering",
                "Social Sciences Division"
            ],
            "acceptance_rate": 0.07
        }"#;

    // convert json to struct
    let uchicago: University = deserialize_jsonstring_to_struct(json_string);
    println!("{:?}", uchicago);

    // convert struct to json
    let serialized = serialize_struct_to_jsonstring(&uchicago);
    println!("serialized = {}", serialized);

    let filename = "uchicago.cbor";

    serialize_struct_to_cbor(&uchicago, filename);

    let uchicago_from_cbor: University = deserialize_struct_from_cbor(filename);
    println!("{:?}", uchicago_from_cbor);
}
