## Simple script evaluation failure repro

### Context

This repo is a basic prototype for a user-script runtime environment.

There are some basic differences in the way it works:
- all scripts are stored and loaded from the database into memory and executed
- all scripts must be expressed as default exports from the module
    ```typescript
    export default mainscript() {
        return "Hello world"
    }
    ```
- scripts make use of the helper library (modelled as `./script-api.ts` in here) that helps them to quickly scaffold the required return objects and provides some default values

> [!NOTE]
> The script runtime supports TypeScript directly and as such a custom module loader is implemented in `src/module_loader.rs`

### Problem:

Scripts are loaded in the `src/evaluator.rs` module using the `.load_main_es_module_from_code(&specifier, code)` function. However using it as such the main loaded module is **not** transpiled, resulting in a runtime error as the `JsRuntime` attempts to evaluate a non-JS module. The helper module `./script-api.ts` gets transpiled correctly, however.

It is therefore unclear what happens with the main module code when using `.load_main_es_module_from_code` (or `.load_main_es_module`) methods.

#### To repro:

1. Clone the repo
2. Run `cargo run pass` and see the `./script-pass.ts` module evaluate correctly. Notice in the `stdout` that only the `./script-api.ts` gets passed through the `ModuleLoader`
3. Run `cargo run fail` and see the `./script-fail.ts` module fail to evaluate with a runtime panic.
