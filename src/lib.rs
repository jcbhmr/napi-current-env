use napi::bindgen_prelude::*;
#[allow(unused_imports)]
use std::{error::Error, result::Result};
use tokio::task_local;

task_local! {
    /// A well-known [`tokio::task_local`] [`napi::Env`] accessible without argument drilling.
    /// 
    /// This is a pseudo `napi::CURRENT_ENV` that can be relied upon in third-party code that can't take a [`napi::Env`] argument, such as an isomorphic API across NAPI-RS, wasm-bindgen, and native implementation.
    /// 
    /// **You should use `sync_scope` or `scope` at every NAPI-RS entry point that provides you with a [`napi::Env`]** to ensure that this well-known global is set for all code within your task that might need it.
    /// 
    /// ```rs
    /// 
    /// ```
    /// 
    /// # Example
    /// 
    /// ```rs
    /// # use napi_derive::napi;
    /// # use napi::bindgen_prelude::*;
    /// # #[allow(unused_imports)]
    /// # use std::{error::Error, result::Result};
    /// use napi_current_env::CURRENT_ENV;
    /// 
    /// # fn main() {}
    /// #
    /// 
    /// mod another_crate {
    ///   // #[cfg(feature = "napi")]
    ///   fn greet(name: &str) -> napi::Result<String> {
    /// #   use napi::bindgen_prelude::*;
    /// #   #[allow(unused_imports)]
    /// #   use std::{error::Error, result::Result};
    ///     use napi_current_env::CURRENT_ENV;
    ///     let env = CURRENT_ENV.get();
    ///     let file_url_string = env.get_module_file_name()?;
    ///     let message = format!("Hello {} from {}!", name, file_url_string);
    ///     Ok(message)
    ///   }
    ///   // #[cfg(feature = "wasm-bindgen")]
    ///   // ...
    ///   // #[cfg(not(any(feature = "napi", feature = "wasm-bindgen")))]
    ///   // ...
    /// }
    /// 
    /// #[napi]
    /// fn print_greeting(env: Env, name: &str) -> napi::Result<()> {
    ///   CURRENT_ENV.sync_scope(env, || {
    ///     let greeting = another_crate::greet(name)?;
    ///     println!("{}", greeting);
    ///     Ok(())
    ///   })
    /// }
    /// ```
    pub static CURRENT_ENV: Env;
}
