#[macro_export]
macro_rules! json_encodable_rustc_serialize {
    ($t:ty) => {
        impl jsontrait::JsonEncodable for $t {
            fn json_to_string(&self) -> Result<String, jsontrait::Error> {
                use rustc_serialize;
                rustc_serialize::json::encode(self).map_err(|e| jsontrait::Error::new(e))
            }
        }
    }
}

#[macro_export]
macro_rules! json_decodable_rustc_serialize {
    ($t:ty) => {
        impl jsontrait::JsonDecodable for $t {
            fn json_from_str(s: &str) -> Result<Self, jsontrait::Error> {
                use rustc_serialize;
                rustc_serialize::json::decode(s).map_err(|e| jsontrait::Error::new(e))
            }
        }
    }
}
