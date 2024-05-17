use crate::prelude::*;

pub fn query(
    json_value: &serde_json::Value,
    keys: Vec<String>
) -> serde_json::Value
{
    if json_value.is_array() {

        let mut accumulator: Vec<serde_json::Value> = Vec::new();

        json_value.as_array()
            .unwrap_or(&vec![json!("error")]) // expect?
            .into_iter()
            .for_each(|elem| {

                let mut res = elem.clone();

                for key in keys.iter() {
                    match res.get(key) {
                        Some(val) => res = val.clone(),
                        None => res = json!(format!("Unable to find value {:?} in JSON object", *key))
                    }
                }

                accumulator.push(res);

            });

        json!(accumulator)


    }
    else if json_value.is_object() {

        let mut res: serde_json::Value = json_value.clone();

        for key in keys.iter() {
            match res.get(key) {
                Some(val) => res = val.clone(),
                None => res = json!(format!("Unable to find value {:?} in JSON object", *key))
            }
        }

        res


    } else {
        json_value.clone()
    }

}

pub fn query_dict(
    json_val: &serde_json::Value,
    keys: (&str, &str)
) -> serde_json::Value
{

    if json_val.is_array() {

        let mut acc: Vec<serde_json::Value> = Vec::new();
        
        match json_val.as_array() {
            Some(arr) => {
                arr.into_iter()
                    .for_each(|elem| acc.push(query_dict(elem, keys)));
                json!(acc)
            }
            None => return json!("Expected json value to be an array, but was unable to parse it"),
        }
    }
    else if json_val.is_object() {

        if let (Some(result1), Some(result2)) =
            (json_val.get(keys.0), json_val.get(keys.1)) {

            json!(vec![result1, result2])
        }
        else {
            serde_json::from_str(
                format!("Extracting {:?},{:?} wasn't possible", keys.0, keys.1).as_str(),
            )
            .unwrap()
        }
    }
    else {
        json!("Couldn't determine JSON object")
    }
}

pub fn query_for_custom(
    json_val: &serde_json::Value,
    map: HashMap<String, Vec<String>>
) -> serde_json::Value
{
    let mut result = json!({});
    for (key, val) in map.iter() {
        result
            .as_object_mut()
            .unwrap()
            .insert(
                key.to_string(),
                query(
                    json_val,
                    val.to_vec()
                )
            );
            
    }

    result
}

pub fn query_from_vec_w_index(
    json_val: &serde_json::Value,
    index: usize
) -> serde_json::Value {

    assert!(json_val.is_array());

    json_val
        .as_array()
        .unwrap_or(json!([]).as_array().unwrap())
        .into_iter()
        .nth(index)
        .unwrap_or(&serde_json::Value::default())
        .clone()

    

}


#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use serde_json::json;


    use crate::query_for_custom;
    use crate::query_from_vec_w_index;

    use super::query;
    use super::query_dict;

    #[test]
    fn extract_from_object() {
        let json = json!({"foo":"cdu", "bar":"quino"});
        assert_eq!(query(&json, vec!["foo".to_string()]), "cdu".to_string());
    }

    #[test]
    fn extract_from_array() {
        let json_arr = json!([{"foo":"cdu", "bar":"quino"}]);
        assert_eq!(query(&json_arr, vec![format!("foo")]), json!(["cdu"]));
    }

    #[test]
    fn extract_dict_object() {
        let json = json!({"foo":"cdu", "bar":"quino"});
        let args = ("foo", "bar");
        assert_eq!(query_dict(&json, args), json!(["cdu", "quino"]));
    }

    #[test]
    fn extract_dict_array() {
        let json = json!([{"foo":"cdu","bar":"quino"}]);
        let args = ("foo", "bar");
        assert_eq!(query_dict(&json, args), json!([["cdu", "quino"]]));
    }

    #[test]
    fn test_query_nested() {
        let json = json!({
            "satoru": {
                "gojo": {
                    "god": "si"
                }
            }
        });
        let vec_arg = vec!["satoru".to_string(), "gojo".to_string(), "god".to_string()];
        assert_eq!(query(&json, vec_arg), "si");
    }

    #[test]
    fn test_query_for_custom() {
        let json = json!({
            "satoru": {
                "gojo": {
                    "god": "si"
                }
            }
        });

        let custom_key = String::from("respuesta_correcta");
        let vec_arg = vec!["satoru".to_string(), "gojo".to_string(), "god".to_string()];
        let expected_result = json!({"respuesta_correcta":"si"});
        let mut keys_set: HashMap<String, Vec<String>> = HashMap::new();
        keys_set.insert(custom_key, vec_arg);
        assert_eq!(query_for_custom(&json, keys_set), expected_result);
    }

    #[test]
    fn test_from_vec_w_index() {
        let json = json!([
            { "cj":"you say run" },
            { "nazar":"specialz" },
            { "quino":"digimon tamers opening castellano" },
            { "julio":"deera deera" }
        ]);
        let expected = json!({"nazar":"specialz"});
        let result = query_from_vec_w_index(&json, 1);
        assert_eq!(expected, result);
    }
}

/*
fn main() {
        let obj = json!({"foo":1,"bar":2});

        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        obj.serialize(&mut ser).unwrap();
        println!("{}", String::from_utf8(buf).unwrap());
    }")
}

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}
*/
