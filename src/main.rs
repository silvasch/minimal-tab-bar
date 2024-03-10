use std::collections::BTreeMap;

use ansi_term::{Color, Style};
use zellij_tile::prelude::*;

#[derive(Default)]
struct Plugin {
    tabs: Vec<TabInfo>,
    mode: InputMode,
}

impl ZellijPlugin for Plugin {
    fn load(&mut self, _: BTreeMap<String, String>) {
        request_permission(&[PermissionType::ReadApplicationState]);
        subscribe(&[EventType::ModeUpdate, EventType::TabUpdate]);
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
            _ => {}
        }

        should_render
    }

    fn render(&mut self, _: usize, cols: usize) {
        let mut tab_strings = vec![];

        for tab in &self.tabs {
            let style = if tab.active {
                Style::new().bold().on(Color::Green)
            } else {
                Style::new()
            };

            tab_strings.push(style.paint(&tab.name));
        }
        let mut tabs = String::new();
        for tab_string in &tab_strings {
            tabs.push_str(&format!(" {} ", tab_string));
        }

        let mode = format!("{:?} ", self.mode).to_uppercase();

        let tabs_section_length = tabs.len();
        let mode_section_length = mode.len();

        let mut spacer = String::new();
        for _ in 0..cols - tabs_section_length - mode_section_length {
            spacer.push(' ');
        }

        print!("{}{}{}", tabs, spacer, mode);
    }
}

register_plugin!(Plugin);
