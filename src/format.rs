use json::JsonValue;

use crate::utils::Result;

pub fn format(json: JsonValue) -> Result
{
    println!("{}", json::stringify_pretty(json, 4));
    Ok(())
}
