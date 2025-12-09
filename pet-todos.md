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
- [x] **c. Vim mode indicator redesign** - Friendly labels instead of cryptic vim commands

---

## Priority Order
1. ~~New window cascade~~ **DONE** - PR submitted
2. ~~Vim mode indicator~~ **DONE** - PR submitted
3. Sidebar buttons to toolbar
4. Per-pane status bar
5. Everything else

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
| 3c Vim mode indicator | Low | **DONE** - PR submitted. Friendly labels via `friendly_mode_display` setting |

## Completed Changes

### Vim Mode Indicator (3c) - PR #44470
Files changed:
- `crates/vim/src/mode_indicator.rs` - Friendly mode names, improved operator display
- `crates/vim/src/state.rs` - Added `friendly_status()` method to `Operator`
- `crates/vim/src/vim.rs` - Added `friendly_mode_display` to `VimSettings`
- `crates/settings/src/settings_content.rs` - Setting definition
- `assets/settings/default.json` - Default value (false)
- `docs/src/vim.md` - Documentation

Setting: `vim.friendly_mode_display` (default: false)
- Mode labels: "Vim", "Insert", "Replace", "Visual", etc. (instead of "NORMAL", "INSERT")
- Operator labels: "Delete...", "Change...", "Yank...", etc. (instead of "d", "c", "y")
- Count display: "3x..." or "Delete 3x..."
- Hides mode label when operator is pending
