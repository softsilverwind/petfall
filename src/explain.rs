use json::JsonValue;

use crate::utils::Result;

// json must be an array of objects else this will panic
fn traverse_and_merge(array: Vec<JsonValue>) -> JsonValue
{
    use JsonValue::{Object, Null};

    let mut ret = json::object!{};

    for obj in array.into_iter() {
        let mut real_obj = match obj {
            Object(object) => object,
            _ => panic!()
        };

        for (key, val) in real_obj.iter_mut() {
            ret[key] = std::mem::replace(val, Null);
        }
    }

    ret
}

pub fn traverse_explain(json: JsonValue) -> JsonValue
{
    use JsonValue::{Object, Array, String, Null};

    match json {
        Object(mut object) => {
            for (_, val) in object.iter_mut() {
                *val = traverse_explain(std::mem::replace(val, Null));
            }
            Object(object)
        },
        Array(mut array) => {
            for val in array.iter_mut() {
                *val = traverse_explain(std::mem::replace(val, Null));
            }

            let length = array.len();
            let contents = if array.iter().all(|x| x.is_object()) {
                traverse_and_merge(array)
            }
            else if array.iter().all(|x| *x == array[0]) {
                std::mem::replace(&mut array[0], Null)
            }
            else {
                Array(array)
            };

            json::array![length, contents]
        }
        x => String(crate::utils::classname(&x))
    }
}

pub fn explain(json: JsonValue) -> Result
{
    let explanation = traverse_explain(json);
    println!("{}", json::stringify_pretty(explanation, 4));
    Ok(())
}
