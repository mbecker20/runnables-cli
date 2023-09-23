use std::{path::PathBuf, rc::Rc, str::FromStr};

use clap::Parser;
use crossterm::event::{Event, KeyCode};
use tui_input::{backend::crossterm::EventHandler, Input};

use crate::{
    helpers::absolute_path,
    sources::{
        get_runnables,
        javascript::{JavascriptCommand, JavascriptParams},
        rust::{RustCommand, RustParams},
    },
    types::{Runnable, RunnableParams},
    CliArgs,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    List,
    Search,
}

pub struct State {
    pub args: CliArgs,
    pub runnables: Vec<Rc<Runnable>>,
    pub active: Vec<Rc<Runnable>>,
    pub selected: usize,
    pub runnable: Runnable,
    pub mode: Mode,
    pub search: Input,
}

impl State {
    pub fn new() -> anyhow::Result<State> {
        let args = CliArgs::parse();
        let mut runnables =
            get_runnables(&PathBuf::from_str(&args.path)?);
        runnables.iter_mut().enumerate().for_each(
            |(index, runnable)| {
                runnable.index = index;
            },
        );
        let mode = if args.search {
            Mode::Search
        } else {
            Mode::List
        };
        let mut state = State {
            runnables: runnables.into_iter().map(Rc::new).collect(),
            active: Default::default(),
            args,
            selected: 0,
            runnable: Default::default(),
            mode,
            search: Default::default(),
        };
        state.set_active_runnables();
        Ok(state)
    }

    pub fn set_active_runnables(&mut self) {
        self.active = self
            .runnables
            .iter()
            .cloned()
            .filter(|runnable| {
                runnable.name.contains(self.search.value())
            })
            .collect();
    }

    pub fn root_absolute_path(&self) -> anyhow::Result<String> {
        let path =
            absolute_path(&self.args.path)?.display().to_string();
        Ok(path)
    }

    // pub fn get_runnables_variants(
    //     &self,
    //     variant: RunnableParamsVariant,
    // ) -> Vec<Rc<Runnable>> {
    //     let search = self.search.value();
    //     self.runnables
    //         .iter()
    //         .cloned()
    //         .filter(|runnable| {
    //             let var: RunnableParamsVariant =
    //                 (&runnable.params).into();
    //             var == variant && runnable.name.contains(search)
    //         })
    //         .collect()
    // }

    fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    /// returns true if render loop should break
    pub fn handle_event(&mut self, event: Event) -> bool {
        match self.mode {
            Mode::List => self.handle_list_event(event),
            Mode::Search => self.handle_search_event(event),
        }
    }

    // ===================
    // LIST MODE
    // ===================

    /// returns true if should break render loop
    fn handle_list_event(&mut self, event: Event) -> bool {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char(key) => self.handle_list_keypress(key),
                KeyCode::Enter => self.handle_list_keypress('r'),
                KeyCode::Up => self.handle_list_keypress('k'),
                KeyCode::Down => self.handle_list_keypress('j'),
                KeyCode::Esc => self.handle_list_keypress('q'),
                KeyCode::Tab => {
                    self.set_mode(Mode::Search);
                    false
                }
                _ => false,
            },
            _ => false,
        }
    }

    /// returns true if should break render loop
    fn handle_list_keypress(&mut self, key: char) -> bool {
        if key == 's' {
            self.mode = Mode::Search;
            return false;
        } else if key == 'j' {
            self.select_next();
            return false;
        } else if key == 'k' {
            self.select_prev();
            return false;
        } else if key == 'q' {
            // just quit
            return true;
        }
        match self.active.get(self.selected) {
            Some(selected) => match &selected.params {
                RunnableParams::RunFile(_) => match key {
                    'r' => {
                        self.set_runnable();
                        true
                    }
                    _ => false,
                },
                RunnableParams::Shell(_) => match key {
                    'r' => {
                        self.set_runnable();
                        true
                    }
                    _ => false,
                },
                RunnableParams::Javascript(_) => {
                    let command = match key {
                        'y' | 'r' => Some(JavascriptCommand::Yarn),
                        'n' => Some(JavascriptCommand::Npm),
                        _ => None,
                    };
                    if let Some(command) = command {
                        self.set_runnable();
                        self.runnable.params =
                            RunnableParams::Javascript(
                                JavascriptParams { command },
                            );
                        true
                    } else {
                        false
                    }
                }
                RunnableParams::Rust(params) => {
                    let command = match key {
                        'r' => Some(RustCommand::Run),
                        'R' => Some(RustCommand::RunRelease),
                        't' => Some(RustCommand::Test),
                        'f' => Some(RustCommand::Fmt),
                        'c' => Some(RustCommand::Check),
                        'C' => Some(RustCommand::Clippy),
                        'b' => Some(RustCommand::Build),
                        'B' => Some(RustCommand::BuildRelease),
                        'p' => Some(RustCommand::Publish),
                        _ => None,
                    };
                    let is_lib = params.is_lib;
                    if let Some(command) = command {
                        self.set_runnable();
                        self.runnable.params =
                            RunnableParams::Rust(RustParams {
                                command,
                                is_lib,
                                args: None,
                            });
                        true
                    } else {
                        false
                    }
                }
                RunnableParams::None => false,
            },
            None => false
        }
    }

    fn select_prev(&mut self) {
        if self.active.is_empty() {
            return;
        }
        if self.selected == 0 {
            self.selected = self.active.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    fn select_next(&mut self) {
        if self.active.is_empty() {
            return;
        }
        self.selected += 1;
        self.selected %= self.active.len();
    }

    fn set_runnable(&mut self) {
        self.runnable = self.active[self.selected].as_ref().clone();
    }

    // ===================
    // SEARCH MODE
    // ===================

    fn handle_search_event(&mut self, event: Event) -> bool {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Tab | KeyCode::Enter => {
                    self.set_mode(Mode::List);
                    false
                }
                KeyCode::Esc => {
                    // self.search.with_value(value)
                    self.set_mode(Mode::List);
                    false
                }
                _ => {
                    self.selected = 0;
                    self.search.handle_event(&Event::Key(key));
                    false
                }
            },
            _ => false,
        }
    }
}
