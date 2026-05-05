use crate::data::{Command, CATEGORIES};

#[derive(PartialEq)]
pub enum Pane {
    Sidebar,
    List,
}

#[derive(PartialEq)]
pub enum Mode {
    Normal,
    Search,
    Help,
}

pub struct App {
    pub mode: Mode,
    pub active_pane: Pane,
    pub category_index: usize,
    pub command_index: usize,
    pub search_query: String,
    pub search_results: Vec<(usize, usize)>,
    pub copy_feedback_timer: u8,
    pub status_message: Option<String>,
}

impl App {
    pub fn new() -> Self {
        App {
            mode: Mode::Normal,
            active_pane: Pane::List,
            category_index: 0,
            command_index: 0,
            search_query: String::new(),
            search_results: Vec::new(),
            copy_feedback_timer: 0,
            status_message: None,
        }
    }

    pub fn visible_commands(&self) -> Vec<&'static Command> {
        if self.mode == Mode::Search && !self.search_query.is_empty() {
            self.search_results
                .iter()
                .map(|(category_id, command_id)| &CATEGORIES[*category_id].commands[*command_id])
                .collect()
        } else {
            CATEGORIES[self.category_index].commands.iter().collect()
        }
    }

    pub fn selected_command(&self) -> Option<&'static Command> {
        let commands = self.visible_commands();
        commands.get(self.command_index).copied()
    }

    pub fn total_commands() -> usize {
        CATEGORIES
            .iter()
            .map(|category| category.commands.len())
            .sum()
    }

    pub fn select_next_command(&mut self) {
        let length = self.visible_commands().len();
        if length == 0 {
            return;
        }
        self.command_index = (self.command_index + 1) % length;
    }

    pub fn select_previous_command(&mut self) {
        let length = self.visible_commands().len();
        if length == 0 {
            return;
        }
        self.command_index = (self.command_index + length - 1) % length;
    }

    pub fn select_next_category(&mut self) {
        self.category_index = (self.category_index + 1) % CATEGORIES.len();
        self.command_index = 0;
        self.exit_search();
    }

    pub fn select_previous_category(&mut self) {
        self.category_index = (self.category_index + CATEGORIES.len() - 1) % CATEGORIES.len();
        self.command_index = 0;
        self.exit_search();
    }

    pub fn enter_search(&mut self) {
        self.mode = Mode::Search;
        self.active_pane = Pane::List;
    }

    pub fn exit_search(&mut self) {
        if self.mode == Mode::Search {
            self.mode = Mode::Normal;
            self.search_query.clear();
            self.search_results.clear();
            self.command_index = 0;
        }
    }

    pub fn append_search_character(&mut self, character: char) {
        self.search_query.push(character);
        self.update_search_results();
    }

    pub fn remove_search_character(&mut self) {
        self.search_query.pop();
        self.update_search_results();
    }

    fn update_search_results(&mut self) {
        let query = self.search_query.to_lowercase();
        self.search_results = CATEGORIES
            .iter()
            .enumerate()
            .flat_map(|(category_id, category)| {
                category
                    .commands
                    .iter()
                    .enumerate()
                    .filter(|(_, command)| {
                        command.cmd.to_lowercase().contains(&query)
                            || command.desc.to_lowercase().contains(&query)
                    })
                    .map(move |(command_id, _)| (category_id, command_id))
            })
            .collect();
        self.command_index = 0;
    }

    pub fn extract_command_for_clipboard(&mut self) -> Option<String> {
        self.selected_command().map(|command| {
            let cleaned_command = command
                .cmd
                .replace(" <name>", "")
                .replace(" <branch>", "")
                .replace(" <file>", "")
                .replace(" <message>", "")
                .replace(" <remote>", "")
                .replace(" <url>", "")
                .replace(" <commit>", "")
                .replace(" <path>", "")
                .replace(" <tag>", "")
                .replace(" <n>", "")
                .replace(" <pattern>", "")
                .replace(" <editor>", "")
                .replace(" <email>", "")
                .replace(" <alias>", "")
                .replace(" <cmd>", "")
                .replace(" <directory>", "")
                .replace(" <old>", "")
                .replace(" <new>", "")
                .replace(" <date>", "")
                .replace(" <stash>", "")
                .replace(" <msg>", "")
                .replace(" <src>", "")
                .replace(" <dst>", "")
                .replace("\"<name>\"", "\"Your Name\"")
                .replace("\"<email>\"", "\"you@email.com\"")
                .replace("\"<message>\"", "\"your message\"")
                .replace("\"<msg>\"", "\"your message\"")
                .replace("<branch1>", "main")
                .replace("<branch2>", "feature")
                .replace("<commit1>", "HEAD~1")
                .replace("<commit2>", "HEAD");
            self.copy_feedback_timer = 8;
            self.status_message = Some(format!("copied: {}", command.cmd));
            cleaned_command
        })
    }

    pub fn update_timers(&mut self) {
        if self.copy_feedback_timer > 0 {
            self.copy_feedback_timer -= 1;
            if self.copy_feedback_timer == 0 {
                self.status_message = None;
            }
        }
    }
}
