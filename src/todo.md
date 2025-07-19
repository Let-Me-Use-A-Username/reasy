
# Current
### Editor
-*Note* Settings now go from 
    - EditorMenu(collect Panes to change) -> EditorLayout(Apply new changes to panes and reload them) -> Each pane reloads itself and holds its own settings, EditorLayout merely propagates them.

-*Problem* When hiding a directory, if expanded and then hidden elemenets are hidden, children of this directory
remain active.


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
- ### Editor - FileTree
    - Add unit tests for flat tree and display tree

<br>
<br>

# Optimizations
### Editor - FileTree
    - Viewport Culling / Virtual scrolling
    - Dirty flag system
    - Implement index range not id range for quicker searching.