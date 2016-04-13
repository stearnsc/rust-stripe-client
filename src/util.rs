use std::collections::BTreeMap;

pub fn structured(name: &str, map: BTreeMap<String, String>) -> BTreeMap<String, String> {
    map.into_iter()
        .map(|(k, v)| (format!("{}[{}]", name, k), v.clone()))
        .collect::<BTreeMap<_, _>>()
}

pub fn or_join<C, A>(a: Option<C>, b: Option<C>) -> Option<C>
    where C: Extend<A> + IntoIterator<Item=A>
{
    match (a, b) {
        (Some(mut a), Some(b)) => {
            a.extend(b);
            Some(a)
        },
        (a@Some(_), None)  => a,
        (None, b@Some(_))  => b,
        (None, None)       => None
    }
}
