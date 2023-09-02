import gen_quickstart
import gen_userstories

pairs = {"QuickStart":gen_quickstart,
         "UserStories": gen_userstories}

for name, mod in pairs.items():
    mod.gen_all(f"sources/{name}_source.md", f"../{name}.md")
