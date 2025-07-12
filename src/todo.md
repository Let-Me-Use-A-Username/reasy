
# Current
### Editor - FileTree
- Implement Tree item to hold directories.
    - Implementing sorting that:
        - Splits items into structures with different depths
        - Sorts those depths by:
            - Dir first
            - then files
            - then files that start withs special characters or special endings (.gitignore, .md , etc.)
        - **IDEA** Children position also isn't correct.
            - This could be be solved with sorting (?)
            - When retrieving visible items, provide a struct that provides some feedback
                to increase the indent

- Implement file moving and importing via drag and drop

<br>
<br>

# General
### Editor UI
- Implement Draggable/Droppable windows (Inspector, console etc)
    
- Implement variable mutation via struct from egui widgets

- Implement console