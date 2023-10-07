import os
import random
import hashlib

# GenPair: List[(prototype, size)]
def gen_nice_test(root, layer_pairs, edge_pairs, lb = 1, ub = 10):
    current_layer = [root]
    for layer_prototype, size in layer_pairs:
        print(layer_prototype, size)
        if size is None:
            size = random.randint(lb, ub)
            
        next_layer = []
        for n in current_layer:
            children = layer_prototype.pull(size)
            for c in children:
                next_layer.append(os.path.join(n, c))
        current_layer = next_layer

    for edge_prototype, size in edge_pairs:
        if size is None:
            size = random.randint(lb, ub)
                    
        for n in current_layer:
            os.makedirs(n)
            children = edge_prototype.pull(size)
            for c in children:
                with open(os.path.join(n, c), 'w') as f:
                    f.write(str(random.choice(range(0,100))))
                    # note this won't pass stat for complex file types


"""
Layer Prototype generator abstract class
"""
class Prototype:
    """
    Produce n valid names for our prototype
    """
    def pull(n):
        pass

def make_prototype(pull, name = "a_prototype"):
    return type(name, (object,), {"pull": pull})

def make_singular_prototype(s, name = "a_prototype"):
    def pull(n):
        k = n
        out = set()
        while len(out) < n:
            out.update(s() for i in range(k))
            k *= 2

        if len(out) == n:
            return tuple(i for i in out)
        else:
            return tuple(out.pop() for i in range(n))
    return make_prototype(pull, name = name)

# make a prototype object for SmallNumDir == Directory<r"[123]">
def make_simple_prototype():
    def pull(n):
        return tuple(f"{i:03}" for i in range(n))
    return make_prototype(pull)

def layerize(prototype, layer_name: str):
    def pull(n):
        return tuple(f"{layer_name}-{i}" for i in prototype.pull(n))
    return make_prototype(pull)

def make_rand_prototype():
    return make_singular_prototype(lambda: hashlib.sha1(random.randbytes(4)).hexdigest()[:10])
    
# A: Directory
# B: Directory
# F: File
# test flatten
def gen_test_1(root):
    r = make_rand_prototype()
    layers = [(layerize(r, l), None) for l in ("A", "B", "C")]
    edges = [(layerize(r, "F"), None)]
    gen_nice_test(f"{root}/test_1", layers, edges)
    

# A: SmallNumDir == Directory<r"A-00[123]">
# B: Directory
# F: File
# Test matching prototype correctness
def gen_test_2(root):
    r = make_rand_prototype()
    n = make_simple_prototype()
    layers = [(n, 3), (r, 2), (r, 2)]
    layers = [(layerize(p, l), s) for ((p, s), l)  in zip(layers, ("A", "B", "C"))]
    edges = [(layerize(r, "F"), None)]
    gen_nice_test(f"{root}/test_2", layers, edges)

# A: ANumDir == Directory<r"A-\d{3}">
# B: BNumDir == Directory<r"B-\d{3}">
# C: CNumDir == Directory<r"C-\d{3}">
# F: FNumFile == File<r"C-\d{3}">
# Test reverse
def gen_test_3(root):
    n = make_simple_prototype()
    r = make_rand_prototype()
    layers = [(layerize(n, l), None) for l in ("A", "B", "C")]
    edges = [(layerize(r, "F"), None)]
    gen_nice_test(f"{root}/test_3", layers, edges, ub = 4)


def main(r):
    gen_test_1(r)
    gen_test_2(r)
    gen_test_3(r)

def display(r, tests):
    for d in tests:
        os.system(f"tree {r}/{d}")

def clean(r, tests):
    for d in tests:
        os.system(f"rm -r {r}/{d}")

if __name__ == "__main__":
    root = "test"
    random.seed(1)
    tests = ["test_1", "test_2", "test_3"]
    # clean(root, tests)
    main(root)
    tests = ["test_1", "test_2", "test_3"]
    display(root, tests)
    # clean(root, tests)

