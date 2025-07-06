
# Current
### Editor - FileTree
- Directories inside directories are recognized as files.
    - Due to `create_tree` function that created a wrong `UIDirectory`
- File/Directory sorting doesn't work correctly. 
    - IO provides a non sorted vector
    - `UIDirectory elemenent` value has to be some struct that can be sorted
    - And then the children of each opened directory have to be sorted seperately
        - **Ideally** create a custom tree structure that handles all the sorting(?)
- Implement file moving and importing via drag and drop

<br>
<br>

# General
### Editor UI
- Implement Draggable/Droppable windows (Inspector, console etc)
    
- Implement variable mutation via struct from egui widgets

- Implement console