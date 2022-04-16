use anyhow::Error;

pub(crate) trait Parse {
    fn parse(input: Option<&json::Value>) -> anyhow::Result<Self>
    where
        Self: Sized;
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
