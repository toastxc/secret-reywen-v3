use serde::Serialize;

// converts the data of a structure into url query parameters - with null safety
pub fn struct_to_url<T: Serialize>(query: T) -> String {
    let mut iter = Vec::new();

    let json_str = serde_json::to_string(&query).unwrap_or_default();
    let json_obj = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(&json_str)
        .unwrap_or_default();

    for (key, value) in json_obj {
        // null handling
        if !value.is_null() {
            iter.push(format!("{key}={value}"));
        };
    }

    if iter.is_empty() {
        return String::new();
    };

    let mut str = String::new();

    for num in 0..iter.len() {
        let temp = match (num, iter[num].clone()) {
            (0, x) => format!("?{x}"),
            (_, x) => format!("&{x}"),
        };
        str += &temp;
    }
    str
}

// converts a generic option to a string of content or an empty string
pub fn option_str<T>(input: Option<T>) -> String
where
    T: std::fmt::Display,
{
    match input {
        None => String::new(),
        Some(a) => a.to_string(),
    }
}

pub fn if_false(t: &bool) -> bool {
    !t
}