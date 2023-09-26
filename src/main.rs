use wasmedge_sdk::VmBuilder;

use wasmedge_sdk::plugin::PluginManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create a new WasmEdge Vm instance
    let vm = VmBuilder::new().build()?;

    PluginManager::load(None).unwrap();
    let _ = vm.auto_detect_plugins().unwrap();
    println!("plugins: {:?}", PluginManager::names());
    Ok(())
}
