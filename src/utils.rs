use json::JsonValue;

pub type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub fn classname(json: &JsonValue) -> String
{
    match json {
        JsonValue::Null => "Null",
        JsonValue::Short(_) => "String",
        JsonValue::String(_) => "String",
        JsonValue::Number(_) => "Number",
        JsonValue::Boolean(_) => "Boolean",
        JsonValue::Object(_) => "Object",
        JsonValue::Array(_) => "Array",
    }.to_owned()
}
