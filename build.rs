fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest: &str = r#"
    <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
    <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
        <security>
            <requestedPrivileges>
                <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
            </requestedPrivileges>
        </security>
    </trustInfo>
    </assembly>
    "#;

    if cfg!(target_os = "windows") {
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
            .compile()?;
    }
    Ok(())
}
