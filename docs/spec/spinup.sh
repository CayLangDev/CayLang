cd gen
python gen_all.py
cd ..
mkdir ../wiki/spec > /dev/null
cp *.md -t ../wiki/spec
