from pathlib import Path

import zipfile

if __name__ == "__main__":

    builder_path = Path("target/debug")
    bin_folder = Path("binaries/x86_64-windows")
    fmu_name = "spring"

    archive = zipfile.ZipFile(f"{fmu_name}.fmu", "w")
    archive.write("modelDescription.xml", "modelDescription.xml")

    archive.write(builder_path / f"{fmu_name}.dll", bin_folder / f"{fmu_name}.dll")
    archive.write(builder_path / f"{fmu_name}.dll.lib", bin_folder / f"{fmu_name}.lib")
    archive.write(builder_path / f"{fmu_name}.dll.exp", bin_folder / f"{fmu_name}.exp")
    archive.write(builder_path / f"{fmu_name}.pdb", bin_folder / f"{fmu_name}.pdb")

    archive.close()

