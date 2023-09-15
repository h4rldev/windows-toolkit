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
    Custom,
}

impl FromStr for WSLDistro {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Ubuntu" => Ok(WSLDistro::Ubuntu),
            "Debian" => Ok(WSLDistro::Debian),
            "Kali" => Ok(WSLDistro::Kali),
            "Ubuntu LTS 18.04" => Ok(WSLDistro::UbuntuLTS1804),
            "Ubuntu LTS 20.04" => Ok(WSLDistro::UbuntuLTS2004),
            "Ubuntu LTS 22.04" => Ok(WSLDistro::UbuntuLTS2204),
            "Oracle Linux 7.9" => Ok(WSLDistro::OracleLinux79),
            "Oracle Linux 8.7" => Ok(WSLDistro::OracleLinux87),
            "Oracle Linux 9.1" => Ok(WSLDistro::OracleLinux91),
            "openSUSE Leap 15.5" => Ok(WSLDistro::OpenSUSELeap155),
            "SUSE Linux Enterprise Server 15 SP4" => Ok(WSLDistro::SUSELinuxEnterpriseServer15SP4),
            "SUSE Linux Enterprise 15S P5" => Ok(WSLDistro::SUSELinuxEnterprise15SP5),
            "openSUSE Tumbleweed" => Ok(WSLDistro::OpenSUSETumbleweed),
            "Void" => Ok(WSLDistro::Void),
            "Arch" => Ok(WSLDistro::Arch),
            "NixOS" => Ok(WSLDistro::NixOS),
            "Custom" => Ok(WSLDistro::Custom),
            _ => Err(()),
        }
    }
}
