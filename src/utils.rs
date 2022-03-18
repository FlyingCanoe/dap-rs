use std::collections::HashMap;

use anyhow::Error;
use fallible_iterator::{convert, FallibleIterator};
use serde_json as json;

pub(crate) trait Parse {
    fn parse(input: Option<&json::Value>) -> anyhow::Result<Self>
    where
        Self: Sized;
}

impl Parse for bool {
    fn parse(input: Option<&json::Value>) -> anyhow::Result<bool> {
        input
            .ok_or(Error::msg("parsing error"))?
            .as_bool()
            .ok_or(Error::msg("parsing error"))
    }
}

impl Parse for u64 {
    fn parse(input: Option<&json::Value>) -> anyhow::Result<u64> {
        input
            .ok_or(Error::msg("parsing error"))?
            .as_u64()
            .ok_or(Error::msg("parsing error"))
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

impl Parse for json::Value {
    fn parse(input: Option<&json::Value>) -> anyhow::Result<json::Value> {
        let output = input.ok_or(Error::msg("parsing error"))?;
        Ok(output.clone())
    }
}

impl<L, R> Parse for either::Either<L, R>
where
    L: Parse + Sized,
    R: Parse + Sized,
{
    fn parse(input: Option<&json::Value>) -> anyhow::Result<Self> {
        if let Ok(output) = L::parse(input) {
            Ok(either::Left(output))
        } else {
            Ok(either::Right(R::parse(input)?))
        }
    }
}

impl<T> Parse for Option<T>
where
    T: Parse,
{
    fn parse(input: Option<&json::Value>) -> anyhow::Result<Option<T>>
where {
        if let Some(value) = input {
            let output = T::parse(Some(value))?;
            Ok(Some(output))
        } else {
            Ok(None)
        }
    }
}

impl<T> Parse for Vec<T>
where
    T: Parse,
{
    fn parse(input: Option<&json::Value>) -> anyhow::Result<Vec<T>> {
        let iter = input
            .ok_or(Error::msg("parsing error"))?
            .as_array()
            .ok_or(Error::msg("parsing error"))?
            .into_iter()
            .map(|value| T::parse(Some(value)));

        let output: Vec<_> = convert(iter).collect()?;
        Ok(output)
    }
}

impl<V> Parse for HashMap<String, V>
where
    V: Parse + Sized,
{
    fn parse(input: Option<&json::Value>) -> anyhow::Result<Self> {
        let map = input
            .ok_or(Error::msg("parsing error"))?
            .as_object()
            .ok_or(Error::msg("parsing error"))?;

        let iter = map
            .iter()
            .map(|(key, value)| -> anyhow::Result<(String, V)> {
                anyhow::Result::Ok((key.clone(), V::parse(Some(value))?))
            });
        let output: HashMap<String, V> = convert(iter).collect()?;
        Ok(output)
    }
}
