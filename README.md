# Maturin을 사용한 rust-python-binding

이전에는 pyo3-pack으로 알려졌던 Maturin은 최소한의 설정으로 pyo3, cffi, uniffi 바인딩뿐만 아니라 Rust 바이너리를 파이썬 패키지로 빌드하고 게시할 수 있게 해줍니다. Windows, Linux, macOS 및 FreeBSD에서 파이썬 3.8 이상을 위한 휠 빌드를 지원하며, PyPI에 업로드할 수 있고 기본적인 PyPy 및 GraalPy 지원 기능을 갖추고 있습니다.

## 설치법

최신 릴리스에서 바이너리를 다운로드하거나 `brew`를 사용하여 설치할 수 있습니다.

```bash
brew install maturin
```

> 참고: 추천하지는 않지만 `pip install maturin`도 사용 가능합니다.

## 사용법


주요 명령어는 네 가지입니다.

- `maturin new`: `maturin`이 설정된 새로운 cargo 프로젝트를 생성합니다.
- `maturin publish`: 크레이트를 파이썬 패키지로 빌드하고 PyPI에 게시합니다.
- `maturin build`: 휠을 빌드하여 기본적으로 `target/wheels` 폴더에 저장하지만 업로드하지는 않습니다. `twine` 또는 `maturin upload`를 사용하여 업로드할 수 있습니다.
- `maturin develop`: 크레이트를 빌드하고 현재 가상 환경(virtualenv)에 파이썬 모듈로 설치합니다.

`maturin`은 추가 구성 파일이 필요 없으며 기존의 `setuptools-rust` 또는 `milksnake` 구성과 충돌하지 않습니다. `tox`와 같은 테스트 도구와 통합할 수도 있습니다. 다양한 바인딩에 대한 예제는 `test-crates` 폴더에 있습니다.

패키지 이름은 cargo 프로젝트의 이름, 즉 `Cargo.toml` 파일의 `[package]` 섹션에 있는 `name` 필드가 됩니다. 임포트할 때 사용하는 모듈의 이름은 `[lib]` 섹션의 `name` 값(기본적으로 패키지 이름과 동일)이 됩니다. 바이너리의 경우 단순히 cargo가 생성한 바이너리 이름입니다.

`maturin build` 및 `maturin develop` 명령을 사용할 때 `-r` 또는 `--release` 플래그를 추가하여 성능 최적화된 프로그램을 컴파일할 수 있습니다.

## 파이썬 패키징 기본 사항

파이썬 패키지는 휠(wheel)이라는 빌드된 형식과 소스 배포(sdist)라는 두 가지 형식으로 제공되며 둘 다 아카이브입니다. 휠은 모든 파이썬 버전, 인터프리터(주로 cpython 및 pypy), 운영 체제 및 하드웨어 아키텍처(순수 파이썬 휠의 경우)와 호환될 수 있으며 특정 플랫폼 및 아키텍처(예: ctypes 또는 cffi 사용 시) 또는 특정 아키텍처 및 운영 체제의 특정 파이썬 인터프리터 및 버전(예: pyo3 사용 시)으로 제한될 수 있습니다.

패키지에 `pip install`을 사용하면 pip는 일치하는 휠을 찾아 설치하려고 시도합니다. 찾지 못하면 소스 배포를 다운로드하고 현재 플랫폼에 대한 휠을 빌드합니다. 이 과정에는 올바른 컴파일러가 설치되어 있어야 합니다. 휠 설치는 일반적으로 소스 배포 설치보다 훨씬 빠릅니다.

`pip install`로 설치할 수 있는 패키지를 게시하려면 공식 패키지 저장소인 PyPI에 업로드해야 합니다. 테스트를 위해 `pip install --index-url https://test.pypi.org/simple/`와 함께 사용할 수 있는 Test PyPI를 대신 사용할 수 있습니다. Linux용 게시의 경우 manylinux docker 컨테이너를 사용해야 하며 깃헙 저장소에서 게시하는 경우 PyO3/maturin-action github 액션을 사용할 수 있습니다.

## 혼합 Rust/Python 프로젝트

혼합 Rust/Python 프로젝트를 만들려면 `Cargo.toml` 옆에 모듈 이름(`Cargo.toml`의 `lib.name`)과 동일한 이름의 폴더를 만들고 해당 폴더에 파이썬 소스를 추가합니다.

```
my-project
├── Cargo.toml
├── my_project
│   ├── __init__.py
│   └── bar.py
├── pyproject.toml
├── README.md
└── src
    └── lib.rs
```

`pyproject.toml`에서 `tool.maturin.python-source`를 설정하여 다른 파이썬 소스 디렉토리를 지정할 수도 있습니다. 예를 들어 다음과 같습니다.

```toml
# pyproject.toml
[tool.maturin]
python-source = "python"
module-name = "my_project._lib_name"
```

그러면 프로젝트 구조는 다음과 같이 됩니다.

```
my-project
├── Cargo.toml
├── python
│   └── my_project
│       ├── __init__.py
│       └── bar.py
├── pyproject.toml
├── README.md
└── src
    └── lib.rs
```

> 이 구조는 흔히 발생하는 `ImportError` 함정을 피하기 위해 권장됩니다.

`maturin`은 네이티브 확장 기능을 파이썬 폴더에 모듈로 추가합니다. `develop`을 사용하면 `maturin`은 네이티브 라이브러리와 cffi의 경우 glue 코드도 파이썬 폴더에 복사합니다. 이러한 파일은 `.gitignore`에 추가해야 합니다.

cffi를 사용하면 `from .my_project import lib`을 수행한 다음 `lib.my_native_function`을 사용할 수 있으며, pyo3를 사용하면 `from .my_project import my_native_function`을 직접 사용할 수 있습니다.

`maturin develop` 후 pyo3를 사용한 예시 레이아웃:

```
my-project
├── Cargo.toml
├── my_project
│   ├── __init__.py
│   ├── bar.py
│   └── _lib_name.cpython-36m-x86_64-linux-gnu.so
├── README.md
└── src
    └── lib.rs
```

이 작업을 수행할 때는 코드에서 모듈 이름을 `module-name`의 마지막 부분과 일치하도록 설정해야 합니다(패키지 경로는 포함하지 마십시오).

```rust
#[pymodule]
#[pyo3(name="_lib_name")]
fn my_lib_name(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<MyPythonRustClass>()?;
    Ok(())
}
```

## 파이썬 메타데이터

`maturin`은 파이썬 패키지 메타데이터를 위한 표준([PEP621](https://www.python.org/dev/peps/pep-0621/))을 지원하며, `pyproject.toml`에서 파이썬 패키지 메타데이터를 지정할 수 있습니다. `maturin`은 `Cargo.toml`과 `pyproject.toml`의 메타데이터를 병합하며, `pyproject.toml`이 `Cargo.toml`보다 우선합니다.

파이썬 종속성을 지정하려면 `pyproject.toml`의 `[project]` 섹션에 `dependencies` 목록을 추가합니다. 이 목록은 setuptools의 `install_requires`와 동일합니다.

```toml
# pyproject.toml
[project]
name = "my-project"
dependencies = ["flask~=1.1.0", "toml==0.10.0"]
```

Pip은 프로그램의 특정 함수를 실행하는 셸 명령어인 콘솔 스크립트 추가를 허용합니다. `[project.scripts]` 섹션에 콘솔 스크립트를 추가할 수 있습니다. 키는 스크립트 이름이고 값은 `some.module.path:class.function` 형식의 함수 경로입니다. 여기서 `class` 부분은 선택 사항입니다. 함수는 인자 없이 호출됩니다. 예시:

```toml
# pyproject.toml
[project.scripts]
get_42 = "my_project:DummyClass.get_42"
```

`pyproject.toml`의 `project.classifiers` 아래에 trove 분류자를 지정할 수도 있습니다.

```toml
# pyproject.toml
[project]
name = "my-project"
classifiers = ["Programming Language :: Python"]
```

## 소스 배포

`maturin`은 `pyproject.toml`을 통한 빌드를 지원합니다. 이를 사용하려면 `Cargo.toml` 옆에 다음 내용으로 `pyproject.toml` 파일을 만듭니다.

```toml
# pyproject.toml
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"
```

`[build-system]` 항목이 있는 `pyproject.toml` 파일이 있으면 `--sdist`가 지정되었을 때 `maturin`은 패키지의 소스 배포를 빌드할 수 있습니다. 소스 배포에는 `cargo package`와 동일한 파일이 포함됩니다. 소스 배포만 빌드하려면 값 없이 `--interpreter`를 전달합니다.

그런 다음 예를 들어 `pip install ..`으로 패키지를 설치할 수 있습니다. `pip install . -v`를 사용하면 cargo 및 maturin의 출력을 확인할 수 있습니다.

`compatibility`, `skip-auditwheel`, `bindings`, `strip` 및 `features`와 같은 일반적인 Cargo 빌드 옵션을 `[tool.maturin]` 아래에서 `maturin`을 직접 실행할 때와 같은 방식으로 사용할 수 있습니다. `bindings` 키는 cffi 및 bin 프로젝트의 경우 자동으로 감지할 수 없으므로 필수입니다. 현재 모든 빌드는 릴리스 모드입니다(자세한 내용은 [이 스레드](https://discuss.python.org/t/pep-517-debug-vs-release-builds/1924) 참조).

cffi 바인딩을 사용한 non-manylinux 빌드의 경우 다음을 사용할 수 있습니다.

```toml
# pyproject.toml
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[tool.maturin]
bindings = "cffi"
compatibility = "linux"
```

`manylinux` 옵션은 이전 버전의 `maturin`과의 호환성을 위해 `compatibility`의 별칭으로도 허용됩니다.

컴파일 중에 사용할 임의의 파일을 sdist에 포함하려면 `format`이 `sdist`로 설정된 `path` glob 배열로 `include`를 지정합니다.

```toml
# pyproject.toml
[tool.maturin]
include = [{ path = "path/**/*", format = "sdist" }]
```

[pypa/pip#6041](https://github.com/pypa/pip/issues/6041)의 임시 해결 방법으로 소스 배포만 빌드하는 `maturin sdist` 명령이 있습니다.

## Manylinux 및 auditwheel

이식성을 위해 Linux의 네이티브 파이썬 모듈은 기본적으로 모든 곳에 설치된 매우 적은 수의 라이브러리만 동적으로 연결해야 합니다. 이것이 manylinux라는 이름의 유래입니다. pypa는 manylinux 규칙 준수를 보장하기 위해 특수 docker 이미지와 auditwheel이라는 도구를 제공합니다. Linux PyPI에 널리 사용 가능한 휠을 게시하려면 manylinux docker 이미지를 사용하거나 zig로 빌드해야 합니다.

Rust 컴파일러 버전 1.64부터는 최소 glibc 2.17이 필요하므로 최소 manylinux2014를 사용해야 합니다. 게시의 경우 manylinux 플래그를 사용하여 이미지와 동일한 manylinux 버전을 적용하는 것이 좋습니다. 예를 들어 `quay.io/pypa/manylinux2014_x86_64`에서 빌드하는 경우 `--manylinux 2014`를 사용하십시오. PyO3/maturin-action github 액션은 예를 들어 `manylinux: 2014`를 설정하면 이를 자동으로 처리합니다.

`maturin`에는 auditwheel의 재구현이 포함되어 생성된 라이브러리를 자동으로 검사하고 휠에 적절한 플랫폼 태그를 지정합니다. 시스템의 glibc가 너무 최신이거나 다른 공유 라이브러리를 연결하면 `linux` 태그를 할당합니다. `--manylinux off`를 사용하여 이러한 검사를 수동으로 비활성화하고 네이티브 Linux 대상을 직접 사용할 수도 있습니다.

완전한 manylinux 호환성을 위해서는 CentOS docker 컨테이너에서 컴파일해야 합니다. `pyo3/maturin` 이미지는 manylinux2014 이미지를 기반으로 하며 `maturin` 바이너리에 인수를 전달합니다. 다음과 같이 사용할 수 있습니다.

```bash
docker run --rm -v $(pwd):/io ghcr.io/pyo3/maturin build --release  # 또는 다른 maturin 인수
```

이 이미지는 매우 기본적인 이미지이며 파이썬, maturin, Rust만 포함하고 있습니다. 추가 도구가 필요한 경우 manylinux 컨테이너 내에서 새로운 명령을 실행해야 합니다.

# Reference

`maturin`에 대한 더 자세한 정보와 고급 사용법은 다음 리소스를 참조하십시오.

- Maturin GitHub 저장소: [https://github.com/PyO3/maturin](https://github.com/PyO3/maturin)
- PyO3 문서: [https://pyo3.rs/v0.20.1/](https://pyo3.rs/v0.20.1/)
- Rust Cargo 문서: [https://doc.rust-lang.org/cargo/](https://doc.rust-lang.org/cargo/)
- Python Packaging User Guide: [https://packaging.python.org/en/latest/](https://packaging.python.org/en/latest/)
- https://indosaram.github.io/rust-python-book/ch15-03.html
- https://hamza-senhajirhazi.medium.com/how-i-published-my-1st-rust-python-binding-package-cb44bc4e2e94
- https://www.maturin.rs/
- https://github.com/PyO3/maturin
- https://github.com/PyO3/maturin/issues/2314
