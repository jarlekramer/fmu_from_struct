from pathlib import Path

import zipfile

import argparse

if __name__ == "__main__":

    parser = argparse.ArgumentParser()
    parser.add_argument("--version", type=int, default=2)

    args = parser.parse_args()


    builder_path = Path("target/debug")

    match args.version:
        case 2:
            bin_folder = Path("binaries/win64")
        case 3:
            bin_folder = Path("binaries/x86_64-windows")
        case _:
            raise ValueError("Invalid version")
        
    fmu_name = "spring"

    archive = zipfile.ZipFile(f"{fmu_name}.fmu", "w")
    archive.write("modelDescription.xml", "modelDescription.xml")

    archive.write(builder_path / f"{fmu_name}.dll",     bin_folder / f"{fmu_name}.dll")
    archive.write(builder_path / f"{fmu_name}.dll.lib", bin_folder / f"{fmu_name}.lib")
    archive.write(builder_path / f"{fmu_name}.dll.exp", bin_folder / f"{fmu_name}.exp")
    archive.write(builder_path / f"{fmu_name}.pdb",     bin_folder / f"{fmu_name}.pdb")

    archive.close()

