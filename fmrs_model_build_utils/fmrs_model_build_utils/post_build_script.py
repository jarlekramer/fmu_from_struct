from pathlib import Path

import zipfile

import argparse

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("name", type=str)
    parser.add_argument("--version", type=int, default=2)

    args = parser.parse_args()

    # Check that modelDescription.xml exists. If not, the user is either in a wrong folder, or "cargo build" is not executed
    if not Path("modelDescription.xml").exists():
        raise FileNotFoundError("modelDescription.xml not found. Are you in the right folder? Have you executed 'cargo build'?")

    builder_path = Path("target/debug")

    match args.version:
        case 2:
            bin_folder = Path("binaries/win64")
        case 3:
            bin_folder = Path("binaries/x86_64-windows")
        case _:
            raise ValueError("Invalid version")

    archive = zipfile.ZipFile(f"{args.name}.fmu", "w")
    archive.write("modelDescription.xml", "modelDescription.xml")

    archive.write(builder_path / f"{args.name}.dll",     bin_folder / f"{args.name}.dll")
    archive.write(builder_path / f"{args.name}.dll.lib", bin_folder / f"{args.name}.lib")
    archive.write(builder_path / f"{args.name}.dll.exp", bin_folder / f"{args.name}.exp")
    archive.write(builder_path / f"{args.name}.pdb",     bin_folder / f"{args.name}.pdb")

    archive.close()

