# Zed Pet Todos

## 1. New Window Behavior
- [x] **a. Window cascade on new window** - PR submitted! New windows now offset 25px from active window
- [ ] **b. Finder opens file in new window** - Files opened from Finder should open in new window unless the file's path is inside an already open project
- [ ] **c. "Move to New Window" command** - Right-click menu on tab bar/project panel, command palette, menu bar

## 2. macOS Native Support
- [ ] **a. Native right-click menus** - Use NSMenu for context menus (HIGH complexity - requires GPUI platform changes)
- [ ] **b. Native project panel** - Use NSOutlineView (VERY HIGH complexity - major rewrite)
- [ ] **c. Better menu bar layout** - Reorganize menu items (LOW complexity - just editing `app_menus.rs`)

## 3. UI Changes
- [ ] **a. Sidebar controls to toolbar** - Move dock toggle buttons from status bar to title bar area
- [ ] **b. Per-pane file info bar** - Floating bar inside each editor pane instead of global status bar (language, lines, issues)
- [ ] **c. Vim mode indicator redesign** - Compact icons ("N", "I", "V") instead of "--NORMAL--" text

---

## Priority Order
1. ~~New window cascade~~ **DONE**
2. Sidebar buttons to toolbar
3. Per-pane status bar
4. Everything else

## Feasibility Notes

| Item | Complexity | Notes |
|------|------------|-------|
| 1a Window cascade | Low | **DONE** - PR submitted |
| 1b Finder new window | Low-Medium | Logic in `zed/src/main.rs:handle_open_request()` and `workspace::open_paths()` |
| 1c Move to new window | Medium | New action + register in context menus |
| 2a Native context menus | High | Requires adding NSMenu popup support to GPUI |
| 2b Native project panel | Very High | Would need NSOutlineView integration - major undertaking |
| 2c Menu bar layout | Low | Just reorganize `zed/src/zed/app_menus.rs` |
| 3a Sidebar to toolbar | Medium | Move `PanelButtons` from status bar to title bar in `workspace.rs` |
| 3b Per-pane file info | Medium-High | New component, wire up data sources per-editor |
| 3c Vim mode indicator | Low | UI changes in `vim/src/mode_indicator.rs` |
