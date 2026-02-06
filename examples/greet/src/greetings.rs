use napi_current_env::CURRENT_ENV;

pub fn hello(name: impl AsRef<str>) -> String {
    fn inner(name: &str) -> String {
        let env = CURRENT_ENV.get();
        let file_url_string = env
            .get_module_file_name()
            .expect("get_module_file_name should succeed");
        format!("Hello {} from {}!", name, file_url_string)
    }
    inner(name.as_ref())
}
