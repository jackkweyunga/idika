from idika import with_cuid, n_with_cuid
import time

N = 15
C = 1000000
# generate a snowflake unique id
print("ID: ", with_cuid(N))

# generate many snowflake unique ids
start = time.time()
ids = n_with_cuid(C, N)
end = time.time()

print("IDs: ", len(ids))
print("Took: ", end - start, "s")
