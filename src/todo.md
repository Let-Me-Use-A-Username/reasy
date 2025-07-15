
# Current
### Editor - FileTree
- Implement editor settings
    - Show hidden files
        - Remove once cell from main
        - Make editor retrieve settings on startup
            - Hold settings globally in a config in a precomputed folder

- Differentiate between editor paths and user project paths
    - Editor paths will be settings, fonts, images etc
    - User project paths, will be paths that hold projects
        - Project dir
        - Assets
        - Code 
        - etc.

- Implement file moving and importing via drag and drop

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