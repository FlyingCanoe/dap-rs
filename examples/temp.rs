use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug, Clone)]
struct Ast {
    name: String,
    field_list: HashMap<FieldType, Vec<Field>>,
}

#[derive(Debug, Clone)]
struct Field {
    comment: String,
    name: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
enum FieldType {
    OptionalAny,
    OptionalU64,
    OptionalU64Vec,
    OptionalBool,
    OptionalString,
    U64,
    String,
}

fn pre_process(input: &str) -> String {
    let input = input
        .replace("{", "")
        .replace("}", "")
        .replace(";", "")
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
    comment.replace("* ", "/// ").trim().to_string()
}

fn parse_struct_name(input: &str) -> (&str, &str) {
    let (_, rem) = input.split_once("interface").unwrap();
    let name = rem.split_whitespace().next().unwrap();
    let (_, rem) = rem.split_once(name).unwrap();

    (name, rem)
}

fn parse_field(input: &str) -> Option<((FieldType, Field), &str)> {
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
        if field.contains("?:") {
            let (name, field_type) = field.split_once("?:").unwrap();
            let field_type = match field_type.trim() {
                "number[]" => FieldType::OptionalU64Vec,
                _ => panic!("{}", field_type.trim()),
            };

            let name = name.to_owned();
            let comment = post_process_comment(comment);
            let field = Field { comment, name };
            Some(((field_type, field), rem))
        } else {
            todo!()
        }
    } else {
        if field.contains("?:") {
            let (name, field_type) = field.split_once("?:").unwrap();
            let field_type = match field_type.trim() {
                "any" => FieldType::OptionalAny,
                "number" => FieldType::OptionalU64,
                "boolean" => FieldType::OptionalBool,
                "string" => FieldType::OptionalString,
                _ => panic!("{}", field_type.trim()),
            };

            let name = name.to_owned();
            let comment = post_process_comment(comment);
            let field = Field { comment, name };
            Some(((field_type, field), rem))
        } else {
            let (name, field_type) = field.split_once(":")?;
            let field_type = match field_type.trim() {
                "number" => FieldType::U64,
                "string" => FieldType::String,
                _ => panic!("{}", field_type.trim()),
            };

            let name = name.to_owned();
            let comment = post_process_comment(comment);
            let field = Field { comment, name };
            Some(((field_type, field), rem))
        }
    }
}

fn parse(input: &str) -> Ast {
    let input = pre_process(input);
    let (name, mut rem) = parse_struct_name(&input);

    let mut field_list: HashMap<FieldType, Vec<Field>> = HashMap::new();
    while let Some(((filed_type, field), rem2)) = parse_field(rem) {
        rem = rem2;
        //println!("{:?}: {:?}", filed_type, field);
        field_list.entry(filed_type).or_default().push(field);
    }
    //println!("{:?}", field_list.entry(FieldType::OptionalU64Vec));
    Ast {
        name: name.to_owned(),
        field_list,
    }
}

fn field_class_to_string(field_type: FieldType, field_list: &Vec<Field>, output: &mut String) {
    let header = match field_type {
        FieldType::OptionalAny => "Option<json::Value>",
        FieldType::OptionalU64 => "Option<u64>",
        FieldType::OptionalBool => "Option<bool>",
        FieldType::OptionalString => "Option<String>",
        FieldType::OptionalU64Vec => "Option<Vec<u64>>",
        FieldType::U64 => "u64",
        FieldType::String => "String",
    };
    output.push_str("\t");
    output.push_str(header);
    output.push_str(" {\n");

    for field in field_list {
        output.push_str("\t\t");
        output.push_str(&field.comment.replace("\n", "\n\t\t"));
        output.push_str("\n\t\t");
        output.push_str(&field.name);
        output.push_str(": \"");
        output.push_str(&field.name);
        output.push_str("\",\n");
    }
    output.push_str("\t},\n")
}

impl Ast {
    fn to_string(&mut self) -> String {
        let mut output = String::new();
        output.push_str(&self.name);
        output.push_str(" {\n");
        output.push_str("\toptional_args = ;\n");

        let field_list = self.field_list.entry(FieldType::U64).or_default();
        field_class_to_string(FieldType::U64, field_list, &mut output);

        let field_list = self.field_list.entry(FieldType::OptionalU64).or_default();
        field_class_to_string(FieldType::OptionalU64, field_list, &mut output);

        let field_list = self
            .field_list
            .entry(FieldType::OptionalU64Vec)
            .or_default();
        field_class_to_string(FieldType::OptionalU64Vec, field_list, &mut output);

        let field_list = self.field_list.entry(FieldType::OptionalBool).or_default();
        field_class_to_string(FieldType::OptionalBool, field_list, &mut output);

        let field_list = self.field_list.entry(FieldType::String).or_default();
        field_class_to_string(FieldType::String, field_list, &mut output);

        let field_list = self
            .field_list
            .entry(FieldType::OptionalString)
            .or_default();
        field_class_to_string(FieldType::OptionalString, field_list, &mut output);

        let field_list = self.field_list.entry(FieldType::OptionalAny).or_default();
        field_class_to_string(FieldType::OptionalAny, field_list, &mut output);

        output.push_str("\tCustom {},\n");
        output.push_str("\tOption<Custom> {},\n");

        output.push_str("}");
        output.replace("{\n\t}", "{}")
    }
}

fn main() {
    let mut file = File::open("./input.txt").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let mut ast = parse(&buf);
    let output = ast.to_string();
    let mut file = File::create("./output.rs").unwrap();
    file.write_all(output.as_bytes()).unwrap();
}
