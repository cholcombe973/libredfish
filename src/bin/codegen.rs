use json_pointer::JsonPointer;
use reqwest;
use serde_json::Value;
use std::borrow::Cow;

use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

// Change the name if it's using reserved keywords
fn sanitize_name(name: &str) -> Option<Cow<str>> {
    if name.contains('#')
        || name.contains('@')
        || name.contains('.')
        || name.contains("self")
        || name.contains("Type")
    {
        if name == "self" {
            Some(Cow::from("_self"))
        } else if name == "Type" {
            Some(Cow::from("_type"))
        } else {
            let sanitized = name.replace('@', "").replace('.', "").replace('#', "");
            Some(Cow::from(sanitized))
        }
    } else {
        None
    }
}

fn resolve_type(field_ref: &str, v: &Value) -> Result<Value, String> {
    if field_ref.starts_with("http") {
        let hash_pos = field_ref.find('#');

        //reqwest resolve it
        let c = reqwest::Client::builder()
            .danger_accept_invalid_hostnames(true)
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        let ref_json: Value = c
            .get(field_ref)
            .send()
            .map_err(|e| e.to_string())?
            .error_for_status()
            .map_err(|e| e.to_string())?
            .json()
            .map_err(|e| e.to_string())?;
        match hash_pos {
            Some(pos) => {
                //println!("field_ref: {}", field_ref.split_at(pos).1);
                let ptr = JsonPointer::from_str(field_ref.split_at(pos).1)
                    .map_err(|e| format!("{:?}", e))?;
                Ok(ptr.get(&ref_json).map_err(|e| format!("{:?}", e))?.clone())
            }
            None => Err(format!(
                "Unable to parse: {} hash position given",
                field_ref
            )),
        }
    } else {
        let ptr = JsonPointer::from_str(field_ref).map_err(|e| format!("{:?}", e))?;
        Ok(ptr.get(v).map_err(|e| format!("{:?}", e))?.clone())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum FieldType {
    Bool,
    Enum,
    Float,
    Long,
    Option(Box<FieldType>),
    String,
    Struct(JsonStruct),
    Vector(Box<FieldType>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Field {
    sanitized_name: Option<String>,
    name: String,
    description: String,
    field_type: FieldType,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.field_type {
            FieldType::Struct(ref s) => write!(f, "{}", s),
            FieldType::Vector(ref v) => write!(f, "{}", v),
            _ => {
                let mut s = String::new();
                s.push_str(&format!("\t///{}\n", self.description));
                if let Some(sane_name) = &self.sanitized_name {
                    s.push_str(&format!("\t#[serde(rename=\"{}\")]\n", sane_name));
                }
                s.push_str(&format!("\tpub {}: {},\n", self.name, self.field_type));
                write!(f, "{}", s)
            }
        }
    }
}

impl Default for Field {
    fn default() -> Self {
        Field {
            sanitized_name: None,
            name: "".into(),
            description: "".into(),
            field_type: FieldType::Bool,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct JsonStruct {
    sanitized_name: Option<String>,
    name: String,
    description: String,
    fields: Vec<Field>,
}

impl JsonStruct {
    fn get_structs(&self) -> Vec<JsonStruct> {
        let mut structs: Vec<JsonStruct> = Vec::new();
        let mut stack: Vec<&Vec<Field>> = Vec::new();

        // Setup
        structs.push(self.clone());
        stack.push(&self.fields);
        // Search
        loop {
            match stack.pop() {
                Some(fields) => {
                    println!("searching: {:?}", fields);
                    for field in fields {
                        match &field.field_type {
                            FieldType::Struct(ref s) => {
                                structs.push(s.clone());
                                stack.push(&s.fields);
                            }
                            _ => {
                                continue;
                            }
                        }
                    }
                }
                None => {
                    break;
                }
            }
        }
        structs
    }
}

impl Default for JsonStruct {
    fn default() -> Self {
        JsonStruct {
            sanitized_name: None,
            name: "".into(),
            description: "".into(),
            fields: Vec::new(),
        }
    }
}

impl fmt::Display for JsonStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Need to recurse and find all the structs.
        // Need to provide field names for the linked structs
        // That should fix the nested struct printing problem
        let mut s = String::new();
        s.push_str(&format!("///{}\n", self.description));
        if let Some(sane_name) = &self.sanitized_name {
            s.push_str(&format!("#[serde(rename=\"{}\")]\n", sane_name));
        }
        s.push_str(&format!("pub struct {} {{\n", self.name));
        for field in &self.fields {
            s.push_str(&format!("{}\n", field));
        }
        s.push_str("}\n");
        write!(f, "{}", s)
    }
}

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FieldType::Bool => write!(f, "bool"),
            FieldType::Enum => write!(f, "enum"),
            FieldType::Float => write!(f, "f64"),
            FieldType::Long => write!(f, "u64"),
            FieldType::Option(t) => write!(f, "Option<{}>", t),
            FieldType::String => write!(f, "String"),
            FieldType::Struct(s) => write!(f, "{}", s),
            FieldType::Vector(s) => write!(f, "Vec<{}>", s),
        }
    }
}

impl FromStr for FieldType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "array" => Ok(FieldType::Vector(Box::new(FieldType::Bool))),
            "boolean" => Ok(FieldType::Bool),
            "enum" => Ok(FieldType::Enum),
            "integer" => Ok(FieldType::Float),
            "number" => Ok(FieldType::Long),
            "object" => Ok(FieldType::Struct(JsonStruct::default())),
            "string" => Ok(FieldType::String),
            _ => Err(format!("Unknown field type: {}", s)),
        }
    }
}

fn print_fields(name: &str, v: &Value, schema: &Value) -> Result<Vec<Field>, String> {
    let mut fields: Vec<Field> = Vec::new();
    let mut field = Field::default();

    if let Some(desc) = v["description"].as_str() {
        field.description = desc.to_string();
    }
    match sanitize_name(name) {
        Some(new_name) => {
            field.sanitized_name = Some(new_name.to_string());
            field.name = name.to_string();
        }
        None => {
            field.name = name.to_string();
        }
    }
    match &v["type"] {
        Value::Array(a) => {
            let base_type = &a[0].as_str().unwrap();
            let f_type = FieldType::from_str(base_type)?;
            match &a.get(1) {
                // Nulls are allowed
                Some(_) => {
                    field.field_type = FieldType::Option(Box::new(f_type));
                }
                // Null is not allowed
                None => {
                    field.field_type = f_type;
                }
            }
        }
        Value::String(s) => {
            let f_type = FieldType::from_str(&s)?;
            match f_type {
                FieldType::Vector(_) => {
                    if let Some(inner_field_type) = v["items"]["type"].as_str() {
                        let inner = FieldType::from_str(inner_field_type)?;
                        field.field_type = FieldType::Vector(Box::new(inner));
                    }
                }
                FieldType::Struct(_) => {
                    field.field_type = FieldType::Struct(print_struct(name, &v, &schema)?);
                }
                _ => {
                    field.field_type = f_type.clone();
                }
            }
        }
        _ => {
            // Ignore the rest
        }
    }
    if let Some(params) = v["properties"].as_object() {
        for (key, value) in params {
            let mut next_field = Field::default();
            match sanitize_name(key) {
                Some(new_name) => {
                    next_field.sanitized_name = Some(new_name.to_string());
                    next_field.name = key.to_string();
                }
                None => {
                    next_field.name = key.to_string();
                }
            }
            if let Some(field_desc) = value["description"].as_str() {
                next_field.description = field_desc.to_string();
            }
            if let Some(field_type) = value["type"].as_str() {
                let f = FieldType::from_str(field_type)?;
                next_field.field_type = f.clone();
                match f {
                    FieldType::Vector(_) => {
                        if let Some(inner_field_type) = value["items"]["type"].as_str() {
                            let inner = FieldType::from_str(inner_field_type)?;
                            next_field.field_type = FieldType::Vector(Box::new(inner));
                        }
                    }
                    FieldType::Struct(_) => {
                        field.field_type = FieldType::Struct(print_struct(key, &value, &schema)?);
                    }
                    _ => {}
                }
            }
            //println!("next_field: {:?}", next_field);
            fields.push(next_field);
        }
    }
    //println!("field: {:?}", field);
    fields.push(field);
    Ok(fields)
}

fn print_struct(name: &str, v: &Value, schema: &Value) -> Result<JsonStruct, String> {
    //let mut structs: HashSet<JsonStruct> = HashSet::new();
    let mut j = JsonStruct::default();

    if let Some(desc) = v["description"].as_str() {
        j.description = desc.to_string();
    }
    match sanitize_name(name) {
        Some(new_name) => {
            j.sanitized_name = Some(new_name.to_string());
        }
        None => {
            j.name = name.to_string();
        }
    }
    if let Some(props) = v["properties"].as_object() {
        let mut struct_fields: Vec<Field> = Vec::new();
        for (key, value) in props {
            match value["$ref"].as_str() {
                Some(field_ref) => {
                    let val_type = resolve_type(field_ref, &schema).map_err(|e| e.to_string())?;
                    struct_fields.extend(print_fields(key, &val_type, &schema)?);
                }
                None => {
                    struct_fields.extend(print_fields(key, value, &schema)?);
                }
            };
        }
        j.fields = struct_fields;
    }
    //println!("struct: {:?}", j);
    //structs.insert(j);

    Ok(j)
}

fn parse_schema(schema: &str) -> Result<(), String> {
    let mut structs: HashSet<JsonStruct> = HashSet::new();
    let v: Value = serde_json::from_str(schema).map_err(|e| e.to_string())?;
    let definitions = v["definitions"]
        .as_object()
        .ok_or_else(|| "definitions object missing".to_string())?;
    for (key, value) in definitions {
        structs.insert(print_struct(key, &value, &v)?);
    }
    //println!("{:?}", structs);

    for s in &structs {
        println!("{}", s);
    }

    let mut final_structs: HashSet<JsonStruct> = HashSet::new();
    for s in structs {
        final_structs.extend(s.get_structs());
        //println!("get_structs: {:?}", s.get_structs());
    }
    println!("final structs: {:?}", final_structs);

    Ok(())
}

fn main() {
    let schema = include_str!("/tmp/redfish.json");

    let result = parse_schema(&schema);
    println!("result: {:?}", result);
}
