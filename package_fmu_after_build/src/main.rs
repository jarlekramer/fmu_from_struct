use std::env;
use std::fs::File;
use std::path::PathBuf;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;
use xmltree::Element;

mod errors;

use errors::PackageFmuError;


#[derive(Debug)]
enum OperatingSystem {
    Windows,
    Linux,
}

impl OperatingSystem {
    fn from_string(input: &str) -> Result<Self, PackageFmuError> {
        match input {
            "windows" => Ok(OperatingSystem::Windows),
            "linux" => Ok(OperatingSystem::Linux),
            _ => Err(PackageFmuError::InvalidOs(input.to_string())),
        }
    }

    fn from_current_os() -> Result<Self, PackageFmuError> {
        let os_name = env::consts::OS;
        OperatingSystem::from_string(os_name)
    }
}

fn get_bin_folder_path(fmi_version: i32, os: &OperatingSystem) -> Result<PathBuf, PackageFmuError> {
    match fmi_version {
        2 => match os {
            OperatingSystem::Windows => Ok(PathBuf::from("binaries/win64/")),
            OperatingSystem::Linux => Ok(PathBuf::from("binaries/linux64/")),
        },
        3 => match os {
            OperatingSystem::Windows => Ok(PathBuf::from("binaries/x86_64-windows/")),
            OperatingSystem::Linux => Ok(PathBuf::from("binaries/x86_64-linux/")),
        },
        _ => Err(PackageFmuError::InvalidFmiVersion),
    }
}

fn main() -> Result<(), PackageFmuError> {
    let args: Vec<String> = env::args().collect();
    let release = args.contains(&String::from("--release"));

    let current_folder_name = env::current_dir()?
        .file_name()
        .ok_or(PackageFmuError::NoFolderPathFromEnv)?
        .to_str()
        .ok_or(PackageFmuError::FailedToConvertFolderNameToString)?
        .to_string();

    let os = OperatingSystem::from_current_os()?;

    let model_description = Element::parse(
        File::open("modelDescription.xml")?
    )?;

    let fmi_version: f32 = model_description
        .attributes
        .get("fmiVersion")
        .ok_or(PackageFmuError::NoFmiVersionInModelDescription)?
        .parse()
        .expect("Failed to parse fmiVersion");

    let bin_folder = get_bin_folder_path(fmi_version as i32, &os)?;

    let model_name = model_description
        .attributes
        .get("modelName")
        .ok_or(PackageFmuError::NoModelNameInModelDescription)?
        .to_string();

    let builder_path = if release {
        println!("Making the release build into an FMU");
        PathBuf::from("target/release")
    } else {
        println!("Making the debug build into an FMU");
        PathBuf::from("target/debug")
    };

    if !builder_path.exists() {
        return Err(PackageFmuError::NoBuilderPath);
    }

    let fmu_path = PathBuf::from(format!("{}.fmu", model_name));
    let fmu_file = File::create(&fmu_path)?;

    let mut zip = ZipWriter::new(fmu_file);

    let zip_options = SimpleFileOptions::default()
        .unix_permissions(0o755);

    zip.start_file("modelDescription.xml", zip_options)?;
    
    
    let mut f = File::open("modelDescription.xml")?;
    std::io::copy(&mut f, &mut zip)?;

    match os {
        OperatingSystem::Windows => {
            zip.start_file(
                bin_folder.join(format!("{}.dll", model_name)).to_str().unwrap(),
                zip_options,
            )?;

            let mut f = File::open(builder_path.join(format!("{}.dll", current_folder_name)))?;
            std::io::copy(&mut f, &mut zip)?;

            zip.start_file(
                bin_folder.join(format!("{}.lib", model_name)).to_str().unwrap(),
                zip_options,
            )?;
            let mut f = File::open(builder_path.join(format!("{}.dll.lib", current_folder_name)))?;
            std::io::copy(&mut f, &mut zip)?;

            zip.start_file(
                bin_folder.join(format!("{}.exp", model_name)).to_str().unwrap(),
                zip_options,
            )?;
            let mut f = File::open(builder_path.join(format!("{}.dll.exp", current_folder_name)))?;
            std::io::copy(&mut f, &mut zip)?;

            zip.start_file(
                bin_folder.join(format!("{}.pdb", model_name)).to_str().unwrap(),
                zip_options,
            )?;
            let mut f = File::open(builder_path.join(format!("{}.pdb", current_folder_name)))?;
            std::io::copy(&mut f, &mut zip)?;
        }
        OperatingSystem::Linux => {
            let shared_lib_path = builder_path.join(format!("lib{}.so", current_folder_name));

            zip.start_file(
                bin_folder.join(format!("{}.so", model_name)).to_str().unwrap(),
                zip_options,
            )?;
            let mut f = File::open(shared_lib_path)?;
            std::io::copy(&mut f, &mut zip)?;
        }
    }

    zip.finish()?;

    Ok(())
}