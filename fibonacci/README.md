# 파이썬 바인딩이란?

파이썬 바인딩은 다른 프로그래밍 언어로 작성된 코드를 파이썬에서 사용하는 것을 의미합니다.

파이썬용 러스트 바인딩을 사용하면 파이썬에서 러스트로 함수를 호출하고 데이터를 전달할 수 있으므로 두 언어의 강점을 모두 활용할 수 있습니다. 이 기능은 테스트를 거쳐 안정적으로 작성된 대규모 라이브러리를 파이썬에서 활용하거나 파이썬 코드의 특정 섹션을 러스트로 변환하여 속도를 높이고자 하는 경우에 유용합니다.

파이썬에서 러스트 바인딩을 생성하는 데 가장 널리 알려진 프로젝트는 PyO3입니다. 이 프로젝트는 러스트로 파이썬 모듈을 작성하거나 파이썬 런타임을 러스트 바이너리에 임베드하는 데 사용할 수 있습니다. PyO3는 파이썬 패키징 및 바인딩이 포함된 러스트 상자를 작성하는 도구인 `Maturin`이라는 또 다른 프로젝트를 활용합니다.

## PyO3

PyO3는 파이썬에서 러스트 코드를 실행할 수 있고, 반대로 러스트에서 파이썬 코드를 실행할 수 있도록 도와주는 크레이트입니다.

## Maturin

maturin은 최소한의 구성으로 러스트로 작성한 파이썬 패키지를 빌드할 수 있는 도구입니다.

# 라이브러리 크레이트 만들기

#[pyfunction]은 PyO3 라이브러리에서 제공하는 어트리뷰트로, 러스트 함수를 파이썬 함수로 정의하는 데 사용할 수 있습니다. 러스트 함수에 #[pyfunction] 어트리뷰트를 추가하면 PyO3는 해당 함수가 일반 파이썬 함수인 것처럼 파이썬에서 호출할 수 있는 코드를 생성합니다.

#[pymodule]은 PyO3 라이브러리에서 제공하는 어트리뷰트로, 러스트 함수를 파이썬 모듈로 정의하는 데 사용할 수 있습니다. 러스트 함수에 #[pymodule] 어트리뷰트를 추가하면 PyO3는 해당 함수를 파이썬 모듈의 초기화 함수로 사용할 수 있는 코드를 생성합니다.

모듈에 함수를 추가하려면 add_function 메서드를 사용합니다. 이렇게 하면 모듈 내에서 함수를 호출 가능한 객체로 사용할 수 있습니다.

```rust
use pyo3::prelude::*;

#[pyfunction]
fn fib(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

#[pymodule]
fn fibonacci(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = m.py();
    m.add_function(wrap_pyfunction!(fib, py)?)?;
    Ok(())
}
```

# 파이썬에서 러스트 코드 실행해 보기

## 개발 모드로 빌드해보기

`maturin develop` 명령어를 사용하면, 러스트 패키지를 빌드한 다음 파이썬 가상환경에 패키지를 자동으로 설치해줍니다. 이때 러스트 컴파일 타겟이 [unoptimized + debuginfo]가 되는데, 빠른 개발을 위해 코드 성능보다는 컴파일 속도를 중요하게 생각한 옵션입니다.

`main.py` 파일을 만들고 다음 코드를 추가합니다. 파이썬으로 피보나치 수열을 구하는 함수 pyrun 을 추가해 러스트 구현체와 성능을 비교해봅니다.

```python
import time
from fibonacci import fib as rust_fib


def py_fib(n: int):
    """파이썬으로 구현한 피보나치 수열 계산 함수 (재귀 방식)"""
    if n <= 0:
        return 0
    elif n == 1:
        return 1
    else:
        return py_fib(n - 1) + py_fib(n - 2)


N = 42

# 파이썬으로 계산한 피보나치 수열 결과 및 시간 측정
start_python = time.time()
python_result = py_fib(N)
python_time = time.time() - start_python
print(
    f"Python으로 계산한 결과 (N={N}): {python_result} (소요 시간: {python_time:.2f} 초)"
)

# Rust로 계산한 피보나치 수열 결과 및 시간 측정
start_rust = time.time()
rust_result = rust_fib(N)
rust_time = time.time() - start_rust
print(f"Rust로 계산한 결과 (N={N}): {rust_result} (소요 시간: {rust_time:.2f} 초)")

# 두 결과가 동일한지 확인
if python_result == rust_result:
    print("파이썬과 Rust 계산 결과가 동일합니다.")
else:
    print("파이썬과 Rust 계산 결과가 다릅니다!")
```

코드 실행

```bash
uv run python main.py
```

결과

```
Python으로 계산한 결과 (N=42): 267914296 (소요 시간: 21.13 초)
Rust로 계산한 결과 (N=42): 267914296 (소요 시간: 0.00 초)
파이썬과 Rust 계산 결과가 동일합니다.
```

## 릴리즈 모드로 빌드해보기

빌드 옵션을 --release 로 주면, 러스트 코드를 최대한 최적화해서 컴파일한 바이너리가 패키지로 만들어지게 됩니다. 컴파일 타겟이 [optimized]인 걸 알 수 있습니다.
