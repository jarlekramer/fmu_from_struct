from pathlib import Path

import zipfile

import xml.etree.ElementTree as ElementTree

import argparse

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Build FMU from Rust model')
    parser.add_argument('--release', action='store_true')

    args = parser.parse_args()

    current_folder_name = Path.cwd().parts[-1]    

    # Check that modelDescription.xml exists. If not, the user is either in a wrong folder, or "cargo build" is not executed
    if not Path("modelDescription.xml").exists():
        raise FileNotFoundError("modelDescription.xml not found. Are you in the right folder? Have you executed 'cargo build'?")

    model_description = ElementTree.parse("modelDescription.xml")

    fmi_version = int(float(model_description.getroot().attrib['fmiVersion']))

    match fmi_version:
        case 2:
            bin_folder = Path("binaries/win64")
        case 3:
            bin_folder = Path("binaries/x86_64-windows")
        case _:
            raise ValueError("Invalid version")

    
    model_name = model_description.getroot().attrib['modelName']

    if args.release:
        builder_path = Path("target/release")
        print("Package a release build")
    else:
        builder_path = Path("target/debug")
        print("Package a debug build")

    archive = zipfile.ZipFile(f"{model_name}.fmu", "w")
    archive.write("modelDescription.xml", "modelDescription.xml")

    archive.write(builder_path / f"{current_folder_name}.dll",     bin_folder / f"{model_name}.dll")
    archive.write(builder_path / f"{current_folder_name}.dll.lib", bin_folder / f"{model_name}.lib")
    archive.write(builder_path / f"{current_folder_name}.dll.exp", bin_folder / f"{model_name}.exp")
    archive.write(builder_path / f"{current_folder_name}.pdb",     bin_folder / f"{model_name}.pdb")

    archive.close()

