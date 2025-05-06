import time
from fibonacci import run

start_time = time.time()
for i in range(1, 40):
    print(i, run(i))

end_time = time.time()
print(f"Time taken: {end_time - start_time} seconds")
