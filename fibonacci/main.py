import time
from fibonacci import run


def pyrun(n: int):
    if n < 2:
        return n
    return pyrun(n - 1) + pyrun(n - 2)


N = 35

start = time.time()
result = pyrun(N)
print(f"python: {time.time() - start:.2f}, result: {result}")

start = time.time()
result = run(N)
print(f"rust: {time.time() - start:.2f}, result: {result}")
