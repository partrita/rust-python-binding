In this tutorial we will wrap a version of the guessing game from The Rust Book to run in Python using pyo3.

# Create a new Rust project

First, create a new Rust library project using `cargo new --lib --edition 2021 guessing-game`. This will create a directory with the following structure.

```
guessing-game/
├── Cargo.toml
└── src
    └── lib.rs
```

## Edit Cargo.toml

```toml
[package]
name = "guessing-game"
version = "0.1.0"
edition = "2021"

[lib]
name = "guessing_game"
# "cdylib" is necessary to produce a shared library for Python to import from.
crate-type = ["cdylib"]

[dependencies]
rand = "0.9.0"

[dependencies.pyo3]
version = "0.24.0"
# "abi3-py38" tells pyo3 (and maturin) to build using the stable ABI with minimum Python version 3.8
features = ["abi3-py38"]
```

## Add pyproject.toml

```toml
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[tool.maturin]
# "extension-module" tells pyo3 we want to build an extension module (skips linking against libpython.so)
features = ["pyo3/extension-module"]
```

> process can be achieved by running `maturin new -b pyo3 guessing_game` then edit `Cargo.toml` to add abi3-py38 feature.


# Install and configure maturin (in a virtual environment)

```bash
uv venv
source .venv/bin/activate
```

# Program the guessing game in Rust


```rust
// src/lib.rs
use pyo3::prelude::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

#[pyfunction]
fn guess_the_number() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn guessing_game(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(guess_the_number, m)?)?;

    Ok(())
}
```

Build and install the module with maturin develop.


# Create a wheel for distribution

실제로 `maturin develop`은 휠 생성 단계를 건너뛰고 현재 환경에 직접 설치합니다. 반면에 `maturin build`는 배포할 수 있는 휠 파일을 생성합니다. 휠 파일 이름에는 지원되는 파이썬 버전, 플랫폼 및/또는 아키텍처에 해당하는 "태그"가 포함되어 있으므로 여러분의 파일 이름은 약간 다를 수 있습니다. 광범위하게 배포하려면 여러 플랫폼에서 빌드하고 다양한 Linux 배포판과 호환되는 휠을 빌드하기 위해 manylinux Docker 컨테이너를 사용해야 할 수도 있습니다.


- https://www.maturin.rs/tutorial.html