use bevy::input::keyboard::{KeyCode, KeyCode::*};

pub const RIGHT: &[KeyCode] = &[ArrowRight, KeyE, KeyD, KeyC, Numpad9, Numpad6, Numpad3];
pub const LEFT: &[KeyCode] = &[ArrowLeft, KeyA, KeyQ, KeyZ, Numpad7, Numpad4, Numpad1];
pub const UP: &[KeyCode] = &[ArrowUp, KeyQ, KeyW, KeyE, Numpad7, Numpad8, Numpad9];
pub const DOWN: &[KeyCode] = &[ArrowDown, KeyZ, KeyX, KeyS, KeyC, Numpad1, Numpad2, Numpad3];
pub const ACCEPT: &[KeyCode] = &[Enter, Space];
pub const CANCEL: &[KeyCode] = &[Escape];
