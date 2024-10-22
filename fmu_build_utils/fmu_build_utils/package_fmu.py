from pathlib import Path
from enum import Enum
import platform

import zipfile

import xml.etree.ElementTree as ElementTree

import argparse

class OperatingSystem(Enum):
    WINDOWS = 1
    LINUX = 2
    MACOS = 3

    @classmethod
    def from_string(cls, input_string: str):
        if input_string == "Windows":
            return cls.WINDOWS
        elif input_string == "Linux":
            return cls.LINUX
        elif input_string == "Darwin":
            return cls.MACOS
        else:
            raise ValueError("Invalid OS")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Build FMU from Rust model')
    parser.add_argument('--release', action='store_true')

    args = parser.parse_args()

    current_folder_name = Path.cwd().parts[-1]

    os_name = platform.system()

    os = OperatingSystem.from_string(os_name)


    # Check that modelDescription.xml exists. If not, the user is either in a wrong folder, or "cargo build" is not executed
    if not Path("modelDescription.xml").exists():
        raise FileNotFoundError("modelDescription.xml not found. Are you in the right folder? Have you executed 'cargo build'?")

    model_description = ElementTree.parse("modelDescription.xml")

    fmi_version = int(float(model_description.getroot().attrib['fmiVersion']))

    match fmi_version:
        case 2:
            match os:
                case OperatingSystem.WINDOWS:
                    bin_folder = Path("binaries/win64")
                case OperatingSystem.LINUX:
                    bin_folder = Path("binaries/linux64")
                case _:
                    raise ValueError("Invalid OS")
        case 3:
            match os:
                case OperatingSystem.WINDOWS:
                    bin_folder = Path("binaries/x86_64-windows")
                case OperatingSystem.LINUX:
                    bin_folder = Path("binaries/x86_64-linux")
                case _:
                    raise ValueError("Invalid OS")
        case _:
            raise ValueError("Invalid version")


    model_name = model_description.getroot().attrib['modelName']

    if args.release:
        builder_path = Path("target/release")
        print("Package a release build")
    else:
        builder_path = Path("target/debug")
        print("Package a debug build")

    if not builder_path.exists():
        raise FileNotFoundError(
            "The build folder does not exist. Have you executed 'cargo build' and specified the correct build type for this script (release or not)?"
        )

    archive_path = Path(f"{model_name}.fmu")
    archive = zipfile.ZipFile(archive_path, "w")
    archive.write("modelDescription.xml", "modelDescription.xml")

    match os:
        case OperatingSystem.WINDOWS:
            archive.write(builder_path / f"{current_folder_name}.dll",     bin_folder / f"{model_name}.dll")
            archive.write(builder_path / f"{current_folder_name}.dll.lib", bin_folder / f"{model_name}.lib")
            archive.write(builder_path / f"{current_folder_name}.dll.exp", bin_folder / f"{model_name}.exp")
            archive.write(builder_path / f"{current_folder_name}.pdb",     bin_folder / f"{model_name}.pdb")
        case OperatingSystem.LINUX:
            shared_lib_path = builder_path / f"lib{current_folder_name}.so"
            shared_lib_path.chmod(0o755) # Not sure if this is necessary...

            archive.write(builder_path / f"lib{current_folder_name}.so", bin_folder / f"{model_name}.so")

    archive.close()
    archive_path.chmod(0o755) # Not sure if this is necessary...
