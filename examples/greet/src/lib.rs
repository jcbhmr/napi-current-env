mod greetings;

use napi_derive::napi;
use napi_current_env::CURRENT_ENV;
use napi::bindgen_prelude::*;

#[napi]
pub fn say_hello(env: Env) {
    fn inner() {
        let name = "Alan Turing";
        let message = greetings::hello(name);
        println!("{}", message);
    }
    CURRENT_ENV.sync_scope(env, || inner())
}
