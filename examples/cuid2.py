from idika import with_cuid, n_with_cuid
import time

# generate a snowflake unique id
print("ID: ", with_cuid())

# generate many snowflake unique ids
start = time.time()
ids = n_with_cuid(1000000)
end = time.time()

print("IDs: ", len(ids))
print("Took: ", end - start, "s")

