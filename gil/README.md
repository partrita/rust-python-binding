이 프로젝트는 Python의 GIL(전역 인터프리터 잠금)을 Rust 코드에서 해제하고 다시 획득하는 방법을 보여주는 예제입니다. PyO3의 `py.allow_threads`를 사용하여 GIL을 해제하면, 계산 집약적인 작업을 Rust에서 병렬로 처리하는 동안 다른 Python 스레드가 동시에 실행될 수 있습니다. 이 예제는 멀티스레드 환경에서 Python 스레드와 Rust 스레드가 어떻게 상호작용하는지 보여주며, GIL을 우회하여 성능을 향상시키는 방법을 설명합니다.

# PyO3와 GIL

`fibonacci`에서는 단일 스레드 환경에서 러스트 코드를 실행하는 예제를 살펴봤습니다. 하지만 이 강의의 가장 큰 목표인 파이썬 GIL을 우회하는 러스트 패키지를 만들기 위해서는 멀티스레드 환경에서 러스트 코드를 실행해보아야 합니다. 이를 살펴보기 위해서 새로운 프로젝트를 생성합니다.


# GIL 획득과 해제

`py.allow_threads`는 PyO3에서 제공하는 메서드로, 클로저를 실행하는 동안 GIL을 일시적으로 해제할 수 있습니다. 이 메서드는 파이썬 인터프리터와 상호 작용할 필요가 없는 장기 실행 계산이 있고 다른 스레드가 파이썬 코드를 병렬로 실행하도록 허용하려는 경우에 유용할 수 있습니다. `py.allow_threads`를 사용하면 GIL의 해제 및 획득 시점을 세밀하게 제어할 수 있으므로 일부 상황에서 유용할 수 있습니다.

PyO3에서 GIL을 해제하는 다른 방법으로는 `Python::with_gil` 메서드를 사용하여 명시적으로 GIL을 획득하고 해제할 수도 있습니다.

일반적으로 파이썬 인터프리터와 상호 작용할 필요가 없는 장기 실행 계산이 있을 때마다 GIL을 해제하는 것이 좋습니다. 이렇게 하면 다른 스레드에서 Python 코드를 병렬로 실행할 수 있으므로 프로그램 성능이 향상될 수 있습니다.

```rust
use std::thread;
use std::time::Duration;

use pyo3::prelude::*;
use pyo3::types::PyList;

fn _double_list(nums: Vec<i32>) -> Vec<i32> {
    println!("Rust: Release GIL");
    println!("Rust: Wait 1 sec");
    thread::sleep(Duration::from_secs(1));
    let result = nums.into_iter().map(|n| n*2).collect();
    println!("Rust: Resume thread");
    result
}

#[pyfunction]
fn double_list(
    py: Python<'_>,
    list: Bound<'_, PyList>,
    result: Bound<'_, PyList>,
    idx: usize,
) -> PyResult<()> {
    println!("Rust: Enter double_list...");
    py.allow_threads(|| {
        println!("Rust: Release GIL...");
        thread::sleep(Duration::from_secs(1));
    });

    let doubled: Vec<i32> = list.extract::<Vec<i32>>()?.iter().map(|x| x * 2).collect();

    Python::with_gil(|py| {
        println!("Rust: Acquire GIL...");
        thread::sleep(Duration::from_secs(1));
        let py_list = PyList::new(py, &doubled)?;
        println!("Rust: Exit...");
        result.set_item(idx, py_list)
    })
}

#[pymodule]
fn gil(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = m.py();
    m.add_function(wrap_pyfunction!(double_list, py)?)?;
    Ok(())
}
```

- 위 코드의 `_double_list`는 러스트에서 벡터의 원소를 2배로 만듭니다.
- `double_list`는 파이썬에서 리스트를 받아와 GIL을 해제하고 `_double_list`를 실행합니다.
- GIL이 해제되었기 때문에 러스트에서 1초 대기하는 동안 파이썬 코드가 실행될 수 있습니다.

이제 `main.py`를 만들어 파이썬에서 sleep 함수를 사용해 GIL을 해제한 다음 러스트 코드와 번갈아가면서 실행되는 예제입니다.

```python
import time
import threading

from gil import double_list


def double_list_py(list, result, idx):
    print("Py: Enter double_list_py...")
    time.sleep(0.1)
    result[idx] = [x * 2 for x in list]
    print("Py: Exit...")


result = [[], []]
nums = [1, 2, 3]

t1 = threading.Thread(target=double_list_py, args=(nums, result, 0))
t2 = threading.Thread(target=double_list, args=(nums, result, 1))

t1.start()
t2.start()

t1.join()
t2.join()

print(f"Py: {result[0]}")
print(f"Rust: {result[1]}")
```