#[macro_use]
extern crate jsontrait;
extern crate serde;
extern crate serde_json;

#[test]
fn encode_custom_type() {

    #[derive(Debug)]
    struct Foo {
        bar: i32,
        qux: String,
    }

    impl serde::Serialize for Foo {
        fn serialize<S: serde::Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
            struct Visitor<'a>(&'a Foo);

            impl<'a> serde::ser::MapVisitor for Visitor<'a> {
                fn visit<S: serde::Serializer>(&mut self,
                                               s: &mut S)
                                               -> Result<Option<()>, S::Error> {
                    let Visitor(x) = *self;
                    try!(s.visit_struct_elt("bar", &x.bar));
                    try!(s.visit_struct_elt("qux", &x.qux));
                    Ok(None)
                }
            }

            s.visit_struct("Foo", Visitor(self))
        }
    }

    json_encodable_serde!(Foo);

    {
        use jsontrait::JsonEncodable;

        let x = Foo {
            bar: 42,
            qux: "ipsum lorem".to_string(),
        };

        let expected = serde_json::to_string(&serde_json::builder::ObjectBuilder::new()
                                                  .insert("bar", 42)
                                                  .insert("qux", "ipsum lorem")
                                                  .unwrap())
                           .unwrap();
        let got = x.json_to_string().unwrap();
        assert_eq!(expected, got);
    }
}

#[test]
fn decode_custom_type() {

    #[derive(Debug, Eq, PartialEq)]
    struct Foo {
        bar: i32,
        qux: String,
    }

    impl serde::Deserialize for Foo {
        fn deserialize<D>(d: &mut D) -> Result<Self, D::Error>
            where D: serde::Deserializer
        {
            enum Field {
                Bar,
                Qux,
            }

            impl serde::Deserialize for Field {
                fn deserialize<D>(d: &mut D) -> Result<Self, D::Error>
                    where D: serde::Deserializer
                {

                    struct Visitor;

                    impl serde::de::Visitor for Visitor {
                        type Value = Field;

                        fn visit_str<E>(&mut self, value: &str) -> Result<Self::Value, E>
                            where E: serde::de::Error
                        {
                            match value {
                                "bar" => Ok(Field::Bar),
                                "qux" => Ok(Field::Qux),
                                _ => Err(E::unknown_field(value)),
                            }
                        }
                    }

                    d.visit(Visitor)
                }
            }

            struct Visitor;

            impl serde::de::Visitor for Visitor {
                type Value = Foo;

                fn visit_map<V>(&mut self, mut visitor: V) -> Result<Foo, V::Error>
                    where V: serde::de::MapVisitor
                {
                    let mut bar = None;
                    let mut qux = None;

                    loop {
                        match try!(visitor.visit_key()) {
                            Some(Field::Bar) => {
                                bar = Some(try!(visitor.visit_value()));
                            }
                            Some(Field::Qux) => {
                                qux = Some(try!(visitor.visit_value()));
                            }
                            None => {
                                break;
                            }
                        }
                    }

                    try!(visitor.end());

                    let bar = match bar {
                        Some(bar) => bar,
                        None => try!(visitor.missing_field("bar")),
                    };

                    let qux = match qux {
                        Some(qux) => qux,
                        None => try!(visitor.missing_field("qux")),
                    };

                    Ok(Foo {
                        bar: bar,
                        qux: qux,
                    })
                }
            }

            static FIELDS: &'static [&'static str] = &["bar", "qux"];
            d.visit_struct("Foo", FIELDS, Visitor)
        }
    }

    json_decodable_serde!(Foo);

    {
        use jsontrait::JsonDecodable;

        let expected = Foo {
            bar: 42,
            qux: "ipsum lorem".to_string(),
        };
        let s = serde_json::to_string(&serde_json::builder::ObjectBuilder::new()
                                           .insert("bar", 42)
                                           .insert("qux", "ipsum lorem")
                                           .unwrap())
                    .unwrap();
        let got = Foo::json_from_str(&s).unwrap();
        assert_eq!(expected, got);
    }
}
