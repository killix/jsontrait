#[macro_use]
extern crate jsontrait;
extern crate rustc_serialize;

#[test]
fn encode_custom_type() {

    #[derive(Debug, RustcEncodable)]
    struct Foo {
        bar: i32,
        qux: String,
    }

    json_encodable_rustc_serialize!(Foo);

    let x = Foo {
        bar: 42,
        qux: "ipsum lorem".to_string(),
    };

    {
        use jsontrait::JsonEncodable;

        let expected = rustc_serialize::json::encode(&Foo {
                           bar: 42,
                           qux: "ipsum lorem".to_string(),
                       })
                           .unwrap();
        let got = x.json_to_string().unwrap();
        assert_eq!(expected, got);
    }
}

#[test]
fn decode_custom_type() {

    #[derive(Debug, Eq, PartialEq, RustcDecodable)]
    struct Foo {
        bar: i32,
        qux: String,
    }

    json_decodable_rustc_serialize!(Foo);

    {
        use jsontrait::JsonDecodable;

        let expected = Foo {
            bar: 42,
            qux: "ipsum lorem".to_string(),
        };
        let s = r#"{"bar": 42, "qux": "ipsum lorem"}"#;
        let got = Foo::json_from_str(&s).unwrap();
        assert_eq!(expected, got);
    }
}
