
# Current
### Editor
### Editor - FileTree
- Editor|Layout|Pane are now aware of file `dnd`
    - Implement UI to show where file will be dropped

- Write better tests for FileTree

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