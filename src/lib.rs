use aegis_core::{Value, NativeFn};
use std::collections::HashMap;

#[unsafe(no_mangle)]
pub extern "C" fn _aegis_register(map: &mut HashMap<String, NativeFn>) {
    map.insert("glfw_init".to_string(), my_glfw_init);
}

fn my_glfw_init(_: Vec<Value>) -> Result<Value, String> {
    println!("GLFW from DLL initialized!");
    Ok(Value::Null)
}
