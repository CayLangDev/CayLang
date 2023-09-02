import random
from example_helpers import make_tree, tree_to_str, gen_tree, dfs_flatten, Tree
from simple_spec_gen import specfunc, gen
from PrettyPrint import PrettyPrintTree
# examples for UserStories.md

# @specfunc("ex1")
def ex1():
    tree = make_tree(["dataset",["1","2","3"]])
    return tree_to_str(tree)

# speaker, chapters
def make_libritree(libridict, fn=14):
    d = {}
    for speaker, chapters in libridict.items():
        cd = {}
        for c in chapters:
            cd[c] = [f"{speaker}-{c}-{i}.flac" for i in range(0,fn)]
            cd[c].append(f"{speaker}-{c}.trans.txt")
        d[speaker] = cd
    return {"Librispeech": {"subset": d}}

def makeup_libridata(ch_n, rd_n):
    d = {}
    ch_s = random.randint(0, 2**10)
    rd_s = random.randint(0, 2**10)
    chapters = list(f"CH{i}" for i in range(ch_s, ch_s+ch_n))
    readers = list(f"RD{i}" for i in range(rd_s, rd_s+rd_n))
    for rd_id in readers:
        k = random.randint(ch_n//10 + 1, ((2*ch_n)//3)+1)
        d[rd_id] = random.sample(chapters, k)
        # print(d[rd_id])
    return d

def librispeech_tree():
    random.seed(1)
    d = make_libritree(makeup_libridata(3, 3), fn=4)
    return Tree.fromdict(d, "Librispeech")

@specfunc("librispeech")
def librispeech():
    tree = librispeech_tree()
    return tree_to_str(tree, orientation = PrettyPrintTree.HORIZONTAL)

# LibriSpeech
# Subset
# ReaderID
# ChapterID
    # chapteraudioparts.flac*
    # chaptertranscript.txt

def dfs_flatten_spec(new_root, root, trace, spec):
    ls = [l for l in root.children if not l.children]
    ns = [n for n in root.children if n.children]
    if ls:
        new_kids = spec(trace, ls)
        for l in new_kids:
            new_root.add_child(Tree(l))
    for n in ns:
        dfs_flatten_spec(new_root, n, trace + "_" + n.val, spec)

def libri_joined_spec(trace, leaves):
    speaker, chapter = trace.split("_")[2:]
    return f"{speaker}-{chapter}.flac", f"{speaker}-{chapter}.trans.txt"


@specfunc("librispeech_flattened")
def librispeech_flattened():
    tree = librispeech_tree()
    new_tree = Tree("Librispeech")
    dfs_flatten_spec(new_tree, tree, "Librispeech", libri_joined_spec)
    return tree_to_str(new_tree, orientation = PrettyPrintTree.HORIZONTAL)

def librispeech_map(tree: Tree):
    readers = tree.children[0].children
    # print(readers)
    return {r.val: [c.val for c in r.children] for r in readers}

def reverse_map(d: dict):
    m = {}
    for k, vals in d.items():
        for v in vals:
            if v in m:
                m[v].append(k)
            else:
                m[v] = [k,]
    return m

@specfunc("librispeech_folded_r_c")
def librispeech_folded_r_c():
    tree = librispeech_tree()
    tree = Tree.fromdict({"root":librispeech_map(tree)}, "root")
    file_leaves(tree)
    return tree_to_str(tree)

def file_leaves(tree: Tree, trace = None):
    if trace == None: trace = []
    if tree.children:
        for c in tree.children:
            file_leaves(c, trace + [tree.val])
    else:
        trace_str = "-".join(sorted((trace + [tree.val])[1:], reverse=True))
        tree.add_children(Tree(s) for s in (f"{trace_str}.flac",f"{trace_str}.trans.txt"))


@specfunc("librispeech_folded_c_r")
def librispeech_folded_c_r():
    tree = librispeech_tree()
    tree = Tree.fromdict({"root":reverse_map(librispeech_map(tree))}, "root")
    file_leaves(tree)
    return tree_to_str(tree)

def gen_all(source, pub):
    l = [librispeech, librispeech_flattened, librispeech_folded_r_c, librispeech_folded_c_r]
    gen(source, pub, l)

if __name__ == "__main__":
    l = [librispeech, librispeech_flattened, librispeech_folded_r_c, librispeech_folded_c_r]
    # demo(l)
    gen("UserStories_source.md", "UserStories.md", l)
