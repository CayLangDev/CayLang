import random
from example_helpers import make_tree, tree_to_str, gen_tree, dfs_flatten, Tree
from simple_spec_gen import specfunc, gen

# examples for QuickStart.md
@specfunc("ex1")
def ex1():
    tree = make_tree(["dataset",["1","2","3"]])
    return tree_to_str(tree)

@specfunc("ex2")
def ex2():
    tree = make_tree(["dataset",["1","2","3"], ["file_a.txt", "file_b.txt", "file_c.txt"]])
    return tree_to_str(tree)

@specfunc("ex3")
def ex3():
    random.seed(1)
    randnums = sorted(random.choices(["1", "2", "3"], k = 5))
    tree = make_tree(["dataset", randnums])
    for c in tree.children:
        i = random.randint(1,3)
        c.add_children([Tree(s) for s in ["file_a.txt", "file_b.txt", "file_c.txt"][0:i]])
    return tree_to_str(tree)

@specfunc("ex4")
def ex4():
    tree = make_tree(["dataset", [f"{num}_{file}" for num in range(1,4) for file in ["file_a.txt", "file_b.txt", "file_c.txt"]]])
    return tree_to_str(tree)

# @specfunc("ex5")
# def ex5():
#     tree = make_tree(["root",["2018","2019","2020"], ["Science", "Maths", "Art"]])
#     return tree_to_str(tree)

def unstr_tree():
    random.seed(11)
    tree = Tree("root")
    gen_tree([tree], 8)
    return tree


@specfunc("ex5.1")
def ex5_1():
    return tree_to_str(unstr_tree())

@specfunc("ex5.2")
def ex5_2():
    new_tree = Tree("root")
    dfs_flatten(new_tree, unstr_tree(), "root")
    return tree_to_str(new_tree)

def gen_all(source, pub):
    gen(source, pub, [ex1, ex2, ex3, ex4, ex5_1, ex5_2])

if __name__ == "__main__":
    gen_all("sources/QuickStart_source.md", "../QuickStart.md")
