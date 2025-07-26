
# Current
### Editor
### Editor - FileTree
- Write better tests for FileTree

- Add mouse input for files (left click opens in editor, right click opens menu)

<br>
<br>

# General
### Editor UI
- Adding files to project will have to be done either
    - *Top menu*
    - *Console*

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