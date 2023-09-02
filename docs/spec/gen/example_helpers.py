from PrettyPrint import PrettyPrintTree
import os
import random
import hashlib

class Tree:
    def __init__(self, value):
        self.val = value
        self.children = []

    def add_child(self, child):
        self.children.append(child)
        return child

    def add_children(self, children):
        self.children.extend(children)

    def fromdict(d: dict, root):
        root_node = Tree(root)
        # print(root)
        if isinstance(d[root], dict):
            children = [Tree.fromdict(d[root], c) for c in d[root].keys()]
            root_node.add_children(children)
        elif isinstance(d[root], list):
            root_node.add_children(Tree(c) for c in d[root])
        return root_node


print_tree = PrettyPrintTree(lambda x: x.children, lambda x: x.val)
tree_to_str = PrettyPrintTree(lambda x: x.children, lambda x: x.val, return_instead_of_print=True, color=None)

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

def gen_randname():
    return hashlib.sha1(random.randbytes(4)).hexdigest()[:5]

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

def gen_tree(level, max_width):
    while level:
        width = random.randint(2, max_width)
        # print(width)
        child_counts = [int(c) for c in rand_dist(width, len(level))]
        # print(child_counts )
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

def dfs_flatten(new_root, root, trace):
    if root.children:
        for c in root.children:
            dfs_flatten(new_root, c, trace + "_" + c.val)
    else:
        new_root.add_child(Tree(trace + "_" + root.val))

def fsys_tree(tree: Tree):
    d = {}
    def _dfs(root: Tree, trace):
        trace += root.val
        d[trace] = root
        for c in root.children:
            _dfs(c, f"{trace}/")
    return d



# def fold(tree: Tree, old_paths: list[str], new_paths: list[str]):
#     fd = fsys_tree(tree)
#

