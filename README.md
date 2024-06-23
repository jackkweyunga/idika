# idika
A simple unique ids generator for python implemented in rust

Algorithms / Implimentations

 - [x] cuid2
 - [ ] sonyflakes

# Installation

```shell
pip install idika
```

## cuid2

```python
import idika

# generate one id
# 10 -> id length
idika.with_cuid(10)
# output: o13q75qk9q


# generate multiple ids
# 10000 -> Count , 10 -> length
idika.n_with_cuid(1000, 10)
"""
output:
[
 'mdse9rnpj1',
 'ub324hvoxm',
 'f1rcv9ysrr',
 'jzeweia5ut',
 'k12lt092sc',
 'k11j9jpbb7',
 ...10000
]
"""

# Pipe
# Run a certain function on all ids generated.
def process_id(id):
    # doing some processing
    # ... e.g database calls 
    print(id)

idika.n_with_cuid(1000, 10).pipe(process_id)

```


