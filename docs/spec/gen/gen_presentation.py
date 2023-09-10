import random
from example_helpers import make_tree, tree_to_str, gen_tree, dfs_flatten, Tree
from simple_spec_gen import specfunc, gen, demo
from PrettyPrint import PrettyPrintTree
# examples for UserStories.md

def opendictify(opendata):
    return {"OpenSpeech": {"subset": opendata}}

# speaker, chapters
def make_opentree(opendict):
    d = {}
    for speaker, chapters in opendict.items():
        cd = {}
        for c in chapters:
            cd[c] = [f"{speaker}-{c}.flac", f"{speaker}-{c}.trans.txt"]
        d[speaker] = cd
    return {"OpenSpeech": {"subset": d}}

# speaker
def make_opentree_fl1_cat(opendict, fn=14):
    d = {}
    for speaker, chapters in opendict.items():
        cd = []
        for c in chapters:
            cd.append(f"{speaker}-{c}.flac")
            cd.append(f"{speaker}-{c}.trans.txt")
        d[speaker] = cd
    return {"OpenSpeech": {"subset": d}}

def makeup_opendata(ch_n, rd_n):
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

def openspeech_tree(maker=make_opentree):
    random.seed(1)
    d = maker(makeup_opendata(3, 3))
    return Tree.fromdict(d, "OpenSpeech")

@specfunc("openspeech")
def openspeech():
    tree = openspeech_tree()
    return tree_to_str(tree, orientation = PrettyPrintTree.HORIZONTAL)

# OpenSpeech - presentation version
# Subset
# ReaderID
# ChapterID
    # chapteraudio.flac*
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

def open_joined_spec(trace, leaves):
    speaker, chapter = trace.split("_")[2:]
    return f"{speaker}-{chapter}.flac", f"{speaker}-{chapter}.trans.txt"


@specfunc("openspeech_partflattened")
def openspeech_partflattened():
    tree = openspeech_tree(maker=make_opentree_fl1_cat)
    return tree_to_str(tree, orientation = PrettyPrintTree.HORIZONTAL)

def openspeech_map(tree: Tree):
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

def file_leaves(tree: Tree, trace = None):
    if trace == None: trace = []
    if tree.children:
        for c in tree.children:
            file_leaves(c, trace + [tree.val])
    else:
        trace_str = "-".join(sorted((trace + [tree.val])[1:], reverse=True))
        tree.add_children(Tree(s) for s in (f"{trace_str}.flac",f"{trace_str}.trans.txt"))


@specfunc("openspeech_folded_c_r")
def openspeech_folded_c_r():
    tree = openspeech_tree()
    tree = Tree.fromdict(opendictify(reverse_map(openspeech_map(tree))), "OpenSpeech")
    file_leaves(tree)
    return tree_to_str(tree)

def get_all():
    return [openspeech, openspeech_partflattened, openspeech_folded_c_r]

def gen_all(source, pub):
    gen(source, pub, get_all())

if __name__ == "__main__":
    # l = [openspeech, openspeech_partflattened, openspeech_flattened, openspeech_folded_r_c, openspeech_folded_c_r]
    # demo(l)
    demo(get_all())
