import random
from example_helpers import make_tree, print_tree, gen_tree, dfs_flatten

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

random.seed(11)
tree = Tree("root")
gen_tree([tree], 8)
print_tree(tree)
print()

new_tree = Tree("root")
dfs_flatten(new_tree, tree, "root")
print_tree(new_tree)

tree = make_tree(["dataset",["1","2","3"], ["A", "B", "C"], , ["A", "B", "C"]])
print_tree(tree)
print()
