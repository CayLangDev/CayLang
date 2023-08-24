from PrettyPrint import PrettyPrintTree
import random

class Tree:
    def __init__(self, value):
        self.val = value
        self.children = []

    def add_child(self, child):
        self.children.append(child)
        return child

    def add_children(self, children):
        self.children.extend(children)

print_tree = PrettyPrintTree(lambda x: x.children, lambda x: x.val)

def _make_tree(current:list[Tree], layers: list[str]):
    if len(layers) == 0: return current
    next_current = [Tree(s) for s in layers[0]]
    for t in current:
        t.add_children(next_current)
    return _make_tree(next_current, layers[1:])


def make_tree(layers: list[str]):
    r = Tree(layers[0])
    _make_tree([r], layers[1:])
    return r

tree = make_tree(["dataset",["1","2","3"]])
print_tree(tree)
print()


tree = make_tree(["dataset",["1","2","3"], ["file_a.txt", "file_b.txt", "file_c.txt"]])
print_tree(tree)
print()

random.seed(1)
randnums = sorted(random.choices(["1", "2", "3"], k = 5)) 
tree = make_tree(["dataset", randnums])
for c in tree.children:
    i = random.randint(1,3)
    c.add_children([Tree(s) for s in ["file_a.txt", "file_b.txt", "file_c.txt"][0:i]])
print_tree(tree)
print()

tree = make_tree(["dataset", [f"{num}_{file}" for num in range(1,4) for file in ["file_a.txt", "file_b.txt", "file_c.txt"]]])
print_tree(tree)
print()


tree = make_tree(["root",["2018","2019","2020"], ["Science", "Maths", "Art"]])
print_tree(tree)


import os
import random
import hashlib
import argparse

def gen_randname():
    return hashlib.sha1(os.urandom(4)).hexdigest()[:5]

def gen_tree(root, max_depth, max_branches, branch_prob, max_files):
    if max_depth == 0 or random.uniform(0,1) > branch_prob:
        num_files = random.randint(1, max_files)
        for _ in range(num_files):
            root.add_child(Tree(gen_randname() + '.txt'))
    else:
        num_folders = random.randint(1, max_branches)
        for i in range(num_folders):
            folder_name = f"{i:03d}"
            child = Tree(folder_name)
            root.add_child(child)
            gen_tree(child, max_depth - 1, max_branches, branch_prob, max_files)

def rand_node():
    return Tree(gen_randname())


def rand_dist(max, k):
    l = [abs(random.uniform(0,max)) for i in range(k)]
    print(l)
    c = max/sum(l)
    print(c)
    return [c*n for n in l]

def middle_meet(l):
    l.sort()
    b = l[len(l)//2:]
    l[len(l)//2:] = reversed(b)

def rand_dist_2(max, k):
    l = [random.randint(0,max) for i in range(k)]
    middle_meet(l)
    return l


def gen_tree_w(level, max_width):
    print(max_width, "mw")
    while level:
        width = random.randint(2, max_width)
        print(width)
        child_counts = [int(c) for c in rand_dist(width, len(level))]
        print(child_counts )
        next_level = []
        f = 0
        for i, count in enumerate(child_counts):
            if count <= 1:
                child = Tree(gen_randname() + '.txt')
                level[i].add_child(child)
            else:
                for j in range(count):
                    child = Tree(f"{f:03d}")
                    f += 1
                    level[i].add_child(child)
                    next_level.append(child)
        level = next_level

print("W")
random.seed(11)
tree = Tree("root")
gen_tree_w([tree], 8)
print_tree(tree)
print()

# tree = Tree("root")
# gen_tree(tree, 3, 4, 1, 4)
# print_tree(tree)
# tbd make this generate a nicer tree for printing (i.e. target width)

def dfs_flatten(new_root, root, trace):
    if root.children:
        for c in root.children:
            dfs_flatten(new_root, c, trace + "_" + c.val)
    else:
        new_root.add_child(Tree(trace + "_" + root.val))

new_tree = Tree("root")
dfs_flatten(new_tree, tree, "root")
print_tree(new_tree)

tree = make_tree(["dataset",["1","2","3"], ["A", "B", "C"], , ["A", "B", "C"]])
print_tree(tree)
print()
