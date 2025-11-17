//! Multi-step process management module
//!
//! This module provides utilities for managing and displaying multi-step processes
//! with progress indicators and spinners.

use indicatif::{ProgressBar, ProgressStyle};

/// State of a step in a multi-step process
#[derive(Debug, Clone, PartialEq)]
pub enum StepState {
    /// Step is pending (not started)
    Pending,
    /// Step is currently in progress (with optional spinner)
    InProgress,
    /// Step completed successfully
    Completed,
    /// Step skipped (not applicable)
    Skipped,
}

/// A single step in a multi-step process
pub struct Step {
    pub label: String,
    pub state: StepState,
    spinner: Option<ProgressBar>,
}

impl Step {
    fn new(label: String) -> Self {
        Self {
            label,
            state: StepState::Pending,
            spinner: None,
        }
    }
}

/// Manager for multi-step processes with progress indication
pub struct MultiStepProcess {
    title: String,
    steps: Vec<Step>,
    current_step: Option<usize>,
    header_printed: bool,
}

impl MultiStepProcess {
    /// Create a new multi-step process with a title
    pub fn new(title: String) -> Self {
        Self {
            title,
            steps: Vec::new(),
            current_step: None,
            header_printed: false,
        }
    }

    /// Add a step to the process
    pub fn add_step(&mut self, label: String) -> usize {
        self.steps.push(Step::new(label));
        self.steps.len() - 1
    }

    /// Start the process and print the header
    pub fn start(&mut self) {
        if !self.header_printed {
            println!("┌  {}", self.title);
            println!("│");
            self.header_printed = true;
        }
    }

    /// Start a specific step (prints its label and optionally shows a spinner)
    pub fn start_step(&mut self, step_index: usize, use_spinner: bool) {
        self.ensure_started();

        if step_index >= self.steps.len() {
            return;
        }

        // Finish any previous step
        if let Some(prev_index) = self.current_step {
            self.finish_step_internal(prev_index);
        }

        let step = &mut self.steps[step_index];
        step.state = StepState::InProgress;
        self.current_step = Some(step_index);

        if use_spinner {
            let spinner = ProgressBar::new_spinner();
            spinner.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.cyan} {msg}")
                    .unwrap()
                    .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
            );
            spinner.set_message(step.label.clone());
            spinner.enable_steady_tick(std::time::Duration::from_millis(100));
            step.spinner = Some(spinner);
        }
        // Note: For steps without spinners, we don't print here.
        // The final status will be printed when complete_step/skip_step is called.
    }

    /// Update the current step's state without finishing it
    pub fn update_step_state(&mut self, step_index: usize, state: StepState) {
        if step_index >= self.steps.len() {
            return;
        }
        self.steps[step_index].state = state;
    }

    /// Update a step's label
    pub fn update_step_label(&mut self, step_index: usize, label: String) {
        if step_index >= self.steps.len() {
            return;
        }
        self.steps[step_index].label = label;
    }

    /// Print a message for the current step (appears indented under the step)
    pub fn step_message(&self, message: &str) {
        println!("│  {}", message);
    }

    /// Start a sub-spinner for a specific operation within a step
    pub fn start_sub_spinner(&self, message: String) -> ProgressBar {
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template("│  {spinner:.cyan} {msg}")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        spinner.set_message(message);
        spinner.enable_steady_tick(std::time::Duration::from_millis(100));
        spinner
    }

    /// Complete a specific step
    pub fn complete_step(&mut self, step_index: usize) {
        if step_index >= self.steps.len() {
            return;
        }

        self.steps[step_index].state = StepState::Completed;
        self.finish_step_internal(step_index);

        if self.current_step == Some(step_index) {
            self.current_step = None;
        }
    }

    /// Skip a specific step
    pub fn skip_step(&mut self, step_index: usize) {
        if step_index >= self.steps.len() {
            return;
        }

        self.steps[step_index].state = StepState::Skipped;
        self.finish_step_internal(step_index);

        if self.current_step == Some(step_index) {
            self.current_step = None;
        }
    }

    /// Finish the entire process and print the footer
    pub fn finish(&mut self) {
        // Finish any current step
        if let Some(current) = self.current_step {
            self.finish_step_internal(current);
        }

        println!("│");
        println!("└  All done!");
    }

    /// Ensure the process has been started
    fn ensure_started(&mut self) {
        if !self.header_printed {
            self.start();
        }
    }

    /// Internal method to finish a step (clear spinner, print status)
    fn finish_step_internal(&mut self, step_index: usize) {
        if step_index >= self.steps.len() {
            return;
        }

        let step = &mut self.steps[step_index];

        // Clear any active spinner
        if let Some(spinner) = step.spinner.take() {
            spinner.finish_and_clear();
        }

        // Print the step with its final state
        match step.state {
            StepState::Completed => println!("◆  {}", step.label),
            StepState::Skipped => println!("◇  {} (skipped)", step.label),
            StepState::Pending | StepState::InProgress => println!("◇  {}", step.label),
        }
    }
}
