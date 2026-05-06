use kube::Client;

use crate::model::{resource::ResourceTable, Resource};

/// Input mode for the command bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Command,
    Filter,
}

/// Top-level application state.
pub struct App {
    pub client: Client,
    pub namespace: Option<String>,
    pub active_resource: Resource,
    pub table: ResourceTable,
    pub input_mode: InputMode,
    pub command_input: String,
    pub filter_input: String,
    pub selected_row: usize,
    pub running: bool,
}

impl App {
    pub fn new(client: Client, namespace: Option<String>) -> Self {
        Self {
            client,
            namespace,
            active_resource: Resource::Pod,
            table: ResourceTable::default(),
            input_mode: InputMode::Normal,
            command_input: String::new(),
            filter_input: String::new(),
            selected_row: 0,
            running: true,
        }
    }

    /// Switch to a different resource view.
    pub fn switch_resource(&mut self, resource: Resource) {
        self.active_resource = resource;
        self.table = ResourceTable::default();
        self.selected_row = 0;
    }

    pub fn move_selection_up(&mut self) {
        self.selected_row = self.selected_row.saturating_sub(1);
    }

    pub fn move_selection_down(&mut self) {
        if !self.table.rows.is_empty() {
            self.selected_row = (self.selected_row + 1).min(self.table.rows.len() - 1);
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}