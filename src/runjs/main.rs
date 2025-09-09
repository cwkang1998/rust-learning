use deno_ast::MediaType;
use deno_ast::ParseParams;
use deno_core::ModuleLoadResponse;
use deno_core::ModuleSourceCode;
use deno_core::error::AnyError;
use deno_core::error::ModuleLoaderError;
use deno_core::extension;
use deno_core::op2;
use std::rc::Rc;

static RUNTIME_SNAPSHOT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/RUNJS_SNAPSHOT.bin"));

#[derive(Debug, thiserror::Error, deno_error::JsError)]
enum RuntimeError {
    #[class(inherit)]
    #[error(transparent)]
    Io(#[inherit] std::io::Error),

    #[class("HttpError")]
    #[error("Http error: {0}")]
    Http(String),
}

#[op2(async)]
#[string]
async fn op_read_file(#[string] path: String) -> Result<String, RuntimeError> {
    let contents = tokio::fs::read_to_string(path)
        .await
        .map_err(RuntimeError::Io)?;
    Ok(contents)
}

#[op2(async)]
#[string]
async fn op_write_file(
    #[string] path: String,
    #[string] contents: String,
) -> Result<(), RuntimeError> {
    tokio::fs::write(path, contents)
        .await
        .map_err(RuntimeError::Io)?;
    Ok(())
}

#[op2(fast)]
#[string]
fn op_remove_file(#[string] path: String) -> Result<(), RuntimeError> {
    std::fs::remove_file(path).map_err(RuntimeError::Io)?;
    Ok(())
}

#[op2(async)]
#[string]
async fn op_fetch(#[string] url: String) -> Result<String, RuntimeError> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| RuntimeError::Http(e.to_string()))?;
    let body = response
        .text()
        .await
        .map_err(|e| RuntimeError::Http(e.to_string()))?;
    Ok(body)
}

struct TsModuleLoader;

impl deno_core::ModuleLoader for TsModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: deno_core::ResolutionKind,
    ) -> Result<deno_core::ModuleSpecifier, deno_core::error::ModuleLoaderError> {
        deno_core::resolve_import(specifier, referrer).map_err(|e| e.into())
    }

    fn load(
        &self,
        module_specifier: &deno_core::ModuleSpecifier,
        _maybe_referrer: Option<&deno_core::ModuleSpecifier>,
        _is_dyn_import: bool,
        _requested_module_type: deno_core::RequestedModuleType,
    ) -> ModuleLoadResponse {
        let module_specifier = module_specifier.clone();
        let module_load = move || {
            let path = module_specifier.to_file_path().unwrap();

            let media_type = MediaType::from_path(&path);
            let (module_type, should_transpile) = match media_type {
                MediaType::JavaScript | MediaType::Mjs | MediaType::Cjs => {
                    (deno_core::ModuleType::JavaScript, false)
                }
                MediaType::Jsx => (deno_core::ModuleType::JavaScript, true),
                MediaType::TypeScript
                | MediaType::Cts
                | MediaType::Dts
                | MediaType::Dmts
                | MediaType::Dcts
                | MediaType::Tsx => (deno_core::ModuleType::JavaScript, true),
                MediaType::Json => (deno_core::ModuleType::Json, false),
                _ => panic!("Unknown extension {:?}", path.extension()),
            };

            // Read and transpile
            let code = std::fs::read_to_string(&path)?;
            let code = if should_transpile {
                let parsed = deno_ast::parse_module(ParseParams {
                    specifier: module_specifier.clone(),
                    text: code.into(),
                    media_type,
                    capture_tokens: false,
                    scope_analysis: false,
                    maybe_syntax: None,
                })
                .map_err(|_err| ModuleLoaderError::NotFound)?;
                parsed
                    .transpile(
                        &Default::default(),
                        &Default::default(),
                        &Default::default(),
                    )
                    .map_err(|_err| ModuleLoaderError::Unsupported {
                        specifier: Box::new(module_specifier.clone()),
                        maybe_referrer: None,
                    })?
                    .into_source()
                    .text
                    .into_bytes()
            } else {
                code.into_bytes()
            };

            let module = deno_core::ModuleSource::new(
                module_type,
                ModuleSourceCode::Bytes(code.into_boxed_slice().into()),
                &module_specifier,
                None,
            );
            Ok(module)
        };
        ModuleLoadResponse::Sync(module_load())
    }
}

extension!(
    runjs,
    ops = [op_read_file, op_write_file, op_remove_file, op_fetch],
);

async fn run_js(file_path: &str) -> Result<(), AnyError> {
    let main_module = deno_core::resolve_path(file_path, &std::env::current_dir()?)?;
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(TsModuleLoader)),
        extensions: vec![runjs::init()],
        startup_snapshot: Some(RUNTIME_SNAPSHOT),
        ..Default::default()
    });

    let mod_id = js_runtime.load_main_es_module(&main_module).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(Default::default()).await?;

    result.await?;
    Ok(())
}

// Main entry point
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.is_empty() {
        eprintln!("Usage: runjs <file>");
        std::process::exit(1);
    }
    let file_path = &args[1];

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    if let Err(error) = runtime.block_on(run_js(file_path)) {
        eprintln!("error: {}", error);
    }
}
