use std::collections::BTreeMap;

use zellij_tile::prelude::*;

#[derive(Default)]
struct Plugin {
    tabs: Vec<TabInfo>,
    mode: InputMode,
}

register_plugin!(Plugin);

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
        let mut tabs = String::new();
        for tab in &self.tabs {
            tabs.push_str("  ");
            tabs.push_str(&tab.name);
            if tab.active {
                tabs.push('*');
            } else {
                tabs.push(' ');
            }
            tabs.push_str(" |");
        }
        if tabs.len() >= 2 {
            tabs.pop();
            tabs.pop();
            tabs.pop();
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
