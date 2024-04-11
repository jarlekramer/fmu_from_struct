from setuptools import setup, find_namespace_packages
import pathlib

here = pathlib.Path(__file__).parent.resolve()

# Get the long description from the README file
long_description = (here / "README.md").read_text(encoding="utf-8")

setup(
    name="fmu_build_utils",  
    version="0.1",  
    description="Python helper functionality to build FMUs",  
    long_description=long_description, 
    long_description_content_type="text/markdown", 
    url="https://github.com/jarlekramer/fmu_from_struct",  
    author="Jarle Vinje Kramer", 
    author_email="jarlekramer@gmail.com",  
    packages=find_namespace_packages(),
    python_requires=">=3.10, <4",
)
