#![recursion_limit = "2048"]

//use inflector::cases::{classcase::to_class_case, snakecase::to_snake_case};
use inflector::Inflector;
use libredfish::schema::{Schema, SimpleTypes};
use reqwest;
use serde_json::Value;
use std::borrow::Cow;
//use std::collections::HashMap;

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

fn resolve_type_value(field_ref: &str, v: &Value) -> Result<Value, String> {
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
                let ptr = field_ref.split_at(pos).1;
                Ok(ref_json.pointer(ptr).unwrap().clone())
            }
            None => Err(format!(
                "Unable to parse: {} hash position given",
                field_ref
            )),
        }
    } else {
        Ok(v.pointer(field_ref).unwrap().clone())
    }
}

fn resolve_type_schema(field_ref: &str, v: &Schema) -> Result<Schema, String> {
    if field_ref.starts_with("http") {
        let hash_pos = field_ref.find('#');

        //reqwest resolve it
        let c = reqwest::Client::builder()
            .danger_accept_invalid_hostnames(true)
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        let ref_json: Schema = c
            .get(field_ref)
            .send()
            .map_err(|e| e.to_string())?
            .error_for_status()
            .map_err(|e| e.to_string())?
            .json()
            .map_err(|e| e.to_string())?;
        match hash_pos {
            Some(pos) => {
                //println!("field_ref: {}, ref_json: {:?}", field_ref, ref_json);
                let field_parts: Vec<&str> = field_ref.split_at(pos).1.split('/').collect();
                let res = field_parts.iter().fold(&ref_json, |schema, comp| {
                    if *comp == "#" {
                        &ref_json
                    } else if *comp == "definitions" {
                        schema
                    } else {
                        schema
                            .definitions
                            .get(&comp.to_string())
                            .unwrap_or_else(|| panic!("Expected definition: `{:?}` {}", v, comp))
                    }
                });
                Ok(res.clone())
            }
            None => Err(format!(
                "Unable to parse: {} hash position given",
                field_ref
            )),
        }
    } else {
        let field_parts: Vec<&str> = field_ref.split('/').collect();
        //println!("field_parts: {:?}", field_parts);
        let res = field_parts.iter().fold(v, |schema, comp| {
            if *comp == "#" {
                v
            } else if *comp == "definitions" {
                schema
            } else {
                schema
                    .definitions
                    .get(&comp.to_string())
                    .unwrap_or_else(|| panic!("Expected definition: `{:?}` {}", v, comp))
            }
        });
        Ok(res.clone())
    }
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum FieldType {
    Bool,
    Enum(JsonEnum),
    EnumRef(String),
    Float,
    Long,
    Option(Box<FieldType>),
    String,
    Struct(JsonStruct),
    StructRef(String),
    Vector(Box<FieldType>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum RootTypes {
    Enum(JsonEnum),
    Struct(JsonStruct),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Field {
    sanitized_name: Option<String>,
    name: String,
    description: Option<String>,
    field_type: FieldType,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.field_type {
            FieldType::Struct(ref s) => write!(f, "{}", s.to_string()),
            FieldType::Vector(ref v) => {
                let mut s = String::new();
                if let Some(desc) = &self.description {
                    s.push_str(&format!("\t///{}\n", desc));
                }
                if let Some(sane_name) = &self.sanitized_name {
                    s.push_str(&format!("\t#[serde(rename=\"{}\")]\n", self.name));
                    s.push_str(&format!("\tpub {}: Vec<{}>,\n", sane_name, self.field_type));
                } else {
                    s.push_str(&format!("\tpub {}: Vec<{}>,\n", self.name, self.field_type));
                }
            
                write!(f, "{}", s)
            },
            _ => {
                let mut s = String::new();
                if let Some(desc) = &self.description {
                    s.push_str(&format!("\t///{}\n", desc));
                }
                if let Some(sane_name) = &self.sanitized_name {
                    s.push_str(&format!("\t#[serde(rename=\"{}\")]\n", self.name));
                    s.push_str(&format!("\tpub {}: {},\n", sane_name, self.field_type));
                } else {
                    s.push_str(&format!("\tpub {}: {},\n", self.name, self.field_type));
                }
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
            description: None,
            field_type: FieldType::Bool,
        }
    }
}
#[derive(Clone, Debug, Eq, Hash)]
struct JsonEnum {
    sanitized_name: Option<String>,
    name: String,
    description: String,
    fields: Vec<Field>,
}

impl Default for JsonEnum {
    fn default() -> Self {
        JsonEnum {
            sanitized_name: None,
            name: "".into(),
            description: "".into(),
            fields: Vec::new(),
        }
    }
}

// Equality by name
impl PartialEq for JsonEnum {
    fn eq(&self, other: &JsonEnum) -> bool {
        self.name == other.name
    }
}

impl JsonEnum {
    // The parse tree doesn't print correct.  It ends
    // up printing either duplicate enums or nested
    // enums.  This will modify self and change all
    // the nest JsonEnums in the fields to be a
    // FieldType::EnumRef(String) to flatten the tree
    fn walk_tree(&self, other: Option<Vec<JsonEnum>>) -> HashSet<JsonEnum> {
        let mut enums: HashSet<JsonEnum> = HashSet::new();

        let mut enum_stack: Vec<JsonEnum> = Vec::new();
        // Start with self
        enum_stack.push(self.clone());
        // Add other structs if needed
        if let Some(other) = other {
            enum_stack.extend(other);
        }
        loop {
            let s = enum_stack.pop();
            if s.is_none() {
                // Done
                break;
            }
            let mut s = s.unwrap();
            for f in s.fields.iter_mut() {
                let field_type = match f.field_type.clone() {
                    FieldType::Enum(j) => {
                        // Todo: use sanitized name
                        // Push to stack so it can be walked
                        enum_stack.push(j.clone());
                        // Remove the child struct
                        FieldType::EnumRef(j.name.clone())
                    }
                    _ => {
                        // No-op
                        f.field_type.clone()
                    }
                };
                f.field_type = field_type;
            }
            enums.insert(s);
        }
        enums
    }
}

#[derive(Clone, Debug, Eq, Hash)]
struct JsonStruct {
    sanitized_name: Option<String>,
    name: String,
    description: String,
    fields: Vec<Field>,
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

// Equality by name
impl PartialEq for JsonStruct {
    fn eq(&self, other: &JsonStruct) -> bool {
        self.name == other.name
    }
}

impl JsonStruct {
    // The parse tree doesn't print correct.  It ends
    // up printing either duplicate structs or nested
    // structs.  This will modify self and change all
    // the nest JsonStructs in the fields to be a
    // FieldType::StructRef(String) to flatten the tree
    fn walk_tree(&self, other: Option<Vec<JsonStruct>>) -> HashSet<JsonStruct> {
        let mut structs: HashSet<JsonStruct> = HashSet::new();

        let mut struct_stack: Vec<JsonStruct> = Vec::new();
        // Start with self
        struct_stack.push(self.clone());
        // Add other structs if needed
        if let Some(other) = other {
            struct_stack.extend(other);
        }
        loop {
            let s = struct_stack.pop();
            if s.is_none() {
                // Done
                break;
            }
            let mut s = s.unwrap();
            for f in s.fields.iter_mut() {
                let field_type = match f.field_type.clone() {
                    FieldType::Struct(j) => {
                        // Todo: use sanitized name
                        // Push to stack so it can be walked
                        struct_stack.push(j.clone());
                        // Remove the child struct
                        FieldType::StructRef(j.name.clone())
                    }
                    _ => {
                        // No-op
                        f.field_type.clone()
                    }
                };
                f.field_type = field_type;
            }
            structs.insert(s);
        }
        structs
    }
}

//impl fmt::Display for JsonStruct {
//fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
impl JsonStruct {
    fn to_string(&self) -> String {
        // Need to recurse and find all the structs.
        // Need to provide field names for the linked structs
        // That should fix the nested struct printing problem
        //println!("json_struct: {:?}", self);
        let mut defer_printing: Vec<&JsonStruct> = Vec::new();

        let mut s = String::new();
        s.push_str(&format!("///{}\n", self.description));
        if let Some(sane_name) = &self.sanitized_name {
            s.push_str(&format!("#[serde(rename=\"{}\")]\n", self.name));
            s.push_str(&format!("pub struct {} {{\n", sane_name));
        } else {
            s.push_str(&format!("pub struct {} {{\n", self.name));
        }
        for field in &self.fields {
            match field.field_type {
                FieldType::Struct(ref j_s) => {
                    // Don't print nested structs
                    s.push_str(&format!("\tpub {}: {}\n", j_s.name, j_s.name));
                    defer_printing.push(j_s);
                },
                _ => {
                    s.push_str(&format!("{}\n", field));
                }
            }
        }
        s.push_str("}\n");
        for d in defer_printing {
            s.push_str(&format!("{}", d.to_string()));
        }
        s
    }
}

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FieldType::Bool => write!(f, "bool"),
            FieldType::Enum(_s) => write!(f, ""),
            FieldType::EnumRef(s) => write!(f, "{}", s),
            FieldType::Float => write!(f, "f64"),
            FieldType::Long => write!(f, "u64"),
            FieldType::Option(t) => write!(f, "Option<{}>", t),
            FieldType::String => write!(f, "String"),
            FieldType::Struct(_s) => write!(f, ""),
            FieldType::StructRef(s) => write!(f, "{}", s),
            FieldType::Vector(s) => write!(f, "{}", s),
        }
    }
}

impl FromStr for FieldType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "array" => Ok(FieldType::Vector(Box::new(FieldType::Bool))),
            "boolean" => Ok(FieldType::Bool),
            "enum" => Ok(FieldType::Enum(JsonEnum::default())),
            "integer" => Ok(FieldType::Float),
            "number" => Ok(FieldType::Long),
            "object" => Ok(FieldType::Struct(JsonStruct::default())),
            "string" => Ok(FieldType::String),
            _ => Err(format!("Unknown field type: {}", s)),
        }
    }
}

fn match_type(v: &Schema, field: &mut Field, name: &str, schema: &Schema) -> Result<(), String> {
    //println!("match_type: {}", name);
    if v.type_.len() == 0 {
        // is there a $ref?
        match &v.ref_ {
            Some(v_ref) => {
                match v_ref {
                   Value::String(s) => {
                        let v = resolve_type_schema(s, &schema)?;
                        // recurse
                        match_type(&v, field, name, &schema)?;
                   }
                   _ => {}
                }
            }
            None => {
                // Ok i don't konw what this is
                if let Some(any_of) = &v.any_of {
                    field.field_type = FieldType::Enum(JsonEnum::default());
                } else if let Some(one_of) = &v.one_of {
                    field.field_type = FieldType::Enum(JsonEnum::default());
                } else {
                    println!("unknown {:?}", v);
                    return Ok(());
                }
            }
        }
    } else if v.type_.len() == 1 {
        // Base type
        match &v.type_[0] {
            SimpleTypes::Array => {
                // Grab one of the array items
                let first_item = v.items.first().unwrap();
                if let Some(item_ref) = &first_item.ref_ {
                    match item_ref {
                        Value::String(s) => {
                            let v = resolve_type_schema(&s, &schema)?;
                            field.field_type =
                                FieldType::Vector(Box::new(FieldType::StructRef(name.to_string())));
                        }
                        _ => {}
                    }
                }
                //println!("vector: {:?}", &v);
            }
            SimpleTypes::Boolean => {
                field.field_type = FieldType::Bool;
            }
            SimpleTypes::Integer => {
                field.field_type = FieldType::Long;
            }
            SimpleTypes::Null => {}
            SimpleTypes::Number => {
                field.field_type = FieldType::Float;
            }
            SimpleTypes::Object => {
                //println!("object: {:?}", &v);
                field.field_type = FieldType::StructRef(name.to_string());
            }
            SimpleTypes::String => {
                field.field_type = FieldType::String;
            }
        }
    } else {
        // Null allowed
        match &v.type_[1] {
            SimpleTypes::Array => {
                //field.field_type = FieldType::Option(Box::new(FieldType::Vector(name.to_string())));
            }
            SimpleTypes::Boolean => {
                field.field_type = FieldType::Option(Box::new(FieldType::Bool));
            }
            SimpleTypes::Integer => {
                field.field_type = FieldType::Option(Box::new(FieldType::Long));
            }
            SimpleTypes::Null => {
                //null_allowed = true;
            }
            SimpleTypes::Number => {
                field.field_type = FieldType::Option(Box::new(FieldType::Float));
            }
            SimpleTypes::Object => {
                field.field_type =
                    FieldType::Option(Box::new(FieldType::StructRef(name.to_string())));
            }
            SimpleTypes::String => {
                field.field_type = FieldType::Option(Box::new(FieldType::String));
            }
        }
    }

    Ok(())
}

fn find_fields(name: &str, v: &Schema, schema: &Schema) -> Result<Vec<Field>, String> {
    let mut fields: Vec<Field> = Vec::new();
    let mut field = Field::default();

    if let Some(desc) = &v.description {
        field.description = Some(desc.to_string());
    }
    match sanitize_name(name) {
        Some(new_name) => {
            field.sanitized_name = Some(new_name.to_snake_case());
            field.name = name.to_string();
        }
        None => {
            field.name = name.to_snake_case();
        }
    }
    match_type(&v, &mut field, name, &schema)?;
    fields.push(field);
    for (key, value) in &v.properties {
        let mut next_field = Field::default();
        if let Some(field_desc) = &value.description {
            next_field.description = Some(field_desc.to_string());
        }
        match sanitize_name(&key) {
            Some(new_name) => {
                next_field.sanitized_name = Some(new_name.to_string());
                next_field.name = key.to_string();
            }
            None => {
                next_field.name = key.to_string();
            }
        }
        match_type(&value, &mut next_field, name, &schema)?;
        fields.push(next_field);
    }
    Ok(fields)
}

fn find_structs(name: &str, v: &Schema, schema: &Schema) -> Result<JsonStruct, String> {
    let mut j = JsonStruct::default();

    if let Some(desc) = &v.description {
        j.description = desc.to_string();
    }
    match sanitize_name(name) {
        Some(new_name) => {
            j.sanitized_name = Some(new_name.to_class_case());
        }
        None => {
            j.name = name.to_class_case();
        }
    }
    let mut struct_fields: Vec<Field> = Vec::new();
    for (key, value) in &v.properties {
        match value.ref_ {
            Some(ref field_ref) => {
                match field_ref {
                    Value::String(s) => {
                        let val_type =
                            resolve_type_schema(&s, &schema).map_err(|e| e.to_string())?;
                        struct_fields.extend(find_fields(&key, &val_type, &schema)?);
                    }
                    _ => {}
                }
            }
            None => {
                struct_fields.extend(find_fields(&key, &value, &schema)?);
            }
        };
    }
    j.fields = struct_fields;

    Ok(j)
}

/*
fn find_all_refs(v: &Value, schema: &Value) -> Result<HashMap<String, JsonStruct>, String> {
    let mut cache: HashMap<String, JsonStruct> = HashMap::new();

    let mut stack: Vec<&Value> = Vec::new();
    stack.push(v);
    loop {
        let val = match stack.pop() {
            Some(val) => val,
            None => {
                break;
            }
        };
        match val {
            Value::Array(a) => {
                for array_val in a {
                    match array_val["$ref"].as_str() {
                        Some(field_ref) => {
                            let mut j = JsonStruct::default();
                            let mut struct_fields: Vec<Field> = Vec::new();
                            let val_type =
                                resolve_type(field_ref, &schema).map_err(|e| e.to_string())?;
                            if let Some(desc) = val_type["description"].as_str() {
                                j.description = desc.to_string();
                            }
                            match sanitize_name(key) {
                                Some(new_name) => {
                                    j.sanitized_name = Some(new_name.to_string());
                                }
                                None => {
                                    j.name = key.to_string();
                                }
                            }
                            j.fields = find_fields("", &val_type, &schema)?;
                            cache.insert("", j);
                        }
                        None => {
                            let mut j = JsonStruct::default();
                            let mut struct_fields: Vec<Field> = Vec::new();
                            j.fields = find_fields("", val, &schema)?;
                            cache.insert("", j);
                        }
                    }
                }
            }
            Value::Object(o) => {
                for (key, val) in o {
                    let mut j = JsonStruct::default();
                    if let Some(desc) = val["description"].as_str() {
                        j.description = desc.to_string();
                    }
                    match sanitize_name(key) {
                        Some(new_name) => {
                            j.sanitized_name = Some(new_name.to_string());
                        }
                        None => {
                            j.name = key.to_string();
                        }
                    }
                    let mut struct_fields: Vec<Field> = Vec::new();
                    if key == "$ref" {
                        let val_type = resolve_type(key, &schema).map_err(|e| e.to_string())?;
                        struct_fields.extend(find_fields(key, &val_type, &schema)?);
                    }
                    j.fields = struct_fields;
                    cache.insert(key.to_string(), j);
                }
            }
            _ => {
                //
            }
        }
    }

    Ok(cache)
}
*/

fn parse_schema(schema: &str) -> Result<(), String> {
    let mut structs: HashSet<JsonStruct> = HashSet::new();
    let v: Schema = serde_json::from_str(schema).map_err(|e| e.to_string())?;
    for (key, value) in &v.definitions {
        structs.insert(find_structs(&key, &value, &v)?);
    }

    // Merge them all into one vec
    let structs: Vec<JsonStruct> = structs.iter().map(|j| j.clone()).collect();
    let new_structs = structs[0].walk_tree(Some(structs[..].to_vec()));
    //let new_enums = structs[0].walk_tree(Some(structs[..].to_vec()));
    for s in new_structs.into_iter() {
        println!("{}", s.to_string());
    }

    Ok(())
}

fn main() {
    let schema = include_str!("/tmp/redfish.json");

    let result = parse_schema(&schema);
    //let schema = resolve_all_references(&schema);
    //let result = recursive_parse_schema(&schema);
    //println!("result: {:#?}", result);
}
