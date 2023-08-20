source $1
if [ $root_name ]
	then
		python folder_tree.py --max-depth $max_depth --max-branches $max_branches --branch-prob $branch_prob --n $n -r $root_name  
else 
		python folder_tree.py --max-depth $max_depth --max-branches $max_branches --branch-prob $branch_prob --n $n  
fi
# usage: folder_tree.py [-h] --max-depth MAX_DEPTH --max-branches MAX_BRANCHES --branch-prob BRANCH_PROB --n N
# folder_tree.py: error: the following arguments are required: --max-depth, --max-branches, --branch-prob, --n
