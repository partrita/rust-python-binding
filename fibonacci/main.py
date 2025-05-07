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
