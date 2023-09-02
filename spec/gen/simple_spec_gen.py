from io import TextIOBase

def specfunc(name: str):
    def wrapper(func):
        func.name = name
        return func
    return wrapper

def f_gen(spec: TextIOBase, gen_spec: TextIOBase, spec_funcs: list):
    spec_text = spec.read()
    for spec_func in spec_funcs:
        spec_text = spec_text.replace("{{" + spec_func.name + "}}", spec_func())
    gen_spec.write(spec_text)

def gen(spec_path: str, gen_spec_path: str, spec_funcs: list):
    with open(spec_path, "r") as sf, open(gen_spec_path, "w") as gsf:
        f_gen(sf, gsf, spec_funcs)

def demo(l: list):
    for f in l:
        print(f.name)
        print(f())
        print()
