# 번역: 파이썬 개발자가 Rust-Python 바인딩을 작성하며 Rust를 배우는 여정

- 출처: https://hamza-senhajirhazi.medium.com/how-i-published-my-1st-rust-python-binding-package-cb44bc4e2e94

러스트에 대한 사람들의 좋은 피드백을 많이 들었습니다. 들을 때마다 러스트를 배워보고 싶다는 마음이 조금씩 더 커졌습니다. 그래서 Udemy 강의를 듣고, 공식 러스트 책을 읽고, Rustlings를 진행했습니다. 그러다 MongoDB와 유사한 SQLite 대체 데이터베이스를 찾던 중 우연히 PoloDB를 발견했습니다. 하지만 제 파이썬 앱에서 사용할 수 없었기에, 저자에게 바인딩을 작성할 의향이 있다면 돕겠다는 메시지를 남겼습니다. 몇 달 후, 저는 그의 프로젝트를 위한 rust-python 바인딩을 작성하고 있었고, 그것은 제 첫 번째 러스트 오픈소스 프로젝트가 되었습니다.

**이 글에서 주로 얻을 수 있는 내용:**

이 글은 파이썬 개발자인 제가 python-rust 바인딩을 작성하며 러스트를 배우려고 노력하는 과정과 겪었던 어려움에 대한 회고입니다. 따라서 이 작은 여정이 어떻게 진행되었는지 따라가며 저와 함께 배우게 될 것입니다.

  * 바인딩을 작성하기 위해 필요한 도구
  * 파이썬과 러스트 간의 프로젝트 디렉토리 구조를 구성하는 방법
  * GIL(Global Interpreter Lock)에 대한 기본적인 이해와 러스트와 파이썬이 객체를 서로 전달하는 방법
  * 마지막으로, 러스트 바인딩 파이썬 프로젝트를 패키징하는 방법

**바인딩을 작성하는 데 필요한 도구**

**파이썬 도구:**

  * **패키지 관리자:** 파이썬 의존성을 관리하기 위해 패키지 관리자가 필요합니다. Poetry 또는 UV를 추천합니다. 저는 최근에 UV를 사용하기 시작했고 매우 만족하고 있습니다.
  * **Maturin:** maturin은 pyo3를 사용하여 러스트 코드를 포함하는 파이썬 패키지를 빌드하고 패키징하는 데 도움이 되는 도구(이자 파이썬 패키지)입니다.

**러스트 도구:**

  * **패키지 관리자:** Cargo는 러스트의 공식 패키지 관리자입니다.
  * **Pyo3:** Pyo3는 파이썬에서 러스트로, 또는 그 반대로 객체를 전달하는 데 도움이 되는 내장 타입이 있는 라이브러리입니다.

**프로젝트 구조 레이아웃:**

아래는 우리 프로젝트의 구조 레이아웃을 나타냅니다.

```
polodb-python/
├── polodb/
│   ├── __init__.py
│   ├── core.py
│   └── version.py
├── src/
│   ├── helper_type_translator.rs
│   ├── lib.rs
│   └── py_database.rs
├── tests/
│   ├── conftest.py
│   └── test_database.py
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── LICENSE.txt
├── README.md
├── pyproject.toml
└── uv.lock
```

보시다시피 일반적인 러스트 프로젝트와 마찬가지로 `src/*.rs`와 `Cargo.toml`이 있고, 일반적인 파이썬 프로젝트와 마찬가지로 `polodb/*.py`와 `pyproject.toml`이 있습니다. "주로" 다른 점은 `lib.rs`에서 파이썬 코드에 노출하려는 러스트 클래스와 파이썬 함수를 정의하고, `pyproject.toml`에서 maturin을 러스트 코드를 파이썬 모듈로 빌드하기 위한 백엔드로 정의한다는 것입니다.

```toml
# pyproject.toml
[build-system]
requires = ["maturin>=1,<2"]
build-backend = "maturin"
```

```rust
// lib.rs
use pyo3::prelude::*;

mod helper_type_translator;
mod py_database;

use py_database::PyCollection;
use py_database::PyDatabase;

#[pymodule]
fn rust_polodb(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDatabase>()?;
    m.add_class::<PyCollection>()?;

    Ok(())
}
```

**파이썬에서 러스트 코드 사용하기:**

실제로 프로젝트의 `Cargo.toml`과 `pyptoject.toml`을 살펴보면, `cargo.toml`과 `pyproject.toml`에 추가적인 항목들이 있습니다. 이러한 항목의 한 예는 러스트의 의존성인 pyo3입니다. 이 의존성은 maturin이 러스트 코드를 파이썬에서 사용할 수 있는 `.so` 파일로 컴파일하는 데 사용됩니다. 그것이 무엇을 하고 어떻게 작동하는지 살펴봅시다.

프로젝트에서 `maturin develop`을 실행하면 다음과 같은 `target` 디렉토리가 생성됩니다.

```
target/
├── debug/*
│   ├── examples/*
│   ├── incremental/*
│   ├── maturin/
│   │   └── librust_polodb.dylib
│   ├── .cargo-lock
│   ├── librust_polodb.d
│   └── librust_polodb.dylib
├── wheels/
│   └── polodb_python-0.1.17-cp311-cp311-macosx_11_0_arm64.whl
└── ...
```

또한 파이썬 프로젝트에서 일반 파이썬 모듈처럼 임포트할 수 있는 `.so` 파일이 생성됩니다.

```
.venv/lib/python3.11/site-packages/rust_polodb/rust_polodb/
├── __pycache__/
│   └── __init__.cpython-311.pyc
├── __init__.py
└── rust_polodb.cpython-311-darwin.so
```

`maturin develop`을 사용하면 생성된 wheel 파일을 설치할 필요가 없습니다. maturin이 개발 환경(`.venv`)에 해당하는 모듈(`rust_polodb`)을 생성하기 때문입니다. `maturin build`를 대신 실행했다면 wheel 파일만 생성되었을 것이고, 이 경우 `pip install polodb_python-0.1.17-cp311-cp311-macosx_11_0_arm64.whl`와 같이 직접 수동으로 설치해야 했을 것입니다. 또한 wheel 파일의 이름은 패키지, 파이썬 및 OS 버전을 기반으로 구성됩니다.

**러스트 코드를 파이썬에 노출하기:**

위에서 프로젝트 구조 레이아웃에 대한 개요를 살펴보았습니다. 더 자세히 알고 싶다면 `cargo.toml`과 `pyproject.toml`을 살펴보고 어떻게 구성되어 있는지 확인할 수 있습니다.

이제 러스트-파이썬 바인딩이 어떻게 만들어지는지 좀 더 자세히 살펴보겠습니다.

이 프로젝트를 진행하면서 두 가지 어려움에 직면했습니다. 하나는 비교적 간단하게 해결할 수 있었지만, 다른 하나는 좀 더 깊이 파고들어야 했습니다. 첫 번째는 파이썬 클래스, 메서드 및 함수를 정의하는 방법이었고, 다른 하나는 PoloDB 라이브러리에서 반환되는 bson 문서 타입과 같은 복잡한 타입을 파이썬 객체로 변환하거나, 파이썬 GIL을 유지하면서 dict를 러스트 bson 문서 타입으로 변환하는 방법이었습니다.

**러스트에서 파이썬 클래스, 메서드 및 함수:**

```rust
use pyo3::prelude::*;

/// 두 숫자를 더하여 결과를 반환하는 간단한 함수입니다.
#[pyfunction]
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

/// 파이썬에 노출할 간단한 구조체입니다.
#[pyclass]
struct Greeting {
    name: String,
}

#[pymethods]
impl Greeting {
    /// 생성자: Greeting::new("Alice")
    #[new]
    fn new(name: String) -> Self {
        Greeting { name }
    }

    /// 인사 메시지를 반환하는 메서드입니다.
    fn hello(&self) -> PyResult<String> {
        Ok(format!("Hello, {}!", self.name))
    }
}

/// 이 모듈은 `pyo3_example`이라는 파이썬 모듈이며 클래스/함수의 모음입니다.
#[pymodule]
fn pyo3_example(py: Python, m: &PyModule) -> PyResult<()> {
    // 함수를 모듈에 추가합니다.
    m.add_function(wrap_pyfunction!(add_numbers, m)?)?;

    // 클래스를 모듈에 추가합니다.
    m.add_class::<Greeting>()?;

    Ok(())
}
```

이 최소한의 예제는 이해하기 매우 간단합니다. pyo3는 모듈을 노출하는 `#[pymodule]`, 클래스를 노출하는 `#[pyclass]`, 메서드를 노출하는 `#[pymethods]`, 파이썬 함수를 노출하는 `#[pyfunction]`과 같은 매크로를 제공합니다. 이 예제에서는 타입 변환에 신경 쓸 필요가 없습니다. 타입이 단순하고 pyo3가 타입 변환을 알아서 처리하기 때문입니다.

**2. 복잡한 객체의 타입 변환:**

러스트 PoloDB는 MongoDB의 API를 모방한 API를 제공합니다. 따라서 MongoDB에 익숙하다면 데이터베이스 객체(하나 이상의 컬렉션을 담는 객체), 컬렉션 객체(문서를 담는 리스트와 유사한 객체), 문서(엄격한 스키마를 가질 수도 있고 딕셔너리처럼 더 유연할 수도 있음)를 다룬다는 것을 알 것입니다. 제 파이썬 바인딩에서는 엄격한 스키마를 가진 문서를 다루지 않았습니다.

다음은 Database 클래스와 Collection 클래스를 정의하고 컬렉션 메서드에서 일부 타입 변환 함수를 사용하는 최소한의 예제입니다.

```rust
use crate::helper_type_translator::{
    bson_to_py_obj, convert_py_obj_to_document

};
use polodb_core::bson::Document;
use polodb_core::options::UpdateOptions;
use polodb_core::{Collection, Database};

#[pyclass]
pub struct PyDatabase {
    inner: Arc<Mutex<Database>>,
}

#[pymethods]
impl PyDatabase {
    #[new]
    fn new(path: &str) -> PyResult<Self> {
        let db_path = Path::new(path);
        match Database::open_path(db_path) {
            Ok(db) => Ok(PyDatabase {
                inner: Arc::new(Mutex::new(db)),
            }),
            Err(e) => Err(PyOSError::new_err(e.to_string())),
        }
    }
  //... 더 많은 메서드들이 있습니다.
}

#[pyclass]
pub struct PyCollection {
    inner: Arc<Collection<Document>>, // 스레드 안전한 공유 소유권을 위해 Arc 사용
}

#[pymethods]
impl PyCollection {
    pub fn name(&self) -> &str {
        self.inner.name()
    }
    pub fn insert_one(&self, doc: Py<PyDict>) -> PyResult<PyObject> {
        // 파이썬 GIL(Global Interpreter Lock) 획득
        Python::with_gil(|py| {
            let bson_doc: Document = match convert_py_obj_to_document(&doc.into_py_any(py).unwrap()) {
                Ok(d) => d,
                Err(e) => return Err(PyRuntimeError::new_err(format!("Insert many error: {}", e))),
            };
            // let bson_doc = convert_py_to_bson(doc);
            match self.inner.insert_one(bson_doc) {
                Ok(result) => {
                    // 러스트 결과로부터 파이썬 객체를 생성하고 반환합니다.
                    let py_inserted_id = bson_to_py_obj(py, &result.inserted_id);
                    let dict = PyDict::new(py);
                    let dict_ref = dict.borrow();
                    dict_ref.set_item("inserted_id", py_inserted_id)?;
                    Ok(dict.into_py_any(py).unwrap())

                    // Ok(Py::new(py, result)?.to_object(py))
                }
                Err(e) => {
                    // 오류 발생 시 파이썬 예외를 발생시킵니다.
                    Err(PyRuntimeError::new_err(format!("Insert error: {}", e)))
                }
            }
        })
    }
//... 더 많은 메서드들이 있습니다.
}
```

이 최소한의 예제에서는 제 파이썬 데이터베이스 클래스와 파이썬 컬렉션 클래스를 정의합니다. 이 클래스들은 기본적으로 PoloDB 객체를 감싸는 래퍼입니다. 이 래핑에서 `convert_py_obj_to_document`와 `bson_to_py_obj` 메서드에 주목해주십시오. 이 메서드들은 파이썬에서 러스트 Bson 문서로 또는 Bson 문서에서 파이썬 객체로 변환하는 데 사용됩니다. 또한 `|py|`로 표현되는 GIL을 획득한 컨텍스트 내에서 이를 수행하고 있다는 점에 유의하십시오. 이는 GIL을 획득하는 동안 파이썬 인터프리터가 GIL이 해제될 때까지 코드를 실행할 수 없음을 의미하며, 이는 파이썬에서 러스트로 또는 그 반대로 객체를 전달하는 동안 객체의 상태가 변경되는 것을 방지합니다. 파이썬에서 GIL이 어떻게 작동하는지 더 자세히 알고 싶다면 Larry Hastings의 PyCon 2015 비디오를 시청하는 것을 추천합니다.

글이 너무 길어지는 것을 막기 위해 변환이 어떻게 구현되었는지 보고 싶다면 `helper_type_translator.rs` 저장소 파일을 참조하십시오.

러스트 구현이 완료되고 `maturin develop`을 실행한 후, 파이썬에서 이러한 클래스나 함수를 임포트하여 사용할 수 있습니다. 제 경우에는 아래와 같이 PyMongo 클라이언트를 모방하려고 시도하는 작은 래퍼를 만들었습니다.

```python
from rust_polodb import PyDatabase, PyCollection


class PoloDB:

    def __init__(self, path: str) -> None:
        self._path = path
        self.__rust_db = PyDatabase(self._path)

   def collection(self, name):
        if name not in self.list_collection_names():
            self.__rust_db.create_collection(name)
        return Collection(self.__rust_db.collection(name))
   ...
....

class Collection:
    def __init__(self, rust_collection) -> None:
        self.__rust_collection: PyCollection = rust_collection

    def name(self):
        return self.__rust_collection.name()

    def insert_one(self, entry: dict):
        return self.__rust_collection.insert_one(entry)
    ...
...
```

**배운 점**

rust-python 바인딩을 작성하는 것은 처음에는 약간 어렵거나 그렇게 보일 수 있지만, 그렇게 어렵지 않습니다. 특히 파이썬에서 성능 병목 현상이 발생하는 경우 강력한 도구를 하나 더 추가하는 것입니다. 이 프로젝트를 통해 러스트를 좀 더 깊이 이해하게 되었고, 매우 아름답게 설계된 언어라고 생각하여 계속 학습하기로 결정했습니다. 막혔을 때는 ChatGPT가 수명과 같은 일부 러스트 개념이나 bson 타입 변환의 일부를 이해하는 데 도움이 되었으므로 파이썬과 함께 러스트를 배우는 데 투자하는 것을 추천합니다.

**핵심 내용 및 요약:**

  * **파이썬 + 러스트 유용한 조합:** 이미 파이썬을 알고 있다면 러스트를 조금 아는 것만으로도 영향력을 높일 수 있으며, 파이썬과 코드 품질에 대한 이해도를 높일 수도 있습니다.
  * **툴링:** 보시다시피 사용할 도구(패키지 관리자, cargo, UV, pyo3 라이브러리), 프로젝트 레이아웃 방법, pyproject.toml 및 cargo.toml 구성 방법을 알아야 합니다.
  * **타입 변환:** pyo3는 이미 기본적인 타입 변환을 제공하지만, 복잡한 타입의 경우 약간의 추가 작업과 pyo3 API에 대한 좀 더 깊은 이해가 필요합니다.

**추가 보너스**

Developer’s Voice 팟캐스트의 한 에피소드가 매우 즐거웠습니다. 이 에피소드에서는 pyo3가 어떻게 구현되었는지, 파이썬의 내부 구조, 변환 방법 등에 대해 설명합니다. [여기](https://www.google.com/search?q=%EB%A7%81%ED%81%AC)에서 확인해보세요.

**참고 자료:**

  * [polodb-python git repo](https://www.google.com/search?q=%EB%A7%81%ED%81%AC)
  * [GIL python](https://www.google.com/search?q=%EB%A7%81%ED%81%AC)
  * [PoloDB’s doc](https://www.google.com/search?q=%EB%A7%81%ED%81%AC)
  * [Udemy Course Rust](https://www.google.com/search?q=%EB%A7%81%ED%81%AC)
  * [Rustlings](https://www.google.com/search?q=%EB%A7%81%ED%81%AC)
  * [Rust book](https://www.google.com/search?q=%EB%A7%81%ED%81%AC)
  * [Python package manager UV](https://www.google.com/search?q=%EB%A7%81%ED%81%AC)
  * [Developer’s voice Rust-Python Pyo3](https://www.google.com/search?q=%EB%A7%81%ED%81%AC)
