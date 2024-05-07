use crate::prelude::*;

pub fn query(
    json_value: &serde_json::Value,
    keys: Vec<&str>
) -> serde_json::Value {
    if json_value.is_array() {

        let mut accumulator: Vec<serde_json::Value> = Vec::new();

        json_value.as_array()
            .unwrap_or(&vec![json!("error")]) // expect?
            .into_iter()
            .for_each(|elem| {

                let mut res = elem.clone();

                for key in keys.iter() {
                    match res.get(*key) {
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
            match res.get(*key) {
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
) -> serde_json::Value {

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


#[cfg(test)]
mod test {
    use serde_json::json;


    use super::query;
    use super::query_dict;

    #[test]
    fn extract_from_object() {
        let json = json!({"foo":"cdu", "bar":"quino"});
        assert_eq!(query(&json, vec!["foo"]), "cdu".to_string());
    }

    #[test]
    fn extract_from_array() {
        let json_arr = json!([{"foo":"cdu", "bar":"quino"}]);
        assert_eq!(query(&json_arr, vec!["foo"]), json!(["cdu"]));
    }

    #[test]
    fn extract_dict_object() {
        let json = json!({"foo":"cdu", "bar":"quino"});
        assert_eq!(query_dict(&json, ("foo", "bar")), json!(["cdu", "quino"]));
    }

    #[test]
    fn extract_dict_array() {
        let json = json!([{"foo":"cdu","bar":"quino"}]);
        assert_eq!(query_dict(&json, ("foo", "bar")), json!([["cdu", "quino"]]));
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
        assert_eq!(query(&json, vec!["satoru", "gojo", "god"]), "si");
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
*/
