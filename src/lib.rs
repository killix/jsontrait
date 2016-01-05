extern crate rustc_serialize as rustc_serialize_crate;
extern crate serde as serde_crate;
extern crate serde_json;

mod rustc_serialize;
mod serde;

// FIXME: Add special support for serde and rustc_serialize?
#[derive(Debug)]
pub struct Error<'a>(Box<std::error::Error + 'a>);

impl<'a> Error<'a> {
    pub fn new<E: 'a + std::error::Error>(e: E) -> Self {
        let x = Box::new(e);
        Error(x)
    }
}

impl<'a> std::error::Error for Error<'a> {
    fn description(&self) -> &str {
        let Error(ref e) = *self;
        e.description()
    }

    fn cause(&self) -> Option<&std::error::Error> {
        let Error(ref e) = *self;
        e.cause()
    }
}

impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let Error(ref e) = *self;
        e.fmt(f)
    }
}

pub trait JsonEncodable {
    fn json_to_string(&self) -> Result<String, Error>;
// fn json_to_writer(&self, writer: &mut Writer) -> Result<usize, Error>;
}

pub trait JsonDecodable: Sized {
    fn json_from_str(string: &str) -> Result<Self, Error>;
// fn json_from_reader(reader: &mut Reader) -> Result<Self, Error>;
}
