use std::fs::File;
use std::io::{Read, Write};

struct Ast {
    variant_list: Vec<String>,
}

fn parse(input: String) -> Ast {
    let (_, rem) = input.split_once("'").unwrap();
    rem.rsplit_once("'");
    let variant_list = rem
        .replace("'", "")
        .replace(";", "")
        .split("|")
        .map(str::trim)
        .map(str::to_string)
        .collect();

    Ast { variant_list }
}

impl Ast {
    fn to_string(self) -> String {
        let variant_declaration: String = self
            .variant_list
            .clone()
            .into_iter()
            .map(|variant| format!("\t{variant},\n"))
            .collect();

        let variant_matching: String = self
            .variant_list
            .into_iter()
            .map(|variant| format!("\t\t\t\"{variant}\" => name::{variant},\n"))
            .collect();

        format!(
            r#"
use anyhow::{{bail, Error}};
use serde_json as json;
        
#[derive(Clone, Debug)]
pub enum name {{
{variant_declaration}
}}
                
impl name {{
    pub(crate) fn parse(input: Option<&json::Value>) -> anyhow::Result<name> {{
        let input = input.ok_or(Error::msg("parsing error"))?.as_str().ok_or(Error::msg("parsing error"))?;
        let output = match input {{
{variant_matching}
            _ => bail!("parsing error"),
        }};
        Ok(output)
    }}

    pub(crate) fn parse_option(input: Option<&json::Value>) -> anyhow::Result<Option<name>> {{
        if input.is_some() {{
            let output = name::parse(input)?;
            Ok(Some(output))
        }} else {{
            Ok(None)
        }}
    }}
}}"#
        )
        .to_string()
    }
}

fn main() {
    let mut file = File::open("./input.txt").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let ast = parse(buf);
    let output = ast.to_string();
    let mut file = File::create("./output.rs").unwrap();
    file.write_all(output.as_bytes()).unwrap();
}
