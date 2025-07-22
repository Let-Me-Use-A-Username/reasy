
# [0.1.12] - 21/7/2025

### Changed
- [FileTree]
    - Now contains a *lookup table* for quicker node search.
- [FileTree-TreeNode]
    - *Node id* generation is now the hashed full path.
    - *Parent finding* is now done via full path instead of last component.

//---------------------------------------------------------------------------------------------------------------------------------//

# [0.1.12] - 21/7/2025
Minor changes in hidding nested elements logic.

### Added 
- [FlatTree-get_visible_items()] now checks if a node's parent is expanded before returning it as a visible element.
- [FlatTree] added tests in regards to Tree structure, node count, and child duplication.
### Changed
- [UIDirectory-reload()] now collapses elements whom's parent is hidden AND collapsed.
    - Addresses issue of visible elements who's parent has the hidden property set, but children don't.

//---------------------------------------------------------------------------------------------------------------------------------//

# [0.1.12] - 19/7/2025
Implemented settings inheritance from *EditorMenu -> EditorLayout*

### Added 
- Live settings for UI components.
### Changed
- [Pane's] are more self contained (added settings and reloading)

//---------------------------------------------------------------------------------------------------------------------------------//

# [0.1.11] - 18/7/2025
Rethinking relation between editor and ui components in regards to setting inheritance.

//---------------------------------------------------------------------------------------------------------------------------------//

# [0.1.11] - 17/7/2025
Skipped a couple of entries since I did not believe the project would `reach this far`.

### Added
    - Template for top bar menu.
    - Template for storing/loading editor settings.
    - Template for updating editor settings.
### Changed

### Deleted