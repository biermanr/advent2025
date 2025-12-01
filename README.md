# advent2025
Advent of Code 2025

Built as a python package with rust extensions using pyo3 and managed with maturin.
Created a new virtual environment (.venv/) with python 3.11.3

Run `maturin develop` to build the rust extensions and install the package in the virtual environment.

Created GHA CI with:
`maturin generate-ci github > .github/workflows/CI.yml`
