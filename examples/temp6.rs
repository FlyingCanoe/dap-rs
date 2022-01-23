use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug, Clone)]
struct Ast {
    name: String,
    field_list: Vec<Field>,
}

#[derive(Debug, Clone)]
struct Field {
    name: String,
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

fn parse_struct_name(input: &str) -> (&str, &str) {
    let (_, rem) = input.split_once("type").unwrap();
    let name = rem.split_whitespace().next().unwrap();
    let (_, rem) = rem.split_once(name).unwrap();

    (name, rem)
}

fn parse_field(input: &str) -> Option<(Field, &str)> {
    let (_, rem) = input.split_once("'")?;
    let (field, rem) = rem.split_once("'")?;

    let field = Field {
        name: field.to_owned(),
    };

    Some((field, rem))
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
            .map(|field| format!("\n\t\t{0} | \"{0}\",", field.name))
            .collect();

        format!(
            r#"
dap_type_enum!(
    {} {{{}
    }}
);
"#,
            self.name, field_declaration
        )
        .replace("\n\n\n", "\n")
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
