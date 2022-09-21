## Pyo3 Dataframe
Explore using maturin to generate Python package from Rust. Just like many python package integrate with C to improve performance, like numpy, Rust could do the same thing using maturin and pyo3.


## Prerequisite
- Python 3.8+
- Rust 1.60+

## Folder Structure
```
examples/pyo3_dataframe
├── python
│   └── pyo3_dataframe
└── src
    └── pyo3_dataframe
```
`python/pyo3_dataframe` stores all python code

`src/*` stores all rust code

## Setup
0. (Optional) Enable your Python virtual environment
1. Install required Python packages, run `pip3 install -r ./python/pyo3_dataframe/requirements.txt`
2. Build Python package from Rust, run `maturin develop --release`
3. Open `python/pyo3_dataframe/main.ipynb` to explore
    - make sure to select correct kernel in Jupyter Notebook
