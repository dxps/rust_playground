use std::collections::HashMap;

// A nice example to test this implementation is to use this query string in the request:
// a=1&b=2&c&d=&e===&d=7&d=abc
// Example: http://localhost:8080/testqs?a=1&b=2&c&d=&e===&d=7&d=abc

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }
            data.entry(key)
                .and_modify(|existing_val: &mut Value| match existing_val {
                    Value::Single(prev_value) => {
                        *existing_val = Value::Multiple(vec![prev_value, val]);
                    }
                    Value::Multiple(vec) => vec.push(val),
                })
                .or_insert(Value::Single(val));
        }
        QueryString { data }
    }
}
