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

