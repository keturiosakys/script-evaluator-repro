mod evaluator;
mod module_loader;

use std::collections::HashMap;

use evaluator::Runtime;

#[tokio::main]
async fn main() {
    let mut runtime = Runtime::new();
    let script = std::fs::read_to_string("script.ts").expect("failed to read the script");

    let mut params = HashMap::new();
    params.insert("title".to_string(), serde_json::to_value("FOOO").unwrap());
    params.insert(
        "message".to_string(),
        serde_json::to_value("barrr").unwrap(),
    );

    let evaluated = runtime
        .evaluate(script, Some(params))
        .await
        .expect("failed to evaluate the template");

    println!("{:?}", evaluated);
}
