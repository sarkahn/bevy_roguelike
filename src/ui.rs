use bevy::prelude::*;
use bevy_ascii_terminal::{terminal::ProgressBar, *};

use crate::{UI_SIZE, combat::HitPoints, components::LogMessage, player::Player};

pub struct UiPlugin;

#[derive(Component)]
pub struct UiTerminal;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(PostUpdate, (handle_print, handle_hp_changed));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Terminal::new(UI_SIZE).with_border(BoxStyle::SINGLE_LINE),
        UiTerminal,
        TerminalMeshPivot::LeftTop,
    ));
}

fn handle_print(
    mut log: MessageReader<LogMessage>,
    mut term: Single<&mut Terminal, With<UiTerminal>>,
    mut buffer: Local<Vec<String>>,
) {
    for lm in log.read() {
        buffer.push(lm.0.clone())
    }
    if buffer.is_empty() {
        return;
    }

    term.clear_inner();

    for (y, text) in buffer.iter().rev().enumerate().take(6) {
        let t = 1.0 - (y as f32 / 6.0);

        let alpha = LinearCurve.sample_clamped(t).clamp(0.15, 1.0);

        let fg_color = LinearRgba::new(1.0, 1.0, 1.0, alpha);
        term.put_string([0, y], text.as_str().fg(fg_color));
    }
}

fn handle_hp_changed(
    mut term: Single<&mut Terminal, With<UiTerminal>>,
    hp: Option<Single<&HitPoints, (With<Player>, Changed<HitPoints>)>>,
) {
    if let Some(hp) = hp.map(|v| v.into_inner()) {
        let hp_string = format!("HP: {} / {}", hp.current, hp.max);

        term.set_padding(Padding::ZERO);

        let bar = ProgressBar::new(
            hp.current as f32 / hp.max as f32,
            term.width() - 4 - hp_string.len(),
            '▓',
            '░',
        )
        .with_fill_color(color::css::RED)
        .with_empty_color(LinearRgba::rgb(0.05, 0.05, 0.05));
        term.progress_bar([hp_string.len() as i32 + 2, 0], bar);
        term.put_string([1, 0], hp_string);
        term.set_padding(Padding::ONE);
    }
}
