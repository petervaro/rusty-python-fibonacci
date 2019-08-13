from itertools import islice
from rusty_fibonacci import Fibonacci

print(tuple(islice(Fibonacci(), 16)))
