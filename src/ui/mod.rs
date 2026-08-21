pub mod action;
pub mod design;
pub mod model;
pub mod state;
pub mod tui;
pub mod views;

#[cfg(feature = "gui")]
pub mod gui;

#[cfg(feature = "gui")]
pub mod operator;

#[cfg(feature = "gui")]
pub mod projector;
