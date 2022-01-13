#[cfg(feature = "rpc")]
use crate::Error;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "rpc")]
use serde_value::Value;
#[cfg(feature = "rpc")]
use std::collections::HashMap;

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Eq, PartialEq)]
pub struct ClientInfo<'a> {
    pub name: &'a str,
    pub tp: &'a str,
    pub source: Option<&'a str>,
    pub port: Option<&'a str>,
}
impl<'a> Ord for ClientInfo<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(other.name)
    }
}
impl<'a> PartialOrd for ClientInfo<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct ClientList<'a> {
    #[cfg_attr(feature = "serialization", serde(borrow))]
    pub clients: Vec<ClientInfo<'a>>,
}

#[allow(clippy::ptr_arg)]
#[cfg(feature = "rpc")]
pub fn str_to_params_map<'a>(s: &'a Vec<&str>) -> Result<HashMap<&'a str, Value>, Error> {
    let mut params: HashMap<&str, Value> = HashMap::new();
    for pair in s {
        if !pair.is_empty() {
            let mut psp = pair.split('=');
            let var = psp
                .next()
                .ok_or_else(|| Error::data("var name not specified"))?;
            let v = psp
                .next()
                .ok_or_else(|| Error::data("var value not specified"))?;
            let value = if v == "false" {
                Value::Bool(false)
            } else if v == "true" {
                Value::Bool(true)
            } else if let Ok(i) = v.parse::<i64>() {
                Value::I64(i)
            } else if let Ok(f) = v.parse::<f64>() {
                Value::F64(f)
            } else {
                Value::String(v.to_owned())
            };
            params.insert(var, value);
        }
    }
    Ok(params)
}