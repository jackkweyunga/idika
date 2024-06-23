from idika import with_cuid, n_with_cuid
import time

def process_id(id):
    # do some functionality
    print(id)

N = 15
C = 100
# generate a cuid2 unique id
print("ID: ", with_cuid(N))

# generate many cuid2 unique ids
# Pipe the result to a function
start = time.time()
ids = n_with_cuid(C, N).pipe(process_id)
end = time.time()

print("IDs: ", len(ids.result))
print("Took: ", end - start, "s")
