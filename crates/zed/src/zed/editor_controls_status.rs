use editor::Editor;
use editor::actions::{
    ToggleDiagnostics, ToggleInlineDiagnostics,
};
use gpui::{
    Action, Context, Focusable, IntoElement, ParentElement, Render, WeakEntity, Window,
};
use project::project_settings::DiagnosticSeverity;
use settings::{Settings, SettingsStore};
use ui::{
    ButtonStyle, ContextMenu, ContextMenuEntry, DocumentationEdge, DocumentationSide, IconButton,
    IconName, IconSize, PopoverMenu, PopoverMenuHandle, Tooltip, prelude::*,
};
use vim_mode_setting::{HelixModeSetting, VimModeSetting};
use workspace::{StatusItemView, Workspace, item::ItemHandle};

/// A status bar item that shows the editor controls dropdown (sliders icon).
/// This is a simplified version of the QuickActionBar's editor settings dropdown.
pub struct EditorControlsStatus {
    active_editor: Option<WeakEntity<Editor>>,
    toggle_settings_handle: PopoverMenuHandle<ContextMenu>,
}

impl EditorControlsStatus {
    pub fn new(_workspace: &Workspace, cx: &mut Context<Self>) -> Self {
        let this = Self {
            active_editor: None,
            toggle_settings_handle: Default::default(),
        };
        cx.observe_global::<SettingsStore>(|_, cx| cx.notify())
            .detach();
        this
    }
}

impl Render for EditorControlsStatus {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(editor) = self.active_editor.as_ref().and_then(|e| e.upgrade()) else {
            return div();
        };

        let editor_weak = editor.downgrade();

        let (
            supports_inlay_hints,
            inlay_hints_enabled,
            inline_values_enabled,
            supports_diagnostics,
            diagnostics_enabled,
            supports_inline_diagnostics,
            inline_diagnostics_enabled,
            git_blame_inline_enabled,
            show_git_blame_gutter,
            auto_signature_help_enabled,
            show_line_numbers,
            has_edit_prediction_provider,
            show_edit_predictions,
            edit_predictions_enabled_at_cursor,
            supports_minimap,
            minimap_enabled,
            selection_menu_enabled,
            editor_focus_handle,
        ) = editor.update(cx, |editor, cx| {
            (
                editor.supports_inlay_hints(cx),
                editor.inlay_hints_enabled(),
                editor.inline_values_enabled(),
                editor.mode().is_full(),
                editor.diagnostics_max_severity != DiagnosticSeverity::Off,
                editor.inline_diagnostics_enabled(),
                editor.show_inline_diagnostics(),
                editor.git_blame_inline_enabled(),
                editor.show_git_blame_gutter(),
                editor.auto_signature_help_enabled(cx),
                editor.line_numbers_enabled(cx),
                editor.edit_prediction_provider().is_some(),
                editor.edit_predictions_enabled(),
                editor.edit_predictions_enabled_at_cursor(cx),
                editor.supports_minimap(cx),
                editor.supports_minimap(cx) && editor.minimap().is_some(),
                editor.selection_menu_enabled(cx),
                editor.focus_handle(cx).clone(),
            )
        });

        let vim_mode_enabled = VimModeSetting::get_global(cx).0;
        let helix_mode_enabled = HelixModeSetting::get_global(cx).0;

        div().child(
            PopoverMenu::new("editor-controls-status")
                .trigger_with_tooltip(
                    IconButton::new("toggle_editor_controls_status_icon", IconName::Sliders)
                        .icon_size(IconSize::Small)
                        .style(ButtonStyle::Subtle)
                        .toggle_state(self.toggle_settings_handle.is_deployed()),
                    Tooltip::text("Editor Controls"),
                )
                .anchor(gpui::Corner::BottomRight)
                .with_handle(self.toggle_settings_handle.clone())
                .menu(move |window, cx| {
                    let menu = ContextMenu::build(window, cx, {
                        let focus_handle = editor_focus_handle.clone();
                        let editor = editor_weak.clone();
                        move |mut menu, _, _| {
                            menu = menu.context(focus_handle);

                            if supports_inlay_hints {
                                menu = menu.toggleable_entry(
                                    "Inlay Hints",
                                    inlay_hints_enabled,
                                    IconPosition::Start,
                                    Some(editor::actions::ToggleInlayHints.boxed_clone()),
                                    {
                                        let editor = editor.clone();
                                        move |window, cx| {
                                            editor
                                                .update(cx, |editor, cx| {
                                                    editor.toggle_inlay_hints(
                                                        &editor::actions::ToggleInlayHints,
                                                        window,
                                                        cx,
                                                    );
                                                })
                                                .ok();
                                        }
                                    },
                                );

                                menu = menu.toggleable_entry(
                                    "Inline Values",
                                    inline_values_enabled,
                                    IconPosition::Start,
                                    Some(editor::actions::ToggleInlineValues.boxed_clone()),
                                    {
                                        let editor = editor.clone();
                                        move |window, cx| {
                                            editor
                                                .update(cx, |editor, cx| {
                                                    editor.toggle_inline_values(
                                                        &editor::actions::ToggleInlineValues,
                                                        window,
                                                        cx,
                                                    );
                                                })
                                                .ok();
                                        }
                                    }
                                );
                            }

                            if supports_minimap {
                                menu = menu.toggleable_entry(
                                    "Minimap",
                                    minimap_enabled,
                                    IconPosition::Start,
                                    Some(editor::actions::ToggleMinimap.boxed_clone()),
                                    {
                                        let editor = editor.clone();
                                        move |window, cx| {
                                            editor
                                                .update(cx, |editor, cx| {
                                                    editor.toggle_minimap(
                                                        &editor::actions::ToggleMinimap,
                                                        window,
                                                        cx,
                                                    );
                                                })
                                                .ok();
                                        }
                                    },
                                );
                            }

                            if has_edit_prediction_provider {
                                let mut edit_prediction_entry = ContextMenuEntry::new("Edit Predictions")
                                    .toggleable(IconPosition::Start, edit_predictions_enabled_at_cursor && show_edit_predictions)
                                    .disabled(!edit_predictions_enabled_at_cursor)
                                    .action(editor::actions::ToggleEditPrediction.boxed_clone())
                                    .handler({
                                        let editor = editor.clone();
                                        move |window, cx| {
                                            editor
                                                .update(cx, |editor, cx| {
                                                    editor.toggle_edit_predictions(
                                                        &editor::actions::ToggleEditPrediction,
                                                        window,
                                                        cx,
                                                    );
                                                })
                                                .ok();
                                        }
                                    });
                                if !edit_predictions_enabled_at_cursor {
                                    edit_prediction_entry = edit_prediction_entry.documentation_aside(
                                        DocumentationSide::Left,
                                        DocumentationEdge::Top,
                                        |_| Label::new("You can't toggle edit predictions for this file as it is within the excluded files list.").into_any_element()
                                    );
                                }
                                menu = menu.item(edit_prediction_entry);
                            }

                            menu = menu.separator();

                            if supports_diagnostics {
                                menu = menu.toggleable_entry(
                                    "Diagnostics",
                                    diagnostics_enabled,
                                    IconPosition::Start,
                                    Some(ToggleDiagnostics.boxed_clone()),
                                    {
                                        let editor = editor.clone();
                                        move |window, cx| {
                                            editor
                                                .update(cx, |editor, cx| {
                                                    editor.toggle_diagnostics(&ToggleDiagnostics, window, cx);
                                                })
                                                .ok();
                                        }
                                    },
                                );

                                if supports_inline_diagnostics {
                                    let mut inline_diagnostics_item = ContextMenuEntry::new("Inline Diagnostics")
                                        .toggleable(IconPosition::Start, diagnostics_enabled && inline_diagnostics_enabled)
                                        .action(ToggleInlineDiagnostics.boxed_clone())
                                        .handler({
                                            let editor = editor.clone();
                                            move |window, cx| {
                                                editor
                                                    .update(cx, |editor, cx| {
                                                        editor.toggle_inline_diagnostics(&ToggleInlineDiagnostics, window, cx);
                                                    })
                                                    .ok();
                                            }
                                        });
                                    if !diagnostics_enabled {
                                        inline_diagnostics_item = inline_diagnostics_item
                                            .disabled(true)
                                            .documentation_aside(
                                                DocumentationSide::Left,
                                                DocumentationEdge::Top,
                                                |_| Label::new("Inline diagnostics are not available until regular diagnostics are enabled.").into_any_element()
                                            );
                                    }
                                    menu = menu.item(inline_diagnostics_item);
                                }

                                menu = menu.separator();
                            }

                            menu = menu.toggleable_entry(
                                "Line Numbers",
                                show_line_numbers,
                                IconPosition::Start,
                                Some(editor::actions::ToggleLineNumbers.boxed_clone()),
                                {
                                    let editor = editor.clone();
                                    move |window, cx| {
                                        editor
                                            .update(cx, |editor, cx| {
                                                editor.toggle_line_numbers(&editor::actions::ToggleLineNumbers, window, cx);
                                            })
                                            .ok();
                                    }
                                },
                            );

                            menu = menu.toggleable_entry(
                                "Selection Menu",
                                selection_menu_enabled,
                                IconPosition::Start,
                                Some(editor::actions::ToggleSelectionMenu.boxed_clone()),
                                {
                                    let editor = editor.clone();
                                    move |window, cx| {
                                        editor
                                            .update(cx, |editor, cx| {
                                                editor.toggle_selection_menu(&editor::actions::ToggleSelectionMenu, window, cx);
                                            })
                                            .ok();
                                    }
                                },
                            );

                            menu = menu.toggleable_entry(
                                "Auto Signature Help",
                                auto_signature_help_enabled,
                                IconPosition::Start,
                                Some(editor::actions::ToggleAutoSignatureHelp.boxed_clone()),
                                {
                                    let editor = editor.clone();
                                    move |window, cx| {
                                        editor
                                            .update(cx, |editor, cx| {
                                                editor.toggle_auto_signature_help_menu(&editor::actions::ToggleAutoSignatureHelp, window, cx);
                                            })
                                            .ok();
                                    }
                                },
                            );

                            menu = menu.separator();

                            menu = menu.toggleable_entry(
                                "Inline Git Blame",
                                git_blame_inline_enabled,
                                IconPosition::Start,
                                Some(editor::actions::ToggleGitBlameInline.boxed_clone()),
                                {
                                    let editor = editor.clone();
                                    move |window, cx| {
                                        editor
                                            .update(cx, |editor, cx| {
                                                editor.toggle_git_blame_inline(&editor::actions::ToggleGitBlameInline, window, cx);
                                            })
                                            .ok();
                                    }
                                },
                            );

                            menu = menu.toggleable_entry(
                                "Column Git Blame",
                                show_git_blame_gutter,
                                IconPosition::Start,
                                Some(git::Blame.boxed_clone()),
                                {
                                    let editor = editor.clone();
                                    move |window, cx| {
                                        editor
                                            .update(cx, |editor, cx| {
                                                editor.toggle_git_blame(&git::Blame, window, cx);
                                            })
                                            .ok();
                                    }
                                },
                            );

                            menu = menu.separator();

                            menu = menu.toggleable_entry(
                                "Vim Mode",
                                vim_mode_enabled,
                                IconPosition::Start,
                                None,
                                {
                                    move |window, cx| {
                                        let new_value = !vim_mode_enabled;
                                        VimModeSetting::override_global(VimModeSetting(new_value), cx);
                                        HelixModeSetting::override_global(HelixModeSetting(false), cx);
                                        window.refresh();
                                    }
                                },
                            );

                            menu = menu.toggleable_entry(
                                "Helix Mode",
                                helix_mode_enabled,
                                IconPosition::Start,
                                None,
                                {
                                    move |window, cx| {
                                        let new_value = !helix_mode_enabled;
                                        HelixModeSetting::override_global(HelixModeSetting(new_value), cx);
                                        VimModeSetting::override_global(VimModeSetting(false), cx);
                                        window.refresh();
                                    }
                                }
                            );

                            menu
                        }
                    });
                    Some(menu)
                })
        )
    }
}

impl StatusItemView for EditorControlsStatus {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.active_editor = active_pane_item
            .and_then(|item| item.downcast::<Editor>())
            .map(|editor| editor.downgrade());
        cx.notify();
    }
}
