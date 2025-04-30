# rust-python-binding

# Tool

## Maturin

formerly pyo3-pack. Build and publish crates with pyo3, cffi and uniffi bindings as well as rust binaries as python packages with minimal configuration. It supports building wheels for python 3.8+ on Windows, Linux, macOS and FreeBSD, can upload them to pypi and has basic PyPy and GraalPy support.

### Install

```bash
brew install maturin
```

### Usage

There are four main commands:

- `maturin new` creates a new cargo project with maturin configured.
- `maturin publish` builds the crate into python packages and publishes them to pypi.
- `maturin build` builds the wheels and stores them in a folder (target/wheels by default), but doesn't upload them. It's possible to upload those with twine or maturin upload.
- `maturin develop` builds the crate and installs it as a python module directly in the current virtualenv. Note that while `maturin develop` is faster, it doesn't support all the feature that running `pip install` after `maturin build` supports.


# Reference

- https://indosaram.github.io/rust-python-book/ch15-03.html
- https://hamza-senhajirhazi.medium.com/how-i-published-my-1st-rust-python-binding-package-cb44bc4e2e94
- https://www.maturin.rs/
- https://github.com/PyO3/maturin
- https://github.com/PyO3/maturin/issues/2314
