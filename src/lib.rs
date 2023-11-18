use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
#[derive(Deserialize, Serialize, Encode, Decode, PartialEq, Debug)]
struct ChatMessage {
    role: String,
    content: String,
}

#[wasm_bindgen]
pub fn serialization(iobject: JsValue) -> js_sys::Uint8Array {
    let a = serde_wasm_bindgen::from_value::<Vec<ChatMessage>>(iobject).unwrap();
    let config = bincode::config::standard();
    let bin = bincode::encode_to_vec(&a, config).unwrap();
    js_sys::Uint8Array::from(&bin[..])
}
#[wasm_bindgen]
pub fn deserialization(iobject: js_sys::Uint8Array) -> JsValue {
    let a = iobject.to_vec();
    let config = bincode::config::standard();
    let bin = bincode::decode_from_slice::<Vec<ChatMessage>, _>(&a, config)
        .unwrap()
        .0;
    serde_wasm_bindgen::to_value(&bin).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_serialization_deserialization() {
        assert!(true)
    }
}
