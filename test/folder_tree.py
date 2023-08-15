import os
import random
import hashlib
import argparse

def gen_randname():
    return hashlib.sha1(os.urandom(4)).hexdigest()[:5]

def gen_tree(path, max_depth, max_branches, branch_prob, num_files):
    if max_depth == 0 or random.random() > branch_prob:
        for _ in range(num_files):
            with open(os.path.join(path, gen_randname() + '.txt'), 'w') as f:
                f.write(str(random.choice(range(0,100))))
    else:
        num_folders = random.randint(1, max_branches)
        for i in range(num_folders):
            folder_name = os.path.join(path,f"{i:03d}")
            os.makedirs(folder_name)
            gen_tree(folder_name, max_depth - 1, max_branches, branch_prob, num_files)

def main(args):
    max_depth = args.max_depth
    max_branches = args.max_branches
    branch_prob = args.branch_prob
    num_files = args.n

    root_folder = f"root-{gen_randname()}"
    os.makedirs(root_folder)
    gen_tree(root_folder, max_depth, max_branches, branch_prob, num_files)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Generate a random tree of folders and files.")
    parser.add_argument("--max-depth", type=int, required=True, help="Max depth of the tree")
    parser.add_argument("--max-branches", type=int, required=True, help="Max branches") 
    parser.add_argument("--branch-prob", type=float, required=True, help="Branching probability")
    parser.add_argument("--n", type=int, required=True, help="Number of files to make at the leaves")
    
    args = parser.parse_args()
    main(args)
