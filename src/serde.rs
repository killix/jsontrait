#[macro_export]
macro_rules! json_encodable_serde {
    ($t:ty) => {
        impl jsontrait::JsonEncodable for $t {
            fn json_to_string(&self) -> Result<String, jsontrait::Error> {
                use serde_json;
                serde_json::to_string(self).map_err(|e| jsontrait::Error::new(e))
            }
        }
    }
}

#[macro_export]
macro_rules! json_decodable_serde {
    ($t:ty) => {
        impl jsontrait::JsonDecodable for $t {
            fn json_from_str(s: &str) -> Result<Self, jsontrait::Error> {
                use serde_json;
                serde_json::from_str(s).map_err(|e| jsontrait::Error::new(e))
            }
        }
    }
}
