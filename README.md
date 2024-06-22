# idika
A simple unique ids generator for python implemented in rust

Algorithms / Implimentations

 - [ x ] cuid2
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

# generate multiple ids
# 10000 -> Count , 15 -> length
idika.n_with_cuid(1000, 15)

```


