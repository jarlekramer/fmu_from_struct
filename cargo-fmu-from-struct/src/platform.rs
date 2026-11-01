use crate::error::CliError;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Architecture {
    X86,
    X86_64,
    Aarch64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum OperatingSystem {
    Windows,
    Linux,
    Darwin,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// The platform that the FMU binary is compiled for
pub struct Platform {
    architecture: Architecture,
    operating_system: OperatingSystem,
}

impl Platform {
    /// Parses a Rust target triple, e.g., 'x86_64-pc-windows-msvc'
    pub fn from_target_triple(target_triple: &str) -> Result<Self, CliError> {
        let unsupported = || CliError::new(format!("Unsupported target: {}", target_triple));

        let architecture = match target_triple.split('-').next() {
            Some("x86_64") => Architecture::X86_64,
            Some("i586" | "i686") => Architecture::X86,
            Some("aarch64") => Architecture::Aarch64,
            _ => return Err(unsupported()),
        };

        let operating_system = if target_triple.contains("-windows") {
            OperatingSystem::Windows
        } else if target_triple.contains("-linux") {
            OperatingSystem::Linux
        } else if target_triple.contains("-apple-darwin") {
            OperatingSystem::Darwin
        } else {
            return Err(unsupported());
        };

        Ok(Self { architecture, operating_system })
    }

    /// Returns the name of the platform folder inside the 'binaries' folder in the FMU, which
    /// depends on the FMI version.
    pub fn binary_folder_name(&self, fmi_major_version: u32) -> Result<String, CliError> {
        let unsupported = || CliError::new(format!(
            "The platform {:?} is not supported by FMI {}", self, fmi_major_version
        ));

        match fmi_major_version {
            2 => {
                let os = match self.operating_system {
                    OperatingSystem::Windows => "win",
                    OperatingSystem::Linux => "linux",
                    OperatingSystem::Darwin => "darwin",
                };

                let bits = match self.architecture {
                    Architecture::X86 => "32",
                    Architecture::X86_64 => "64",
                    // FMI 2 does not define names for ARM platforms, but 'darwin64' is commonly
                    // used for Apple silicon
                    Architecture::Aarch64 if self.operating_system == OperatingSystem::Darwin => "64",
                    Architecture::Aarch64 => return Err(unsupported()),
                };

                Ok(format!("{}{}", os, bits))
            },
            3 => {
                let architecture = match self.architecture {
                    Architecture::X86 => "x86",
                    Architecture::X86_64 => "x86_64",
                    Architecture::Aarch64 => "aarch64",
                };

                let os = match self.operating_system {
                    OperatingSystem::Windows => "windows",
                    OperatingSystem::Linux => "linux",
                    OperatingSystem::Darwin => "darwin",
                };

                Ok(format!("{}-{}", architecture, os))
            },
            _ => Err(CliError::new(format!("Unsupported FMI version: {}", fmi_major_version))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_folder_names() {
        let cases = [
            ("x86_64-pc-windows-msvc", "win64", "x86_64-windows"),
            ("x86_64-pc-windows-gnu", "win64", "x86_64-windows"),
            ("i686-pc-windows-msvc", "win32", "x86-windows"),
            ("x86_64-unknown-linux-gnu", "linux64", "x86_64-linux"),
            ("aarch64-apple-darwin", "darwin64", "aarch64-darwin"),
            ("x86_64-apple-darwin", "darwin64", "x86_64-darwin"),
        ];

        for (triple, fmi2, fmi3) in cases {
            let platform = Platform::from_target_triple(triple).unwrap();

            assert_eq!(platform.binary_folder_name(2).unwrap(), fmi2, "{}", triple);
            assert_eq!(platform.binary_folder_name(3).unwrap(), fmi3, "{}", triple);
        }
    }

    #[test]
    fn unsupported() {
        assert!(Platform::from_target_triple("wasm32-unknown-unknown").is_err());

        let arm_linux = Platform::from_target_triple("aarch64-unknown-linux-gnu").unwrap();
        assert!(arm_linux.binary_folder_name(2).is_err());
        assert_eq!(arm_linux.binary_folder_name(3).unwrap(), "aarch64-linux");
    }
}
