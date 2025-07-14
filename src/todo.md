
# Current
### Editor - FileTree
- Implement editor settings
    - Show hidden files
        - Remove once cell from main
        - Make editor retrieve settings on startup
            - Store in editor?
            - Store in user project as .reasy, similar to .vscode
                - Test it

- Differentiate between editor paths and user project paths
    - Editor settings will be saved per user project

- Implement file moving and importing via drag and drop

- #### Testing
    - Add unit tests for flat tree and display tree

<br>
<br>

# General
### Editor UI
- Implement Draggable/Droppable windows (Inspector, console etc)
    
- Implement variable mutation via struct from egui widgets

- Implement console

<br>
<br>

# Optimizations
### Editor - FileTree
    - Viewport Culling / Virtual scrolling
    - Dirty flag system
    - Implement index range not id range for quicker searching.