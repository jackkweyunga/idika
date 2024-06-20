from idika import with_snowflake, n_with_snowflake
import time

# generate a snowflake unique id
print("ID: ", with_snowflake())

# generate many snowflake unique ids
start = time.time()
ids = n_with_snowflake(1000000)
end = time.time()

print("IDs: ", len(ids))
print("Took: ", end - start, "s")

