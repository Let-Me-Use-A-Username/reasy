
# Current
### Editor
### Editor - FileTree
- Remove and Rename operations are not submitted to the files below.
    - *Idea*:
        - Place a *modified* bit on NodeTree entries, collect modified entries and write them via IO
        - Execute asynchronously 

<br>
<br>

# General
### Editor UI
- Adding files to project will have to be done either
    - *Top menu*
    - *Console*
    - *IDEA* Drop zone / pop up
        -https://claude.ai/chat/7bf6c484-3682-42c6-a43a-76a2c3dee785

- Implement panes (console, inspector, editor in middle for file writing/reading OR scene manager)

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