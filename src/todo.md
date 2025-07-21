
# Current
### Editor
#### Editor - FileTree
-*VERIFIED ERROR* from test results
    - Some elements have wrong ids and cause parent mixup.
    - This addresses issues: 1, 2(possibly) and 3 
    - *Implement* hasher for path to usize
        - https://chatgpt.com/share/687e3371-be18-8004-8e55-e573ee262958
- *Test* if `get_parent` works with same named parents
    - If it doesn't work, change to full path.

-*Problems*
    - 1. Closing directory sometimes closes sibling as well
        - .git/objects , .git/refs
        - If refs is opened, and objects is opened, and then object closes, refs closes too
    - 2. Leaf directories are sometimes empty when they shouldn't
        - *objects* directory
    - 3. Some leaf directories have duplicated entries
        - .git/refs/heads
        - In directory two `master` items exists in editor, only single one is real.

### Editor - FileTree
- Implement file moving and importing via drag and drop


<br>
<br>

# General
### Editor UI
- Implement panes (console, inspector etc)

- Implement editor in middle for file writing/reading(?)

- Implement `user new project` to test file_tree, global settings, local user settings
    - User project paths, will be paths that hold data similar to .vscode

- When saving settings, give user the option to save as global or project settings.

<br>
<br>

# Testing

<br>
<br>

# Optimizations
### Editor - FileTree
    - Viewport Culling / Virtual scrolling
    - Dirty flag system
    - Implement index range not id range for quicker searching.