use std::{env, error::Error, process::Command};

#[test]
fn test_greet() -> Result<(), Box<dyn Error>> {
    let status = Command::new(env::var("CARGO")?)
        .args(&["build", "--package=examples-greet"])
        .status()?;
    if !status.success() {
        return Err(format!("build examples-greet failed: {:?}", status).into());
    }
    _ = fs_err::remove_file("target/debug/examples_greet.node");
    fs_err::rename(
        "target/debug/libexamples_greet.so",
        "target/debug/examples_greet.node",
    )?;
    const NODE_CODE: &str = r#"
        import * as module from "node:module";
        const require = module.createRequire(import.meta.filename);
        const examplesGreet = require("./target/debug/examples_greet.node");
        examplesGreet.sayHello();
    "#;
    let status = Command::new("node")
        .args(&["--input-type=module", "--eval", NODE_CODE])
        .status()?;
    if !status.success() {
        return Err(format!("run node failed: {:?}", status).into());
    }
    Ok(())
}
