use std::collections::BTreeMap;
use std::fmt::Display;

pub trait UrlEncodable {
    fn key_value_pairs(&self) -> Vec<(String, String)>;
    fn encoded_string(&self) -> String {
        let mut s = String::new();
        for (k, v) in self.key_value_pairs() {
            s.push_str(&format!("{}={}", k, v));
            s.push('&');
        }
        if s.ends_with('&') { s.pop(); }
        s
    }
}

//TODO move all of these to CallArgs
impl UrlEncodable {
    pub fn list<T, S>(list_name: S, list: &Vec<T>) -> Vec<(String, String)>
        where T: Display, S: Display
    {
        let mut vec = vec![];
        if !list.is_empty() {
            for t in list {
                vec.push((format!("{}[]", list_name), t.to_string()));
            }
        } else {
            // Stripe uses 'name[]=' to indicate an empty list
            vec.push((format!("{}[]", list_name), "".to_string()));
        }
        vec
    }

    pub fn named<T, S>(structure_key: S, value: &T) -> Vec<(String, String)>
        where T: UrlEncodable, S: Display
    {
        let mut vec = vec![];
        for (k, v) in value.key_value_pairs() {
            vec.push((format!("{}[{}]", structure_key, k), v));
        }
        vec
    }

    pub fn structured<T, S>(dict: &BTreeMap<S, T>) -> Vec<(String, String)>
        where T: UrlEncodable, S: Display
    {
        let mut vec = vec![];
        for (k_outer, t) in dict {
            vec.extend(UrlEncodable::named(k_outer, t));
        }
        vec
    }

    pub fn structured_list<T, S>(list_name: S, list: Vec<T>) -> Vec<(String, String)>
        where T: UrlEncodable, S: Display
    {
        let mut vec = vec![];
        if !list.is_empty() {
            for t in list {
                for (k, v) in t.key_value_pairs() {
                    vec.push((format!("{}[][{}]", list_name, k), v));
                }
            }
        } else {
            // Stripe uses 'name[]=' to indicate an empty list
            vec.push((format!("{}[]", list_name), "".to_string()));
        }
        vec
    }
}

impl<'a> UrlEncodable for &'a UrlEncodable {
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        (*self).key_value_pairs()
    }
}

impl<K, V> UrlEncodable for BTreeMap<K, V>
    where K: Display, V: Display
{
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        self.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }
}

impl UrlEncodable for (String, String) {
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        let (ref k, ref v) = *self;
        vec![(k.to_string(), v.to_string())]
    }
}

impl<'a> UrlEncodable for (&'a str, String) {
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        let (ref k, ref v) = *self;
        vec![(k.to_string(), v.to_string())]
    }
}

impl<'a, 'b> UrlEncodable for (&'a str, &'b str) {
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        let (ref k, ref v) = *self;
        vec![(k.to_string(), v.to_string())]
    }
}

impl<T: UrlEncodable> UrlEncodable for Option<T> {
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        match *self {
            Some(ref t) => t.key_value_pairs(),
            None        => vec![]
        }
    }
}

impl<V: Display> UrlEncodable for (&'static str, Option<V>) {
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        match *self {
            (key, Some(ref v)) => vec![(key.to_string(), v.to_string())],
            (_,   None)        => vec![]
        }
    }
}

impl<'a, V: Display> UrlEncodable for (&'static str, &'a Option<V>) {
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        match *self {
            (key, &Some(ref v)) => vec![(key.to_string(), v.to_string())],
            (_,   &None)        => vec![]
        }
    }
}

impl UrlEncodable for () {
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        Vec::new()
    }
}

impl<T: UrlEncodable> UrlEncodable for Vec<T> {
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        let mut vec = vec![];
        for t in self {
            vec.extend(t.key_value_pairs());
        }
        vec
    }
}

impl<A, B> UrlEncodable for (A, B)
    where A: UrlEncodable,
          B: UrlEncodable
{
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        let (ref a, ref b) = *self;
        let mut vec = a.key_value_pairs();
        vec.extend(b.key_value_pairs());
        vec
    }
}

impl<A, B, C> UrlEncodable for (A, B, C)
    where A: UrlEncodable,
          B: UrlEncodable,
          C: UrlEncodable,
{
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        let (ref a, ref b, ref c) = *self;
        let mut vec = a.key_value_pairs();
        vec.extend(b.key_value_pairs());
        vec.extend(c.key_value_pairs());
        vec
    }
}

impl<A, B, C, D> UrlEncodable for (A, B, C, D)
    where A: UrlEncodable,
          B: UrlEncodable,
          C: UrlEncodable,
          D: UrlEncodable,
{
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        let (ref a, ref b, ref c, ref d) = *self;
        let mut vec = a.key_value_pairs();
        vec.extend(b.key_value_pairs());
        vec.extend(c.key_value_pairs());
        vec.extend(d.key_value_pairs());
        vec
    }
}

impl<A, B, C, D, E> UrlEncodable for (A, B, C, D, E)
    where A: UrlEncodable,
          B: UrlEncodable,
          C: UrlEncodable,
          D: UrlEncodable,
          E: UrlEncodable,
{
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        let (ref a, ref b, ref c, ref d, ref e) = *self;
        let mut vec = a.key_value_pairs();
        vec.extend(b.key_value_pairs());
        vec.extend(c.key_value_pairs());
        vec.extend(d.key_value_pairs());
        vec.extend(e.key_value_pairs());
        vec
    }
}

impl<A, B, C, D, E, F> UrlEncodable for (A, B, C, D, E, F)
    where A: UrlEncodable,
          B: UrlEncodable,
          C: UrlEncodable,
          D: UrlEncodable,
          E: UrlEncodable,
          F: UrlEncodable,
{
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        let (ref a, ref b, ref c, ref d, ref e, ref f) = *self;
        let mut vec = a.key_value_pairs();
        vec.extend(b.key_value_pairs());
        vec.extend(c.key_value_pairs());
        vec.extend(d.key_value_pairs());
        vec.extend(e.key_value_pairs());
        vec.extend(f.key_value_pairs());
        vec
    }
}

impl<A, B, C, D, E, F, G> UrlEncodable for (A, B, C, D, E, F, G)
    where A: UrlEncodable,
          B: UrlEncodable,
          C: UrlEncodable,
          D: UrlEncodable,
          E: UrlEncodable,
          F: UrlEncodable,
          G: UrlEncodable,
{
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        let (ref a, ref b, ref c, ref d, ref e, ref f, ref g) = *self;
        let mut vec = a.key_value_pairs();
        vec.extend(b.key_value_pairs());
        vec.extend(c.key_value_pairs());
        vec.extend(d.key_value_pairs());
        vec.extend(e.key_value_pairs());
        vec.extend(f.key_value_pairs());
        vec.extend(g.key_value_pairs());
        vec
    }
}

impl<A, B, C, D, E, F, G, H> UrlEncodable for (A, B, C, D, E, F, G, H)
    where A: UrlEncodable,
          B: UrlEncodable,
          C: UrlEncodable,
          D: UrlEncodable,
          E: UrlEncodable,
          F: UrlEncodable,
          G: UrlEncodable,
          H: UrlEncodable,
{
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        let (ref a, ref b, ref c, ref d, ref e, ref f, ref g, ref h) = *self;
        let mut vec = a.key_value_pairs();
        vec.extend(b.key_value_pairs());
        vec.extend(c.key_value_pairs());
        vec.extend(d.key_value_pairs());
        vec.extend(e.key_value_pairs());
        vec.extend(f.key_value_pairs());
        vec.extend(g.key_value_pairs());
        vec.extend(h.key_value_pairs());
        vec
    }
}
