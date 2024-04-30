use super::module_loader::TsModuleLoader;
use anyhow::Context;
use deno_core::serde_v8::from_v8;
use deno_core::{
    v8::{self},
    JsRuntime, PollEventLoopOptions, RuntimeOptions,
};
use std::rc::Rc;

pub struct Runtime {
    js: JsRuntime,
}

#[derive(serde::Deserialize, Debug)]
pub struct Baz {
    pub baz: String,
    pub boo: i32,
}

#[derive(serde::Deserialize, Debug)]
pub struct ScriptReturn {
    pub title: String,
    pub message: String,
    pub baz: Baz,
}

impl Runtime {
    pub fn new() -> Self {
        let js = JsRuntime::new(RuntimeOptions {
            extensions: vec![],
            module_loader: Some(Rc::new(TsModuleLoader)),
            ..Default::default()
        });

        return Self { js };
    }

    pub async fn evaluate(
        &mut self,
        template: String,
        params: Option<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Result<ScriptReturn, anyhow::Error> {
        let script_url = deno_core::resolve_path(
            "script.ts",
            &std::env::current_dir().context("failed to get cwd")?,
        )?;

        let template_module = self
            .js
            .load_main_es_module_from_code(&script_url, template)
            .await?;

        let _ = self
            .js
            .run_event_loop(PollEventLoopOptions::default())
            .await?;
        let _ = self.js.mod_evaluate(template_module).await?;

        let template_module = self.js.get_module_namespace(template_module)?;
        let template_scope = &mut self.js.handle_scope();
        let template_module = v8::Local::<v8::Object>::new(template_scope, template_module);

        let template_key =
            v8::String::new(template_scope, "default").expect("Couldn't create a v8 string block");
        let template_function = template_module
            .get(template_scope, template_key.into())
            .expect("Couldn't find a default export");
        let template_fn = v8::Local::<v8::Function>::try_from(template_function)?;

        let params = if let Some(params) = params {
            deno_core::serde_v8::to_v8(template_scope, params)?
        } else {
            v8::undefined(template_scope).into()
        };

        let undefined = v8::undefined(template_scope);

        let Some(res) = template_fn.call(template_scope, undefined.into(), &[params]) else {
            anyhow::bail!("Failed to evaluate script")
        };

        Ok(from_v8(template_scope, res)?)
    }
}
