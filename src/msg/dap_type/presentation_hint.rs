use anyhow::bail;

#[derive(Clone, Debug)]
pub enum PresentationHint {
    Normal,
    Emphasize,
    Deemphasize,
}

impl PresentationHint {
    pub(crate) fn parse(input: &str) -> anyhow::Result<PresentationHint> {
        let hint = match input {
            "normal" => PresentationHint::Normal,
            "emphasize" => PresentationHint::Emphasize,
            "deemphasize" => PresentationHint::Deemphasize,
            _ => bail!("invalid field"),
        };
        Ok(hint)
    }
}
