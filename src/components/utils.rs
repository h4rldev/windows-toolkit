use std::str::FromStr;

#[derive(Debug)]
pub enum WSLDistro {
    Ubuntu,
    Debian,
    Kali,
    UbuntuLTS1804,
    UbuntuLTS2004,
    UbuntuLTS2204,
    OracleLinux79,
    OracleLinux87,
    OracleLinux91,
    OpenSUSELeap155,
    SUSELinuxEnterpriseServer15SP4,
    SUSELinuxEnterprise15SP5,
    OpenSUSETumbleweed,
    Void,
    Arch,
    NixOS,
    Custom
}

impl FromStr for WSLDistro {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Ubuntu" => Ok(WSLDistro::Ubuntu),
            "Debian" => Ok(WSLDistro::Debian),
            "Kali" => Ok(WSLDistro::Kali),
            "UbuntuLTS1804" => Ok(WSLDistro::UbuntuLTS1804),
            "UbuntuLTS2004" => Ok(WSLDistro::UbuntuLTS2004),
            "UbuntuLTS2204" => Ok(WSLDistro::UbuntuLTS2204),
            "OracleLinux79" => Ok(WSLDistro::OracleLinux79),
            "OracleLinux87" => Ok(WSLDistro::OracleLinux87),
            "OracleLinux91" => Ok(WSLDistro::OracleLinux91),
            "OpenSUSELeap155" => Ok(WSLDistro::OpenSUSELeap155),
            "SUSELinuxEnterpriseServer15SP4" => Ok(WSLDistro::SUSELinuxEnterpriseServer15SP4),
            "SUSELinuxEnterprise15SP5" => Ok(WSLDistro::SUSELinuxEnterprise15SP5),
            "OpenSUSETumbleweed" => Ok(WSLDistro::OpenSUSETumbleweed),
            "Void" => Ok(WSLDistro::Void),
            "Arch" => Ok(WSLDistro::Arch),
            "NixOS" => Ok(WSLDistro::NixOS),
            "Custom" => Ok(WSLDistro::Custom),
            _ => Err(()),
        }
    }
}
