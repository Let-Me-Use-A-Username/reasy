
# Current
### Editor - FileTree
- Implement file moving and importing via drag and drop

- Implement console 

- Implement editor in middle for file writing/viewing(?)

- Implement Settings store/load
    - Editor paths (EDITOR_ROOT_DIR) hold information on Editor. 
        - This is where we store/load global editor settings
        - Retrieve default fonts, icons etc.

- Implement `user new project` to test file_tree, global settings, local user settings
    - User project paths, will be paths that hold data similar to .vscode


<br>
<br>

# General
### Editor UI
- Implement panes (console, inspector etc)

<br>
<br>

# Testing
- ### Editor - FileTree
    - Add unit tests for flat tree and display tree

<br>
<br>

# Optimizations
### Editor - FileTree
    - Viewport Culling / Virtual scrolling
    - Dirty flag system
    - Implement index range not id range for quicker searching.