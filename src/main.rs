mod evaluator;
mod module_loader;

use std::collections::HashMap;

use evaluator::Runtime;

#[tokio::main]
async fn main() {
    let mut args = std::env::args();
    let mut runtime = Runtime::new();

    let unknown_command = "unknown command: either 'pass' or 'fail' supported";

    let evaluated = match args.nth(1).expect(unknown_command).as_str() {
        "pass" => {
            let script =
                std::fs::read_to_string("script-pass.ts").expect("failed to read the script");
            runtime
                .evaluate(script, None)
                .await
                .expect("failed to evaluate the script")
        }
        "fail" => {
            let script =
                std::fs::read_to_string("script-fail.ts").expect("failed to read the script");

            let mut params = HashMap::new();
            params.insert("title".to_string(), serde_json::to_value("FOOO").unwrap());
            params.insert(
                "message".to_string(),
                serde_json::to_value("barrr").unwrap(),
            );

            runtime
                .evaluate(script, Some(params))
                .await
                .expect("failed to evaluate the script")
        }
        _ => unreachable!("{}", unknown_command),
    };

    println!("{:?}", evaluated);
}
