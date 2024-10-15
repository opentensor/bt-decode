use scale_info::{form::PortableForm, PortableRegistry, Type, TypeDef};
use scale_info::{
    PortableType, TypeDefArray, TypeDefCompact, TypeDefPrimitive, TypeDefSequence, TypeDefTuple,
};
use std::any::TypeId;
use std::collections::HashMap;
use std::vec;

/*
 * Get the sub type string from a type string
 * This handles the case of Vec<T>, (T1, T2, T3, ...), [T; N]
 */
fn get_inner_string(type_string: &str) -> &str {
    let type_chars: Vec<char> = type_string.chars().collect();
    // last char of type is either >, ), or ]
    let close_bracket_char = type_chars[type_chars.len() - 1];
    let bracket_char = match close_bracket_char {
        // Get the corresponding open bracket
        '>' => '<',
        ')' => '(',
        ']' => '[',
        _ => panic!("Invalid type string"),
    };

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
        if ty.type_params.is_empty() && ty.type_params.iter().all(|param| param.ty.is_none()) {
            return path
                .clone()
                .segments
                .last()
                .expect("type path is empty after checking")
                .to_string();
        } else {
            // This is a generic type
            let type_params_string = ty
                .type_params
                .iter()
                .map(|param| {
                    if param.ty.is_none() {
                        return "".to_string();
                    }
                    let param_id = param.ty.unwrap().id;
                    let param_type = registry
                        .resolve(param_id)
                        .expect("type param not found in registry");
                    transform_type_to_string(param_type, registry)
                })
                .filter(|x| !x.is_empty())
                .collect::<Vec<String>>()
                .join(", ");

            format!("{}<{}>", path.segments.last().unwrap(), type_params_string)
        }
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
    let primitives = [
        "bool", "str", "u8", "u16", "u32", "u64",
        "u128",
        // Apparently, u256, and i## are not supported
    ];
    let mut count = 0;
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
                count += 1;
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

/**
 * Adds a new type to the registry, without checking if it already exists.
 *
 * Returns the id of the new type
 */
fn add_to_registry_no_check(new_type: Type<PortableForm>, registry: &mut PortableRegistry) -> u32 {
    let next_index = registry.types.len();
    let new_type_id = next_index as u32;

    let type_entry: PortableType = PortableType {
        id: new_type_id,
        ty: new_type,
    };
    // Add the new type to the registry
    registry.types.push(type_entry);

    // Return the id of the new type
    new_type_id
}

/*
 * Returns the TypeId in registry_builder of the type string
 */
pub fn get_type_id_from_type_string(
    memo: &mut HashMap<String, u32>,
    type_string: &str,
    registry: &mut PortableRegistry,
) -> Option<u32> {
    // Check if the type string is in the memo
    if let Some(idx) = memo.get(type_string) {
        return Some(*idx);
    } // This handles primitive types

    // This means the type is NOT in the memo
    // We only handle types that are ultimately constructed from other types in the memo.
    // e.g. Vec<T>, (T1, T2, T3, ...), [T; N]

    // TODO: Handle structs ("composite") and enums ("variants")

    // Create a new type and add it to the registry, memoize it, and return the id
    let type_chars: Vec<char> = type_string.chars().collect();
    if type_chars.len() >= 12 && type_chars[0..12].iter().collect::<String>() == "scale_info::" {
        // This is a special formatting which has the type id in the string
        let type_id = type_string[12..]
            .trim()
            .parse::<u32>()
            .unwrap_or_else(|_| panic!("Failed to parse type id from string: {}", type_string));

        Some(type_id)
    } else if type_chars[type_chars.len() - 1] == '>'
        && type_chars.len() >= 4
        && type_chars[0..4].iter().collect::<String>() == "Vec<"
    {
        // This is a Vec<T> type, which is a sequence of one type T
        let sub_type_string = get_inner_string(type_string).trim();
        let sub_type_id = get_type_id_from_type_string(memo, sub_type_string, registry)?;

        let type_def = TypeDef::Sequence(TypeDefSequence::<PortableForm>::new(sub_type_id.into()));

        let new_type = scale_info::Type::<PortableForm>::new(
            scale_info::Path::default(),
            vec![], // Vecs don't have any type parameters
            type_def,
            vec![],
        );

        // Add the new type to the registry
        let new_type_id = add_to_registry_no_check(new_type, registry);
        // Insert to memo
        memo.insert(type_string.to_string(), new_type_id);

        Some(new_type_id)
    } else if type_string != "()" && type_chars[0] == '(' && type_chars[type_chars.len() - 1] == ')'
    {
        // This is a tuple; (T1, T2, T3, ...)
        // Made of multiple sub types T1, T2, T3, possibly different
        let inner_string = get_inner_string(type_string).trim();
        let sub_types: Vec<String> = inner_string.split(',').map(|x| x.trim().into()).collect();

        let mut sub_type_ids = vec![];

        for sub_type_string in sub_types {
            let sub_type_id = get_type_id_from_type_string(memo, &sub_type_string, registry)?;

            sub_type_ids.push(sub_type_id);
        }

        let sub_type_params: Vec<scale_info::interner::UntrackedSymbol<TypeId>> =
            sub_type_ids.iter().map(|&id| id.into()).collect();

        let type_def = TypeDef::Tuple(TypeDefTuple::<PortableForm>::new_portable(sub_type_params));

        let new_type = scale_info::Type::<PortableForm>::new(
            scale_info::Path::default(),
            vec![], // Tuples don't have any type parameters
            type_def,
            vec![],
        );

        // Add the new type to the registry
        let new_type_id = add_to_registry_no_check(new_type, registry);
        // Insert to memo
        memo.insert(type_string.to_string(), new_type_id);

        Some(new_type_id)
    } else if type_string != "[]" && type_chars[0] == '[' && type_chars[type_chars.len() - 1] == ']'
    {
        // Is an array; [T; N] where T is in the memo
        let inner_string = get_inner_string(type_string).trim();
        let semi_colon_index = inner_string.find(';')?;
        let sub_type_string = inner_string[..semi_colon_index].trim();

        let array_length = inner_string[semi_colon_index + 1..]
            .trim()
            .parse::<u32>()
            .unwrap();

        let sub_type_id = get_type_id_from_type_string(memo, sub_type_string, registry)?;

        let type_def = TypeDef::Array(TypeDefArray::<PortableForm>::new(
            array_length,
            sub_type_id.into(),
        ));

        let new_type = scale_info::Type::<PortableForm>::new(
            scale_info::Path::default(),
            vec![], // Arrays don't have any type parameters
            type_def,
            vec![],
        );

        // Add the new type to the registry
        let new_type_id = add_to_registry_no_check(new_type, registry);
        // Insert to memo
        memo.insert(type_string.to_string(), new_type_id);

        Some(new_type_id)
    } else if type_chars[type_chars.len() - 1] == '>'
        && type_chars.len() >= 8
        && type_chars[0..8].iter().collect::<String>() == "Compact<"
    {
        // This is a Compact<T> type, which is a compact encoding of one type T
        let sub_type_string = get_inner_string(type_string).trim();
        let sub_type_id = get_type_id_from_type_string(memo, sub_type_string, registry)?;

        let type_def = TypeDef::Compact(TypeDefCompact::<PortableForm>::new(sub_type_id.into()));

        let mut new_path = scale_info::Path::<PortableForm>::default();
        new_path.segments.push("Compact".to_string());

        let new_type = scale_info::Type::<PortableForm>::new(
            new_path,
            vec![], // Compact doesn't have any type parameters
            type_def,
            vec![],
        );

        // Add the new type to the registry
        let new_type_id = add_to_registry_no_check(new_type, registry);
        // Insert to memo
        memo.insert(type_string.to_string(), new_type_id);

        Some(new_type_id)
    } else if type_chars[type_chars.len() - 1] == '>'
        && type_chars.len() >= 7
        && type_chars[0..7].iter().collect::<String>() == "Option<"
    {
        // This is an Option<T> type, which is an enum with two variants: None and Some(T)

        // e.g. Option<T>
        let inner_string = get_inner_string(type_string).trim();
        let sub_type_string = inner_string;
        let sub_type_id = get_type_id_from_type_string(memo, sub_type_string, registry)?;

        let new_variants: Vec<scale_info::Variant<PortableForm>> = vec![
            scale_info::Variant {
                name: "None".to_string(),
                fields: vec![],
                index: 0,
                docs: vec![],
            },
            scale_info::Variant {
                name: "Some".to_string(),
                fields: vec![scale_info::Field {
                    name: None,
                    ty: sub_type_id.into(),
                    type_name: None,
                    docs: vec![],
                }],
                index: 1,
                docs: vec![],
            },
        ];

        let new_type_def = TypeDef::Variant(scale_info::TypeDefVariant::<PortableForm>::new(
            new_variants,
        ));

        let mut new_path = scale_info::Path::<PortableForm>::default();
        new_path.segments.push("Option".to_string());

        let new_type = scale_info::Type::<PortableForm>::new(
            new_path,
            vec![scale_info::TypeParameter::<PortableForm>::new_portable(
                "T".to_string(),
                Some(sub_type_id.into()),
            )], // Tuples don't have any type parameters
            new_type_def,
            vec![],
        );

        // Add the new type to the registry
        let new_type_id = add_to_registry_no_check(new_type, registry);
        // Insert to memo
        memo.insert(type_string.to_string(), new_type_id);

        Some(new_type_id)
    } else {
        None
    }
}
