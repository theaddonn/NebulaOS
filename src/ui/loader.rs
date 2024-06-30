use alloc::string::String;

use crate::ui::control::UiControl;

pub enum LoadNebulaUiError {
    Utf8Error,
    TomlError(String),
}
//
// fn print_data(data: &Value, indent: u64) {
//     for (name, value) in data.iter() {
//         match value {
//             Value::Table(table) => {
//                 println!("{}[{}] (Table)", " "*indent, name);
//                 print_data(&Value::Table(table), indent+1); // Recursively print the nested table
//             }
//             Value::String(str_val) => {
//                 println!("{}[{}] {}", name, " "*indent, str_val);
//             }
//             Value::Integer(int_val) => {
//                 println!("{}[{}] {}", name, " "*indent, int_val);
//             }
//             Value::Float(float_val) => {
//                 println!("{}[{}] {}", name, " "*indent, float_val);
//             }
//             Value::Boolean(bool_val) => {
//                 println!("{}[{}] {}", name, " "*indent, bool_val);
//             }
//             Value::Datetime(dt_val) => {
//                 println!("{}[{}] {}", name, " "*indent, dt_val);
//             }
//             Value::Array(array_val) => {
//                 println!("{}[{}] (Array)", " "*indent, name);
//                 for (i, item) in array_val.iter().enumerate() {
//                     println!(" {}[{}] {}", " "*indent, i, item);
//                 }
//             }
//         }
//     }
// }

pub fn load_nebula_ui(file_content: &[u8]) -> Result<UiControl, LoadNebulaUiError> {
    // let text = match String::from_utf8(file_content.to_vec()) {
    //     Ok( text ) => { text.as_str() }
    //     Err( _ ) => { return Err(LoadNebulaUiError::Utf8Error) }
    // };
    //
    // let data: Table = match toml::from_str(text) {
    //     Ok( table ) => { table },
    //     Err( error ) => { return Err(LoadNebulaUiError::TomlError(String::from(error.message()))) }
    // };
    //
    // for (name, value) in data.iter() {
    //
    //     print_data(value, 0);
    // }

    Err(LoadNebulaUiError::Utf8Error)
}
