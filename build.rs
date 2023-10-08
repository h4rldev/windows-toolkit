fn build() {
    let manifest: &str = r#"
        <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
        <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
            <security>
                <requestedPrivileges>
                    <requestedExecutionLevel level="highestAvailable" />
                </requestedPrivileges>
            </security>
        </trustInfo>
        </assembly>
        "#;
    let mut res = winresource::WindowsResource::new();
    res.set_icon("icon.ico")
        .set("InternalName", "WSL Installer")
        .set("OriginalFilename", "wslinstaller.exe")
        .set("ProductName", "WSL Installer")
        .set(
            "Description",
            "Install WSL distributions and set them up for use with Windows Terminal",
        )
        .set_manifest(manifest)
        .compile()
        .expect("Failed to compile resource file");
}

fn main() {
    let target = build_target::target_triple().unwrap();
    if target.as_str() == "x86_64-pc-windows-gnu" {
        build();
    } else {
        println!("cargo:warning=Not building for Windows, skipping resource compilation");
    }
}
