import math
import time


def totien_numbers(n):
    # return a generator of totien numbers
    for i in range(1, n):
        if math.gcd(i, n) == 1:
            yield i


def totien(n):
    # return totien number of n
    return len([i for i in totien_numbers(n)])


t1 = time.time()
print(totien(56088))
print("Execution time: ", (time.time() - t1) * 1000, "ms")
