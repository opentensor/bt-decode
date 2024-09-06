
use scale_info::TypeDefPrimitive;
use scale_info::{form::PortableForm, PortableRegistry, Type, TypeDef};
use std::collections::HashMap;

/*
    * Get the sub type string from a type string
    * This handles the case of Vec<T>, (T1, T2, T3, ...), [T; N]
    */
fn get_inner_string(type_string: &str) -> &str {
    let type_chars: Vec<char> = type_string.chars().collect();
    // last char of type is either >, ), or ]
    let bracket_char = type_chars[type_chars.len() - 1];

    // Find start of sub type; starts after the first bracket
    let start = type_chars.iter().position(|&x| x == bracket_char).unwrap();
    // Find end of sub type
    let end = type_chars.len() - 1;

    &type_string[(start + 1)..end]
}

fn primitive_to_type_string(primitive: &TypeDefPrimitive) -> String {
    match primitive {
        TypeDefPrimitive::Bool => "bool",
        TypeDefPrimitive::Char => "char",
        TypeDefPrimitive::Str => "str",
        TypeDefPrimitive::U8 => "u8",
        TypeDefPrimitive::U16 => "u16",
        TypeDefPrimitive::U32 => "u32",
        TypeDefPrimitive::U64 => "u64",
        TypeDefPrimitive::U128 => "u128",
        TypeDefPrimitive::U256 => "u256",
        TypeDefPrimitive::I8 => "i8",
        TypeDefPrimitive::I16 => "i16",
        TypeDefPrimitive::I32 => "i32",
        TypeDefPrimitive::I64 => "i64",
        TypeDefPrimitive::I128 => "i128",
        TypeDefPrimitive::I256 => "i256",
    }
    .to_string()
}

fn transform_type_to_string(ty: &Type<PortableForm>, registry: &PortableRegistry) -> String {
    let path = ty.path.clone();
    let type_def = ty.type_def.clone();

    if !path.is_empty() {
        return path
            .clone()
            .segments
            .last()
            .expect("type path is empty after checking")
            .to_string();
    } else {
        match type_def {
            TypeDef::Array(value) => {
                let length = value.len;
                let inner_type_id = value.type_param.id;

                let inner_type = registry
                    .resolve(inner_type_id)
                    .expect("inner type not found in registry");
                let inner_type_string = transform_type_to_string(inner_type, registry);

                format!("[{}; {}]", inner_type_string, length) // [T; N]
            }
            TypeDef::Primitive(primitive) => primitive_to_type_string(&primitive).to_string(),
            TypeDef::Compact(compact) => {
                let inner_type_id = compact.type_param.id;
                let inner_type = registry
                    .resolve(inner_type_id)
                    .expect("inner type not found in registry");

                let inner_type_string = transform_type_to_string(inner_type, registry);

                format!("Compact<{}>", inner_type_string)
            }
            TypeDef::Sequence(sequence) => {
                let inner_type_id = sequence.type_param.id;
                let inner_type = registry
                    .resolve(inner_type_id)
                    .expect("inner type not found in registry");

                let inner_type_string = transform_type_to_string(inner_type, registry);

                format!("Vec<{}>", inner_type_string)
            }
            TypeDef::Tuple(tuple) => {
                let inner_type_ids = tuple
                    .fields
                    .iter()
                    .map(|field| field.id)
                    .collect::<Vec<u32>>();

                let inner_types = inner_type_ids
                    .iter()
                    .map(|id| {
                        let inner_type = registry
                            .resolve(*id)
                            .expect("inner type not found in registry");
                        transform_type_to_string(inner_type, registry)
                    })
                    .collect::<Vec<String>>();

                format!("({})", inner_types.join(", "))
            }
            _ => "Unknown".to_string(),
        }
    }
}

pub fn fill_memo_using_well_known_types(
    type_string_to_index: &mut HashMap<String, u32>,
    registry: &PortableRegistry,
) {
    // Start with primitives
    let primitives = vec![
        "bool", "char", "str", "u8", "u16", "u32", "u64", "u128", "u256", "i8", "i16", "i32",
        "i64", "i128", "i256", // Matches the scale_info::TypeDefPrimitive enum
    ];
    let count = 0;
    let expected_count = primitives.len();

    // Add primitives to memo first, stop when all primitives are added
    for ty in registry.types.iter() {
        if count == expected_count {
            break; // Done with primitives
        }
        match &ty.ty.type_def {
            TypeDef::Primitive(primitive) => {
                let primitive_name = primitive_to_type_string(primitive);

                type_string_to_index.insert(primitive_name.to_string(), ty.id);
            }
            _ => continue,
        }
    }

    // Add other types to memo, should only depend on primitives
    for ty in registry.types.iter() {
        let type_string = transform_type_to_string(&ty.ty, registry);

        type_string_to_index.insert(type_string, ty.id);
    }
}

/*
    * Returns the TypeId in registry_builder of the type string
    */
pub fn get_type_id_from_type_string(
    memo: &mut HashMap<String, u32>,
    type_string: &str,
    registry: &PortableRegistry,
) -> Option<u32> {
    // Check if the type string is in the memo
    if let Some(idx) = memo.get(type_string) {
        return Some(*idx);
    } // This handles primitive types

    /* TODO: Implement where type string is not well-known
    // Create a new type and add it to the registry, memoize it, and return the id
    let type_chars: Vec<char> = type_string.chars().collect();
    if type_chars[type_chars.len() - 1] == '>' {
        // Has a sub type
        // We will assume it's a Vec
        let sub_type_string = get_inner_string(type_string).trim();
        let sub_type_id =
            get_type_id_from_type_string(memo, sub_type_string, registry_builder)?;

        let new_type = scale_info::Type::builder_portable()
            .path(scale_info::Path::<PortableForm>::from_segments(vec![type_string]))
            .composite(
                scale_info::build::FieldsBuilder::<_, scale_info::build::UnnamedFields>::default()
                    .field(|f| f.ty::<[u8]>().type_name("Vec<u8>")),
            );

        let new_type_id = registry_builder.register_type(new_type);
        Some(new_type_id)
    } else if type_string != "()"
        && type_chars[0] == '('
        && type_chars[type_chars.len() - 1] == ')'
    {
        // Is a tuple
        let inner_string = get_inner_string(type_string).trim();
        let sub_types: Vec<String> = inner_string.split(",").map(|x| x.trim()).collect().into();

        Some(TypeDef::Tuple(TypeDefTuple::new_portable(
            sub_types
                .iter()
                .map(|x| {
                    TypeParameter::<PortableForm>::new_portable(
                        "".to_string(),
                        get_type_def_from_type_string(memo, x, registry)
                            .unwrap()
                            .into(),
                    )
                })
                .collect(),
        )))
    } else if type_string != "[]" && type_chars[0] == '[' && type_chars[type_chars.len()] == ']'
    {
        // Is an array
        let inner_string = get_inner_string(type_string).trim();
        let semi_colon_index = inner_string.find(";")?;
        let sub_type_string = inner_string[..semi_colon_index].trim();

        let array_length = inner_string[semi_colon_index + 1..].parse::<u32>().unwrap();

        Some(TypeDef::Array(TypeDefArray::new(
            array_length,
            get_type_def_from_type_string(memo, sub_type_string, registry)?.into(),
        )))
    }}
        */

    None
}
