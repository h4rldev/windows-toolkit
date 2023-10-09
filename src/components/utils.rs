use command_group::AsyncCommandGroup;
//use reqwest::Url;
use std::str::FromStr;
use tokio::{join, process::Command, spawn};
//use windows::core::imp::HANDLE;
use windows::Win32::{
    Foundation::HANDLE,
    Security::{
        GetTokenInformation, TokenElevation, TOKEN_ACCESS_MASK, TOKEN_ELEVATION, TOKEN_QUERY,
    },
    System::Threading::{GetCurrentProcess, OpenProcessToken},
};

#[derive(Debug)]
pub enum WSLDistro {
    Ubuntu,
    Debian,
    Kali,
    Ubuntu1804,
    Ubuntu2004,
    Ubuntu2204,
    Oracle79,
    Oracle87,
    Oracle91,
    Leap,
    SUSEServer15SP4,
    SUSEServer15SP5,
    Tumbleweed,
    Void,
    Arch,
    NixOS,
    Custom,
}

// this might be a bad idea, don't know yet

impl FromStr for WSLDistro {
    type Err = Box<dyn std::error::Error>;

    fn from_str(distro: &str) -> Result<Self, Self::Err> {
        match distro.to_lowercase().as_str() {
            "ubuntu" => Ok(WSLDistro::Ubuntu),
            "debian" => Ok(WSLDistro::Debian),
            "kali" => Ok(WSLDistro::Kali),
            "ubuntu lts 18.04" | "ubuntu 18.04" => Ok(WSLDistro::Ubuntu1804),
            "ubuntu lts 20.04" | "ubuntu 20.04" => Ok(WSLDistro::Ubuntu2004),
            "ubuntu lts 22.04" | "ubuntu 22.04" => Ok(WSLDistro::Ubuntu2204),
            "oracle linux 7.9" => Ok(WSLDistro::Oracle79),
            "oracle linux 8.7" => Ok(WSLDistro::Oracle87),
            "oracle linux 9.1" => Ok(WSLDistro::Oracle91),
            "opensuse leap 15.5" | "opensuse leap" | "suse leap" => Ok(WSLDistro::Leap),
            "suse linux enterprise server 15 sp4"
            | "suse linux enterprise server sp4"
            | "suse enterprise server sp4"
            | "suse server sp4" => Ok(WSLDistro::SUSEServer15SP4),
            "suse linux enterprise 15 sp5"
            | "suse linux enterprise server sp5"
            | "suse enterprise server sp5"
            | "suse server sp5" => Ok(WSLDistro::SUSEServer15SP5),
            "opensuse tumbleweed" | "suse tumbleweed" => Ok(WSLDistro::Tumbleweed),
            "void" | "void linux" | "voidlinux" => Ok(WSLDistro::Void),
            "arch" | "arch linux" | "archlinux" => Ok(WSLDistro::Arch),
            "nixos" | "nix" => Ok(WSLDistro::NixOS),
            "custom" => Ok(WSLDistro::Custom),
            _ => Err("Invalid distro".into()),
        }
    }
}

#[allow(dead_code)]
pub fn check(distro: WSLDistro) -> bool {
    matches!(
        distro,
        WSLDistro::Void | WSLDistro::Arch | WSLDistro::NixOS | WSLDistro::Custom
    )
}

#[allow(dead_code)]
pub async fn is_feature_enabled(feature_name: &str) -> bool {
    let output = Command::new("powershell")
        .args([
            "-Command",
            &format!(
                "Get-WindowsOptionalFeature -Online -FeatureName {}",
                feature_name
            ),
        ])
        .group_output()
        .await
        .unwrap();
    let output = String::from_utf8(output.stdout).unwrap();
    output.contains("Enabled")
}

#[allow(dead_code)]
pub async fn enable_feature(feature_name: &str) {
    let output = Command::new("powershell")
        .args([
            "-Command",
            &format!(
                "Enable-WindowsOptionalFeature -Online -FeatureName {}",
                feature_name
            ),
        ])
        .group_output()
        .await
        .unwrap();
    let output = String::from_utf8(output.stdout).unwrap();
    if output.contains("RestartNeeded : False") {
        println!("Restart your computer to finish enabling {}", feature_name);
        std::process::exit(0);
    }
    println!("{}", output);
}

#[allow(dead_code)]
pub async fn is_wsl_enabled() -> bool {
    let wsl = spawn(is_feature_enabled("Microsoft-Windows-Subsystem-Linux"));
    let virtual_machine_platform = spawn(is_feature_enabled("VirtualMachinePlatform"));
    let (wsl, virtual_machine_platform) = join!(wsl, virtual_machine_platform);
    let results = (
        wsl.expect("Failed to check wsl"),
        virtual_machine_platform.expect("Failed to check virtual machine platform"),
    );
    matches!(results, (true, true))
}

#[allow(dead_code)]
pub async fn get_default_config() -> Result<u32, Box<dyn std::error::Error>> {
    Ok(1)
}

pub fn is_elevated() -> windows::core::Result<bool> {
    unsafe {
        let mut h_token: HANDLE = HANDLE(0 as _);
        let result = OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_ACCESS_MASK(TOKEN_QUERY.0),
            &mut h_token,
        );
        match result {
            Ok(_) => {
                let mut token_elevation: TOKEN_ELEVATION = std::mem::zeroed();
                let mut return_length = 0;

                match GetTokenInformation(
                    h_token,
                    TokenElevation,
                    Some(&mut token_elevation as *mut _ as *mut _),
                    std::mem::size_of::<TOKEN_ELEVATION>() as u32,
                    &mut return_length,
                ) {
                    Ok(_) => {
                        if token_elevation.TokenIsElevated != 0 {
                            Ok(true)
                        } else {
                            Ok(false)
                        }
                    }
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }
}
