
# Current
### Editor - FileTree
- Implement *Editor settings load/store from file*
    - File loading has been implemented
    - *File saving hasn't*

- Implement *Editor settings live update*
    - Saving could occur from `menu.ui` where we know which pane's settings was changed.
    - *note* if singlenton implemented, update it, then take reference
        - Then reload component with new settings
    - Review this approach: https://claude.ai/share/83da24e8-0276-498c-97d6-1975ff796388
    
    - Implement a way to share `editor_settings` among lower-level components (EditorLayout)

- Implement file moving and importing via drag and drop


<br>
<br>

# General
### Editor UI
- Implement panes (console, inspector etc)

- Implement editor in middle for file writing/reading(?)

- Implement `user new project` to test file_tree, global settings, local user settings
    - User project paths, will be paths that hold data similar to .vscode

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