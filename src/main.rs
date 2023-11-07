use std::path::Path;
use wasmedge_sdk::config::{ConfigBuilder, HostRegistrationConfigOptions};
use wasmedge_sdk::plugin::PluginManager;
use wasmedge_sdk::VmBuilder;
use wasmedge_sys::plugin::PluginManager as p;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create a new WasmEdge Vm instance
    let host_options = HostRegistrationConfigOptions::default();
    let host_options = host_options.wasi(true);
    let config = ConfigBuilder::default()
        .with_host_registration_config(host_options)
        .build()
        .unwrap();
    let mut vm = VmBuilder::new().with_config(config).build().unwrap();
    vm.wasi_module_mut().unwrap().initialize(
        Some(vec![
            "--model-alias default",
            "--ctx-size",
            "4096",
            "--n-predict",
            "128",
            "--log-enable",
            "--stream-stdout",
            "--prompt",
            "Robert Oppenheimer most important achievement is",
        ]),
        None,
        Some(vec![
            "/home/vincent/workspace/_docker/test-autodetect-wasmedge-plugins:/",
        ]),
    );

    // PluginManager::load(Some(Path::new(
    //     "/home/vincent/workspace/_docker/WasmEdge/build/plugins/wasi_nn",
    // )))
    // .unwrap();
    PluginManager::load(None).unwrap();
    p::nn_preload(vec![
        "default:GGML:CPU:/disk/model_zoo/llama-2-7b.Q5_K_M.gguf",
    ]);
    let vm = vm.auto_detect_plugins().unwrap();
    let vm = vm
        .register_module_from_file("ggml", &"llama-simple.wasm")
        .unwrap();

    vm.run_func(Some("ggml"), "_start", vec![])?;

    println!("plugins: {:?}", PluginManager::names());
    Ok(())
}
