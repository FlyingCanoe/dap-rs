use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug, Clone)]
struct Ast {
    name: String,
    field_list: Vec<Field>,
}

#[derive(Debug, Clone)]
struct Field {
    comment: String,
    name: String,
    field_type: String,
}

fn pre_process(input: &str) -> String {
    let input = input
        .replace("{", "")
        .replace("}", "")
        .replace(";", "")
        .replace("string", "String")
        .replace("number", "u64")
        .replace("boolean", "bool");
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut line = line.to_string();
            line.push('\n');
            line
        })
        .collect()
}

fn post_process_comment(comment: &str) -> String {
    comment.replace("* ", "/// ").trim().to_string()
}

fn parse_struct_name(input: &str) -> (&str, &str) {
    let (_, rem) = input.split_once("interface").unwrap();
    let name = rem.split_whitespace().next().unwrap();
    let (_, rem) = rem.split_once(name).unwrap();

    (name, rem)
}

fn parse_field(input: &str) -> Option<(Field, &str)> {
    let (_, rem) = input.split_once("/**")?;
    let (comment, rem) = rem.split_once("*/\n")?;

    let (field, rem) = if let Some((field, rem)) = rem.split_once('\n') {
        (field, rem)
    } else {
        if !rem.trim().is_empty() {
            (rem, "")
        } else {
            return None;
        }
    };

    if field.contains("[]") {
        let field = field.replace("[]", "");
        if field.contains("?:") {
            let (name, field_type) = field.split_once("?:").unwrap();
            let field_type = field_type.trim();

            let name = name.to_owned();
            let field_type = format!("Option<Vec<{field_type}>>");

            let comment = post_process_comment(comment);
            let field = Field {
                comment,
                name,
                field_type,
            };
            Some((field, rem))
        } else {
            let (name, field_type) = field.split_once(":").unwrap();
            let field_type = field_type.trim();

            let name = name.to_owned();
            let field_type = format!("Vec<{field_type}>");

            let comment = post_process_comment(comment);
            let field = Field {
                comment,
                name,
                field_type,
            };
            Some((field, rem))
        }
    } else {
        if field.contains("?:") {
            let (name, field_type) = field.split_once("?:").unwrap();
            let field_type = field_type.trim();

            let name = name.to_owned();
            let field_type = format!("Option<{field_type}>");

            let comment = post_process_comment(comment);
            let field = Field {
                comment,
                name,
                field_type,
            };
            Some((field, rem))
        } else {
            let (name, field_type) = field.split_once(":").unwrap();
            let field_type = field_type.trim();

            let name = name.to_owned();
            let field_type = field_type.to_owned();

            let comment = post_process_comment(comment);
            let field = Field {
                comment,
                name,
                field_type,
            };
            Some((field, rem))
        }
    }
}

fn parse(input: &str) -> Ast {
    let input = pre_process(input);
    let (name, mut rem) = parse_struct_name(&input);

    let mut field_list: Vec<Field> = Vec::new();
    while let Some((field, rem2)) = parse_field(rem) {
        rem = rem2;
        //println!("{:?}: {:?}", filed_type, field);
        field_list.push(field);
    }
    //println!("{:?}", field_list.entry(FieldType::OptionalU64Vec));
    Ast {
        name: name.to_owned(),
        field_list,
    }
}

impl Ast {
    fn to_string(self) -> String {
        let field_declaration: String = self
            .field_list
            .iter()
            .map(|field| format!("{}\n{}:{},", field.comment, field.name, field.field_type))
            .collect();

        let field_parsing: String = self
            .field_list
            .iter()
            .map(|field| match field.field_type.as_str() {
                "String" => format!(r#"let {0} = parse_string(input.get("{0}"))?;"#, field.name),
                "Option<String>" => format!(
                    r#"let {0} = parse_optional_string(input.get("{0}"))?;"#,
                    field.name
                ),
                "bool" => format!(r#"let {0} = parse_bool(input.get("{0}"))?;"#, field.name),
                "Option<bool>" => format!(
                    r#"let {0} = parse_optional_bool(input.get("{0}"))?;"#,
                    field.name
                ),
                "u64" => format!(r#"let {0} = parse_u64(input.get("{0}"))?;"#, field.name),
                "Option<u64>" => format!(
                    r#"let {0} = parse_optional_u64(input.get("{0}"))?;"#,
                    field.name
                ),
                "Option<Vec<u64>>" => format!(
                    r#"let {0} = parse_optional_u64_vec(input.get("{0}"))?;"#,
                    field.name
                ),
                "Vec<String>" => format!(
                    r#"let {0} = parse_string_vec(input.get("{0}"))?;"#,
                    field.name
                ),
                _ => format!(r#"let {0} = todo!();"#, field.name),
            })
            .collect();

        let field_init: String = self
            .field_list
            .iter()
            .map(|field| format!("{},", field.name))
            .collect();

        format!(
            r#"
use serde_json as json;
use anyhow::Error;
use fallible_iterator::{{convert, FallibleIterator}};

use crate::utils::{{parse_u64, parse_optional_u64, parse_string, parse_bool, parse_optional_bool, parse_optional_string, parse_optional_u64_vec, parse_string_vec}};


#[derive(Debug, Clone)]
pub struct {0} {{
{field_declaration}
}}

impl {0} {{
    pub(crate) fn parse(input: Option<&json::Value>) -> anyhow::Result<{0}> {{
        let input = input.ok_or(Error::msg("parsing error"))?;
        {field_parsing}

        let output = {0} {{
            {field_init}
        }};
        Ok(output)
    }}

    pub(crate) fn parse_option(input: Option<&json::Value>) -> anyhow::Result<Option<{0}>> {{
        if input.is_some() {{
            let output = {0}::parse(input)?;
            Ok(Some(output))
        }} else {{
            Ok(None)
        }}
    }}

    pub(crate) fn parse_vec(input: Option<&json::Value>) -> anyhow::Result<Vec<{0}>> {{
        let input = input.ok_or(Error::msg("parsing error"))?;
        let iter =  input.as_array().ok_or(Error::msg("parsing error"))?.iter().map(|value| {0}::parse(Some(value)));
        let output: Vec<_> = convert(iter).collect()?;
        Ok(output)
    }}

        pub(crate) fn parse_optional_vec(
            input: Option<&json::Value>,
        ) -> anyhow::Result<Option<Vec<{0}>>> {{
            if input.is_some() {{
                let output = {0}::parse_vec(input)?;
                Ok(Some(output))
            }} else {{
                Ok(None)
            }}
        }}
}}
"#,
            self.name
        )
    }
}

fn main() {
    let mut file = File::open("./input.txt").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let ast = parse(&buf);
    let output = ast.to_string();
    let mut file = File::create("./output.rs").unwrap();
    file.write_all(output.as_bytes()).unwrap();
}
