#[macro_use]
extern crate neon;
extern crate crypto;

use neon::vm::{Call, JsResult};
use neon::js::JsString;
use neon::js::error::JsError;
use neon::js::error::Kind;

use crypto::digest::Digest;
use crypto::sha1::Sha1;

use std::fs::File;
use std::io::Read;

fn sha1(call: Call) -> JsResult<JsString> {
    let scope = call.scope; // Tell JS not to free objects in this scope
    let input = call.arguments.require(scope, 0)?
        .check::<JsString>()?
        .value();
    //let input = "I haven't learnt how to do that yet";

    let mut hasher = Sha1::new();
    hasher.input_str(&input);
    let hex = hasher.result_str();
    
    Ok(JsString::new(scope, &hex).unwrap())
}

fn sha1_file(call: Call) -> JsResult<JsString> {
    let scope = call.scope; // Tell JS not to free objects in this scope
    let input_file = call.arguments.require(scope, 0)?
        .check::<JsString>()?
        .value();
    //let input = "I haven't learnt how to do that yet";

    // initialise hasher & file Read
    let mut hasher = Sha1::new();
    print!("Opening: {}", input_file);
    let f = File::open(input_file)
        .or_else(|_e| JsError::throw(Kind::Error,"File could not be opened"))?;

    for byte in f.bytes() {
        hasher.input(&[byte.unwrap()]);
    }
    let hex = hasher.result_str();
    
    Ok(JsString::new(scope, &hex).unwrap())
}

register_module!(m, {
    m.export("sha1", sha1)?;
    m.export("sha1File", sha1_file)?;
    Ok(())
});
