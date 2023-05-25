import sys
import time
import matplotlib.pyplot as plt
import numpy as np
from bose_hubbard import System

# Reading system from yaml in stdin
system = System(sys.stdin.read())

# Displaying system
print("\nParsed system:\n")
system.show()
print()

# Solving system
print("Solving...\n")
start = time.time()
answer = system.solve()
elapsed = time.time() - start
print(f"Solved in {elapsed:.0f} seconds\n")

# Saving to ./answer.csv
print("Saving answer to ./answer.csv\n")
with open("answer.csv", "w") as f:
    for row in answer:
        f.write(' '.join([f"{e:.3f}" for e in row]) + '\n')

print("Done")

# Plotting
answer = np.loadtxt("answer.csv")

fig, ax = plt.subplots()

ax.imshow(answer)

plt.show()