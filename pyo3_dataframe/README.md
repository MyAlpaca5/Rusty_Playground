# PyO3 Dataframe
Explore using `maturin` to transform Rust functions into Python package. Just like many Python package using C to improve performance, such as `numpy`, Rust could do the same thing using `maturin` and `pyo3`.

This solution uses a MPSC channel in Rust to communicate between Rust and Python. Rust part will start the computation in a new thread, and then create the channel and wrap the Receiver part of the channel into a Python iterator before returning the iterator back to Python. Python part will just read from the iterator till the underline channel were closed.

# Prerequisite
- Python 3.10 (3.11 may not work)
- Rust 1.65

# Setup
0. (Optional) Enable your Python virtual environment
1. Install required Python packages, run `pip3 install -r ./python/requirements.txt`
2. Build Python package from Rust, run `maturin develop --release`
3. Open `./python/main.ipynb` to explore
    - make sure to select correct kernel in Jupyter Notebook