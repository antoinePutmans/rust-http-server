use std::collections::HashMap;
use std::fmt::Debug;

use crate::http::query_string::Value::{Multiple, Single};

#[derive(Debug)]
pub struct QueryString<'buff> {
    data: HashMap<&'buff str, Value<'buff>>,
}

#[derive(Debug)]
pub enum Value<'buff> {
    Single(&'buff str),
    Multiple(Vec<&'buff str>),
}

impl<'buff> QueryString<'buff> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buff> From<&'buff str> for QueryString<'buff> {
    fn from(value: &'buff str) -> Self {
        let mut data = HashMap::new();

        for sub_str in value.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            data.entry(key)
                .and_modify(|cur: &mut Value| match cur {
                    Single(prev_val) => {
                        *cur = Multiple(vec![prev_val, val]);
                    }
                    Multiple(vec) => vec.push(val)
                })
                .or_insert(Single(val));
        }

        QueryString { data }
    }
}