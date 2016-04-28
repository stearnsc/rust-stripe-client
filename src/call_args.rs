use std::fmt::Display;
use url_encodable::UrlEncodable;

#[derive(Clone, Debug)]
pub struct CallArgs(pub Vec<(String, String)>);

impl CallArgs {
    pub fn new() -> Self {
        CallArgs(Vec::new())
    }

    pub fn add<T: UrlEncodable>(&mut self, t: T) {
        let CallArgs(ref mut args) = *self;
        args.extend(t.key_value_pairs());
    }

    pub fn add_arg<T: Display>(&mut self, key: &str, value: T) {
        self.add((key, value.to_string()));
    }

    pub fn add_named<T: UrlEncodable>(&mut self, name: &str, t: T) {
        self.add(UrlEncodable::named(name, &t));
    }
}

//TODO remove once client calls take owned CallArgs
impl UrlEncodable for CallArgs {
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        let CallArgs(ref args) = *self;
        args.clone()
    }
}

