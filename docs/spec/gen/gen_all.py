import gen_quickstart
import gen_userstories
import gen_presentation

pairs = {"QuickStart":gen_quickstart,
         "UserStories": gen_userstories,
         "Presentation": gen_presentation}

for name, mod in pairs.items():
    mod.gen_all(f"sources/{name}_source.md", f"../{name}.md")
