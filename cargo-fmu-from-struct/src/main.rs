//! Cargo subcommand that builds a crate that uses the `fmu_from_struct` macro, and packages the
//! result as an FMU.
//!
//! Usage: `cargo fmu-from-struct [OPTIONS] [CARGO BUILD ARGS]...`. See the `HELP` text below.

use std::env;
use std::path::PathBuf;
use std::process::{self, Command, Stdio};
use std::io::BufReader;

use cargo_metadata::{Message, MetadataCommand, TargetKind};

mod error;
mod model_description;
mod package;
mod platform;

use error::CliError;
use model_description::ModelDescription;
use package::FmuContent;
use platform::Platform;

const HELP: &str = "\
Builds a crate that uses the fmu_from_struct macro and packages it as an FMU.

USAGE:
    cargo fmu-from-struct [OPTIONS] [CARGO BUILD ARGS]...

OPTIONS:
    --resources <PATH>    Folder to include as the 'resources' folder in the FMU. Defaults to the
                          'resources' folder in the crate, if it exists. Otherwise, the FMU gets an
                          empty 'resources' folder.
    -o, --output <PATH>   Path of the generated FMU file. Defaults to
                          '<target dir>/fmu/<profile>/<ModelName>.fmu'
    -h, --help            Print this help text
    -V, --version         Print the version

All other arguments are passed on to 'cargo build', for instance:
    cargo fmu-from-struct --release
    cargo fmu-from-struct --release -p my_fmu --resources path/to/resources -o MyFmu.fmu
";

/// Environment variable used to tell the macro where the target directory is. Must match the
/// variable used in `fmu_from_struct_derive`.
const TARGET_DIR_ENV_VAR: &str = "FMU_FROM_STRUCT_TARGET_DIR";

#[derive(Debug, Default)]
/// The parsed command line arguments
struct Arguments {
    resources: Option<PathBuf>,
    output: Option<PathBuf>,
    cargo_build_arguments: Vec<String>,
}

impl Arguments {
    /// Parses the arguments. Returns `None` if the program should exit early, e.g., after printing
    /// the help text.
    fn parse() -> Result<Option<Self>, CliError> {
        let mut raw_arguments = env::args().skip(1).peekable();

        // When run as `cargo fmu-from-struct`, cargo passes the subcommand name as first argument
        if raw_arguments.peek().map(|a| a.as_str()) == Some("fmu-from-struct") {
            raw_arguments.next();
        }

        let mut arguments = Arguments::default();

        while let Some(argument) = raw_arguments.next() {
            let mut value_for = |name: &str| raw_arguments.next().ok_or_else(|| {
                CliError::new(format!("Missing value for {}", name))
            });

            match argument.as_str() {
                "-h" | "--help" => {
                    print!("{}", HELP);
                    return Ok(None);
                },
                "-V" | "--version" => {
                    println!("cargo-fmu-from-struct {}", env!("CARGO_PKG_VERSION"));
                    return Ok(None);
                },
                "--resources" => arguments.resources = Some(PathBuf::from(value_for("--resources")?)),
                "-o" | "--output" => arguments.output = Some(PathBuf::from(value_for("--output")?)),
                _ => {
                    if let Some(value) = argument.strip_prefix("--resources=") {
                        arguments.resources = Some(PathBuf::from(value));
                    } else if let Some(value) = argument.strip_prefix("--output=") {
                        arguments.output = Some(PathBuf::from(value));
                    } else {
                        arguments.cargo_build_arguments.push(argument);
                    }
                }
            }
        }

        Ok(Some(arguments))
    }

    /// Returns the value of a cargo build argument, given either as `--name value` or
    /// `--name=value`
    fn cargo_build_argument_value(&self, name: &str) -> Option<String> {
        let prefix = format!("{}=", name);

        self.cargo_build_arguments.iter().enumerate().find_map(|(i, argument)| {
            if argument == name {
                self.cargo_build_arguments.get(i + 1).cloned()
            } else {
                argument.strip_prefix(&prefix).map(|value| value.to_string())
            }
        })
    }
}

/// Information about a compiled shared library from the cargo build output
struct CompiledLibrary {
    library_path: PathBuf,
    crate_folder: PathBuf,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {}", error);
        process::exit(1);
    }
}

fn run() -> Result<(), CliError> {
    let arguments = match Arguments::parse()? {
        Some(arguments) => arguments,
        None => return Ok(()),
    };

    // Check the resource folder before building, to fail early
    if let Some(resources) = &arguments.resources {
        if !resources.is_dir() {
            return Err(CliError::new(format!(
                "The resource folder does not exist: {}", resources.display()
            )));
        }
    }

    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    // ------------------------- Find the target directory -----------------------------------------
    let mut metadata_command = MetadataCommand::new();
    metadata_command.cargo_path(&cargo).no_deps();

    if let Some(manifest_path) = arguments.cargo_build_argument_value("--manifest-path") {
        metadata_command.manifest_path(manifest_path);
    }

    let target_dir: PathBuf = metadata_command.exec()?.target_directory.into_std_path_buf();

    // ------------------------- Build -------------------------------------------------------------
    let compiled_libraries = build(&cargo, &arguments, &target_dir)?;

    // ------------------------- Package -----------------------------------------------------------
    // Only the libraries with an embedded model description are FMUs
    let mut fmu_libraries: Vec<(CompiledLibrary, ModelDescription)> = Vec::new();

    for library in compiled_libraries {
        if let Some(model_description) = ModelDescription::from_library(&library.library_path)? {
            fmu_libraries.push((library, model_description));
        }
    }

    if fmu_libraries.is_empty() {
        return Err(CliError::new(
            "No FMU found in the build output. Make sure the crate uses the fmu_from_struct \
            macro, has crate-type = [\"cdylib\"], and uses fmu_from_struct version 0.3 or later."
        ));
    }

    if fmu_libraries.len() > 1 && arguments.output.is_some() {
        return Err(CliError::new(
            "--output can only be used when a single FMU is built. Use -p to select the crate."
        ));
    }

    let platform = Platform::from_target_triple(&target_triple(&cargo, &arguments)?)?;

    for (library, model_description) in fmu_libraries {
        let resources = match &arguments.resources {
            Some(resources) => Some(resources.clone()),
            None => {
                let default_resources = library.crate_folder.join("resources");

                default_resources.is_dir().then_some(default_resources)
            }
        };

        let output = match &arguments.output {
            Some(output) => output.clone(),
            None => default_output_path(&target_dir, &library.library_path, &model_description),
        };

        let content = FmuContent {
            model_description: &model_description,
            library_path: library.library_path.clone(),
            resources: resources.clone(),
        };

        package::write_fmu(&content, &platform, &output)?;

        println!("    Packaged FMU: {}", output.display());

        if let Some(resources) = resources {
            println!("       Resources: {}", resources.display());
        }
    }

    Ok(())
}

/// Runs `cargo build` and returns the compiled shared libraries from workspace crates
fn build(
    cargo: &str,
    arguments: &Arguments,
    target_dir: &PathBuf
) -> Result<Vec<CompiledLibrary>, CliError> {
    let mut child = Command::new(cargo)
        .arg("build")
        .arg("--message-format=json-render-diagnostics")
        .args(&arguments.cargo_build_arguments)
        .env(TARGET_DIR_ENV_VAR, target_dir)
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take().expect("stdout is piped");

    let mut compiled_libraries: Vec<CompiledLibrary> = Vec::new();

    for message in Message::parse_stream(BufReader::new(stdout)) {
        match message? {
            Message::CompilerArtifact(artifact) => {
                if !artifact.target.kind.contains(&TargetKind::CDyLib) {
                    continue;
                }

                let library_path = artifact.filenames.iter()
                    .find(|path| matches!(path.extension(), Some("dll" | "so" | "dylib")));

                if let Some(library_path) = library_path {
                    compiled_libraries.push(CompiledLibrary {
                        library_path: library_path.clone().into_std_path_buf(),
                        crate_folder: artifact.manifest_path.parent()
                            .map(|path| path.to_path_buf().into_std_path_buf())
                            .unwrap_or_default(),
                    });
                }
            },
            Message::TextLine(line) => println!("{}", line),
            _ => {}
        }
    }

    let status = child.wait()?;

    if !status.success() {
        return Err(CliError::new("cargo build failed"));
    }

    Ok(compiled_libraries)
}

/// Returns the target triple of the build: the value of `--target` if given, otherwise the host
fn target_triple(cargo: &str, arguments: &Arguments) -> Result<String, CliError> {
    if let Some(target) = arguments.cargo_build_argument_value("--target") {
        return Ok(target);
    }

    // Use the rustc that cargo uses, to get the right host
    let rustc = env::var("RUSTC").unwrap_or_else(|_| {
        PathBuf::from(cargo).with_file_name("rustc").to_string_lossy().into_owned()
    });

    let output = Command::new(&rustc).arg("-vV").output()
        .or_else(|_| Command::new("rustc").arg("-vV").output())?;

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .find_map(|line| line.strip_prefix("host: ").map(|host| host.trim().to_string()))
        .ok_or_else(|| CliError::new("Could not determine the host target triple from rustc"))
}

/// Returns `<target dir>/fmu/<profile folder>/<ModelName>.fmu`, where the profile folder is the
/// folder of the compiled library relative to the target directory, e.g., 'debug', 'release' or
/// 'x86_64-unknown-linux-gnu/release'.
fn default_output_path(
    target_dir: &PathBuf,
    library_path: &PathBuf,
    model_description: &ModelDescription
) -> PathBuf {
    let profile_folder = library_path.parent()
        .and_then(|folder| folder.strip_prefix(target_dir).ok())
        .map(|folder| folder.to_path_buf())
        .unwrap_or_default();

    target_dir
        .join("fmu")
        .join(profile_folder)
        .join(format!("{}.fmu", model_description.model_name))
}
