use anyhow::Error;

pub(crate) trait Parse {
    fn parse(input: Option<&json::Value>) -> anyhow::Result<Self>
    where
        Self: Sized;
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
