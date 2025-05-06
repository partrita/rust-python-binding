import time


def fib_sum(i):
    if i <= 0:
        return 0
    elif i == 1:
        return 1
    else:
        return fib_sum(i - 1) + fib_sum(i - 2)


start_time = time.time()
for i in range(1, 40):
    print(i, fib_sum(i))

end_time = time.time()
print(f"Time taken: {end_time - start_time} seconds")
