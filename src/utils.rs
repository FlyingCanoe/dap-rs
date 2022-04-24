use std::collections::HashMap;

use anyhow::Error;

pub(crate) trait Parse {
    fn parse(input: Option<&json::Value>) -> anyhow::Result<Self>
    where
        Self: Sized;
}

impl Parse for u64 {
    fn parse(input: Option<&json::Value>) -> anyhow::Result<u64> {
        input
            .ok_or(Error::msg("parsing error"))?
            .as_u64()
            .ok_or(Error::msg("parsing error"))
    }
}

impl Parse for bool {
    fn parse(input: Option<&json::Value>) -> anyhow::Result<bool> {
        let output = input
            .ok_or(Error::msg("parsing error"))?
            .as_bool()
            .ok_or(Error::msg("parsing error"))?
            .to_owned();

        Ok(output)
    }
}

impl Parse for String {
    fn parse(input: Option<&json::Value>) -> anyhow::Result<String> {
        let output = input
            .ok_or(Error::msg("parsing error"))?
            .as_str()
            .ok_or(Error::msg("parsing error"))?
            .to_owned();

        Ok(output)
    }
}

impl<T: Parse> Parse for Option<T> {
    fn parse(input: Option<&json::Value>) -> anyhow::Result<Option<T>> {
        if let Some(input) = input {
            let output = T::parse(Some(input))?;
            Ok(Some(output))
        } else {
            Ok(None)
        }
    }
}

pub(crate) trait ToValue {
    fn to_value(self) -> Option<json::Value>
    where
        Self: Sized;
}

impl ToValue for u64 {
    fn to_value(self) -> Option<json::Value> {
        Some(self.into())
    }
}

impl ToValue for bool {
    fn to_value(self) -> Option<json::Value> {
        Some(self.into())
    }
}

impl ToValue for String {
    fn to_value(self) -> Option<json::Value> {
        Some(self.into())
    }
}

impl<T: ToValue> ToValue for Option<T> {
    fn to_value(self) -> Option<json::Value> {
        if let Some(value) = self {
            value.to_value()
        } else {
            None
        }
    }
}

impl<T: ToValue> ToValue for Vec<T> {
    fn to_value(self) -> Option<json::Value> {
        let mut value_vec = vec![];
        for elm in self.into_iter() {
            value_vec.push(elm.to_value()?)
        }

        let value = value_vec.into_iter().collect();
        Some(value)
    }
}

impl<V: ToValue> ToValue for HashMap<String, V> {
    fn to_value(self) -> Option<json::Value> {
        use json::Value;

        let map: json::Map<String, Value> = self
            .into_iter()
            .filter_map(|(key, value)| {
                if let Some(value) = value.to_value() {
                    Some((key, value))
                } else {
                    None
                }
            })
            .collect();

        Some(map.into())
    }
}
