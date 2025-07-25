fn main() {
    
    println!("cargo:rerun-if-changed=src/rs_winrt_sample.idl");
    let metadata_dir = format!("{}\\System32\\WinMetadata", env!("windir"));
    let mut command = std::process::Command::new("midlrt.exe");

    command.args([
        "/winrt",
        "/nomidl",
        "/h",
        "nul",
        "/metadata_dir",
        &metadata_dir,
        "/reference",
        &format!("{metadata_dir}\\Windows.Foundation.winmd"),
        "/winmd",
        "rs_winrt_sample.winmd",
        "src/rs_winrt_sample.idl",
    ]);

    if !command.status().unwrap().success() {
        panic!("Failed to run midlrt");
    }

    windows_bindgen::bindgen([
        "--in",
        "rs_winrt_sample.winmd",
        &metadata_dir,
        "--out",
        "src/bindings.rs",
        "--filter",
        "rs_winrt_sample",
        "--flat",
        "--implement",
    ])
    .unwrap();
}