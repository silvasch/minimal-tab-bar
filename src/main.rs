use std::collections::BTreeMap;

use ansi_term::{unstyled_len, ANSIString, ANSIStrings, Color, Style};
use zellij_tile::prelude::*;

#[derive(Default)]
struct Plugin {
    tabs: Vec<TabInfo>,
    pane_manifest: PaneManifest,
    mode: InputMode,
}

impl ZellijPlugin for Plugin {
    fn load(&mut self, _: BTreeMap<String, String>) {
        request_permission(&[PermissionType::ReadApplicationState]);
        subscribe(&[
            EventType::ModeUpdate,
            EventType::TabUpdate,
            EventType::PaneUpdate,
        ]);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;

        match event {
            Event::ModeUpdate(mode_info) => {
                self.mode = mode_info.mode;
                should_render = true;
            }
            Event::TabUpdate(tabs) => {
                self.tabs = tabs;
                should_render = true;
            }
            Event::PaneUpdate(pane_manifest) => {
                self.pane_manifest = pane_manifest;
                should_render = true;
            }
            _ => {}
        }

        should_render
    }

    fn render(&mut self, _: usize, cols: usize) {
        if cols == 0 {
            return;
        }

        let mut tab_strings: Vec<ANSIString> = vec![];
        let mut tab_index = None;

        for tab in &self.tabs {
            let style = if tab.active {
                tab_index = Some(tab.position);
                Style::new().bold().on(Color::Green)
            } else {
                Style::new()
            };

            tab_strings.push(style.paint(format!(" {} ", tab.name)));
        }
        let tab_string = ANSIStrings(&tab_strings);

        let active_pane_name = if let Some(tab_index) = tab_index {
            format!(
                " {}",
                match zellij_tile::shim::get_focused_pane(tab_index, &self.pane_manifest) {
                    Some(pane) => pane.title.clone(),
                    None => String::new(),
                }
            )
        } else {
            String::new()
        };

        let mode = format!("{:?} ", self.mode).to_uppercase();

        let tabs_section_length = unstyled_len(&tab_string);
        let pane_name_section_length = active_pane_name.len();
        let mode_section_length = mode.len();

        let mut spacer = String::new();
        for _ in 0..cols - tabs_section_length - pane_name_section_length - mode_section_length {
            spacer.push(' ');
        }

        print!("{}{}{}{}", tab_string, active_pane_name, spacer, mode);
    }
}

register_plugin!(Plugin);
