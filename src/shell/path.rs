use json::JsonValue;

use crate::utils::Result;

#[derive(Clone, Debug)]
pub enum PathElem
{
    Key(String),
    Index(usize)
}

impl std::fmt::Display for PathElem
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        use PathElem::{Key, Index};
        match self {
            Key(key) => write!(f, "{}", key),
            Index(index) => write!(f, "{}", index)
        }
    }
}

pub type Path = Vec<PathElem>;

pub fn reach<'a>(json: &'a JsonValue, path: &Path) -> &'a JsonValue
{
    let mut ret = json;
    for pathelem in path {
        match pathelem {
            PathElem::Key(key) => ret = &ret[key],
            PathElem::Index(index) => ret = &ret[*index],
        }
    }
    ret
}

pub fn advance(json: &JsonValue, path: &mut Path, key: &str) -> Result
{
    match (json, key) {
        (_, "..") => {
            if path.is_empty() {
                return Err("Cannot go outside root!".into())
            }
            path.pop();
        },
        (_, ".") => {},
        (obj, str) if obj.is_object() => {
            if !obj.has_key(str) {
                return Err("No such key!".into());
            }
            path.push(PathElem::Key(str.to_owned()));
        }
        (arr, str) if arr.is_array() => {
            let index: usize = str.parse().map_err(|_| "Arrays can only be indexed by integers!")?;

            if arr.len() <= index {
                return Err("Array index out of bounds!".into());
            }

            path.push(PathElem::Index(index));
        },
        _ => return Err("Reached leaf node!".into())
    }

    Ok(())
}

pub fn ls(json: &JsonValue) -> Vec<PathElem>
{
    let mut ret = Vec::new();

    match json {
        JsonValue::Object(object) =>
            for (key, _) in object.iter() {
                ret.push(PathElem::Key(key.to_owned()));
            },
        JsonValue::Array(array) =>
            for i in 0..array.len() {
                ret.push(PathElem::Index(i))
            },
        _ => ()
    }

    ret
}

pub fn edit(root: &mut JsonValue, path: &Path, newval: JsonValue) -> Result
{
    let mut ptr = root;
    for pathelem in path {
        match pathelem {
            PathElem::Key(key) => ptr = &mut ptr[key],
            PathElem::Index(index) => ptr = &mut ptr[*index],
        }
    }
    *ptr = newval;

    Ok(())
}

pub fn cat_except(root: &JsonValue, except: &[String]) -> Result
{
    let mut obj = match root {
        JsonValue::Object(object) => object.clone(),
        _ => return Err("Cat Except is only applicable on objects!".into())
    };

    for key in except {
        obj.remove(key);
    }
    
    println!("{}", json::stringify_pretty(obj, 4));
    Ok(())
}
