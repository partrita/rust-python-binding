# Using Rust from python

```bash
mkdir string_sum
cd string_sum
uv venv
source .venv/bin/activate
uv pip install maturin
uv run maturin init --bindings pyo3
uv run maturin develop
```


# Reference

- https://pyo3.rs/v0.24.2/
