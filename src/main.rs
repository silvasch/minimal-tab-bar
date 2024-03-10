use std::collections::BTreeMap;

use ansi_term::{unstyled_len, ANSIString, ANSIStrings, Color, Style};
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
        if cols == 0 {
            return;
        }

        let mut tab_strings: Vec<ANSIString> = vec![];

        for tab in &self.tabs {
            let style = if tab.active {
                Style::new().bold().on(Color::Green)
            } else {
                Style::new()
            };

            tab_strings.push(style.paint(format!(" {} ", tab.name)));
        }

        let tab_string = ANSIStrings(&tab_strings);

        let mode = format!("{:?} ", self.mode).to_uppercase();

        let mode_section_length = mode.len();
        dbg!(&mode_section_length);
        let tabs_section_length = unstyled_len(&tab_string);
        dbg!(&tabs_section_length);

        dbg!(&cols);

        let mut spacer = String::new();
        for _ in 0..cols - tabs_section_length - mode_section_length {
            spacer.push(' ');
        }

        print!("{}{}{}", tab_string, spacer, mode);
    }
}

register_plugin!(Plugin);
