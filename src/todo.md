
# Current
### Editor
- Implement settings of different hierarchy.
    - Editor has its own settings
        - font size, max_memory_usage etc
    - UI components (file tree, console etc) have their own settings

    -*NOTE* Settings are passed down hierachically from the editor to its components,
        some editor settings, affect lower level entities (like font-size)
        *BUT* ALL sub-component settings, affect only themselves.
            - Load settings in editor via `load_settings`
            - Pass settings onto tiles/panes
                - Either in UIDirectory i.e. or in TreeBehavior
            - When editor settings change, that also affect UI component
                - editor settings change via `menu.ui` and reloading component with new settings
            - When UI component settings change that don't affect editor
                - changes will still be made via `menu.ui` and reloading will still be needed


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