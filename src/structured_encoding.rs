use std::collections::BTreeMap;

pub trait StructuredEncoding {
    fn encode(&self) -> Vec<String>;
    fn encoded_string(&self) -> String {
        let mut acc = String::new();
        for s in self.encode() {
            acc.push_str(&s);
            acc.push('&');
        }
        acc.pop();
        acc
    }
}

impl StructuredEncoding for BTreeMap<String, String> {
    fn encode(&self) -> Vec<String> {
        self.into_iter().fold(Vec::new(), |mut vec, (key, value)| {
            vec.push(format!("{}={}", key, value));
            vec
        })
    }
}
