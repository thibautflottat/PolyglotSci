import time

s = time.time()

for i in range(5000):
    for j in range(5000):
        pass

print(time.time()-s)