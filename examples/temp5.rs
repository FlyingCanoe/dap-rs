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
    wire_name: String,
    field_type: String,
}

fn pre_process(input: &str) -> String {
    let input = input
        .replace("{", "")
        .replace("}", "")
        .replace(";", "")
        .replace("string", "String")
        .replace("number", "u64")
        .replace("boolean", "bool")
        .replace("Arguments", "Request");
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
    comment
        .replace("* ", "/// ")
        .trim()
        .to_string()
        .lines()
        .map(|line| format!("\t\t{}\n", line))
        .collect::<String>()
        .trim_end()
        .to_owned()
}

fn post_process_name(name: &str) -> String {
    let mut rem = name;
    let mut part_list = Vec::new();
    while !rem.is_empty() {
        let mut part = rem.chars().next().unwrap().to_string();
        part.push_str(
            rem.chars()
                .skip(1)
                .take_while(|ch| ch.is_lowercase())
                .collect::<String>()
                .as_str(),
        );
        rem = &rem[part.len()..];
        part_list.push(part);
    }
    let mut output = String::new();

    for part in part_list {
        output.push_str(&part.to_ascii_lowercase());
        output.push_str("_");
    }
    output.pop();
    output
}

fn parse_struct_name(input: &str) -> (String, &str) {
    let (_, rem) = input.split_once("interface").unwrap();
    let name = rem.split_whitespace().next().unwrap();
    let name = post_process_name(name);

    (name, rem)
}

fn parse_field(input: &str) -> Option<(Field, &str)> {
    let (_, rem) = input.split_once("/**")?;
    let (comment, rem) = rem.split_once("*/\n")?;

    let (name, wire_name, rem) = if let Some((field, rem)) = rem.split_once('\n') {
        (post_process_name(field), field, rem)
    } else {
        if !rem.trim().is_empty() {
            (rem.to_string(), rem, "")
        } else {
            return None;
        }
    };

    if wire_name.contains("[]") {
        let name = name.replace("[]", "");
        let wire_name = wire_name.replace("[]", "");
        if wire_name.contains("?:") {
            let (name, _) = name.split_once("_?_:").unwrap();
            let (wire_name, field_type) = wire_name.split_once("?:").unwrap();
            let field_type = field_type.trim();

            let name = name.to_owned();
            let wire_name = wire_name.to_owned();
            let field_type = format!("Option<Vec<{field_type}>>");

            let comment = post_process_comment(comment);
            let field = Field {
                comment,
                name,
                wire_name,
                field_type,
            };
            Some((field, rem))
        } else {
            let (name, field_type) = name.split_once(":").unwrap();
            let field_type = field_type.trim();

            let name = name.to_owned();
            let field_type = format!("Vec<{field_type}>");

            let comment = post_process_comment(comment);
            let field = Field {
                comment,
                wire_name,
                name,
                field_type,
            };
            Some((field, rem))
        }
    } else {
        if wire_name.contains("?:") {
            let (name, _) = name.split_once("_?_:").unwrap();
            let (wire_name, field_type) = wire_name.split_once("?:").unwrap();
            let field_type = field_type.trim();

            let name = name.to_owned();
            let wire_name = wire_name.to_owned();

            let field_type = format!("Option<{field_type}>");

            let comment = post_process_comment(comment);
            let field = Field {
                comment,
                name,
                wire_name,
                field_type,
            };
            Some((field, rem))
        } else {
            let (name, _) = name.split_once(":").unwrap();
            let (wire_name, field_type) = wire_name.split_once(":").unwrap();
            let field_type = field_type.trim();

            let name = name.to_owned();
            let wire_name = wire_name.to_owned();

            let field_type = field_type.to_owned();

            let comment = post_process_comment(comment);
            let field = Field {
                comment,
                name,
                wire_name,
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
            .map(|field| match field.field_type.as_str() {
                "bool" => format!(
                    "{0}\n\t\t{1} | \"{2}\": {3} = parse_bool,\n\n",
                    field.comment, field.name, field.wire_name, field.field_type
                ),
                "Option<bool>" => format!(
                    "{0}\n\t\t{1} | \"{2}\": {3} = parse_optional_bool,\n\n",
                    field.comment, field.name, field.wire_name, field.field_type
                ),
                "u64" => format!(
                    "{0}\n\t\t{1} | \"{2}\": {3} = parse_u64,\n\n",
                    field.comment, field.name, field.wire_name, field.field_type
                ),
                "Option<u64>" => format!(
                    "{0}\n\t\t{1} | \"{2}\": {3} = parse_optional_u64,\n\n",
                    field.comment, field.name, field.wire_name, field.field_type
                ),
                "Option<Vec<u64>>" => format!(
                    "{0}\n\t\t{1} | \"{2}\": {3} = parse_optional_u64_vec,\n\n",
                    field.comment, field.name, field.wire_name, field.field_type
                ),
                "String" => format!(
                    "{0}\n\t\t{1} | \"{2}\": {3} = parse_string,\n\n",
                    field.comment, field.name, field.wire_name, field.field_type
                ),
                "Option<String>" => format!(
                    "{0}\n\t\t{1} | \"{2}\": {3} = parse_optional_string,\n\n",
                    field.comment, field.name, field.wire_name, field.field_type
                ),
                "Vec<String>" => format!(
                    "{0}\n\t\t{1} | \"{2}\": {3} = parse_string_vec,\n\n",
                    field.comment, field.name, field.wire_name, field.field_type
                ),
                _ if field.field_type.contains("Option<Vec") => {
                    let inner_type = field
                        .field_type
                        .split_once("Option<Vec<")
                        .unwrap()
                        .1
                        .split_once(">")
                        .unwrap()
                        .0;
                    format!(
                        "{0}\n\t\t{1} | \"{2}\": {3} = {4}::parse_optional_vec,\n\n",
                        field.comment, field.name, field.wire_name, field.field_type, inner_type
                    )
                }
                _ if field.field_type.contains("Option") => {
                    let inner_type = field
                        .field_type
                        .split_once("Option<")
                        .unwrap()
                        .1
                        .split_once(">")
                        .unwrap()
                        .0;
                    format!(
                        "{0}\n\t\t{1} | \"{2}\": {3} = {4}::parse_optional,\n\n",
                        field.comment, field.name, field.wire_name, field.field_type, inner_type
                    )
                }
                _ => format!(
                    "{0}\n\t\t{1} | \"{2}\": {3} = {3}::parse,\n\n",
                    field.comment, field.name, field.wire_name, field.field_type
                ),
            })
            .collect();

        format!(
            r#"
use crate::utils::{{parse_u64, parse_optional_u64, parse_string, parse_bool, parse_optional_bool, parse_optional_string, parse_optional_u64_vec, parse_string_vec}};

dap_type_struct!(
    {} {{
{}
    }}
);
"#,
            self.name, field_declaration
        )
        .replace("\n\n\n", "\n")
        .replace("_?_", "")
        .replace("?\"", "\"")
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
