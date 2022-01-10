use bevy::prelude::*;
use bevy_ascii_terminal::*;

use crate::{UI_SIZE, GAME_SIZE, VIEWPORT_SIZE, events::AttackEvent, render::RENDER_SYSTEM_LABEL};
use bevy_easings::Lerp;

pub struct UiPlugin;

#[derive(Component)]
pub struct UiTerminal;

#[derive(Default)]
struct PrintLog {
    log: Vec<String>,
}

impl PrintLog {
    pub fn push(&mut self, message: &str) {
        self.log.push(message.to_string());
    }
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
        .add_system(handle_attacks)
        .add_system(handle_print.after(RENDER_SYSTEM_LABEL))
        .init_resource::<PrintLog>()
        ;
    }
}

fn setup(
    mut commands: Commands,
) {
    let term_y = -(VIEWPORT_SIZE[1] as f32 / 2.0) + UI_SIZE[1] as f32 / 2.0;
    let mut term = TerminalBundle {
        transform: Transform::from_xyz(0.0, term_y, 1.0),
        ..TerminalBundle::new().with_size(UI_SIZE)
    };

    term.terminal.draw_border_single();

    commands.spawn_bundle(term).insert(UiTerminal);
}

fn handle_attacks(
    mut print_log: ResMut<PrintLog>,
    mut event_attacked: EventReader<AttackEvent>,
) {
    for ev in event_attacked.iter() {
        print_log.push(&format!("{} attacked {}", ev.attacker_name, ev.defender_name));
    }
}

fn handle_print(
    mut print_log: ResMut<PrintLog>,
    mut q_term: Query<&mut Terminal, With<UiTerminal>>,
) {
    if print_log.is_changed() {
        let len = print_log.log.len();
        if len > 6 {
            print_log.log.drain(0..len - 6);
        }
        let mut term = q_term.single_mut();

        term.clear();
        let border = BorderGlyphs {
            left: '│',
            right: '│',
            bottom: '─',
            top: '═',
            top_left: '╞',
            top_right: '╡',
            bottom_left: '└',
            bottom_right: '┘',
        };
        term.draw_border(border);
        for (i,s) in print_log.log.iter().rev().enumerate().take(6) {
            let (t, min,max) = (i as f32 / 6.0, 0.15, 1.0);
            let alpha = f32::lerp(&min, &max, &t);
            let y = term.top_index() as i32 - 1 - i as i32;
            term.put_string_color([1,y], s, Color::rgba(1.0, 1.0, 1.0, 1.0 - alpha).into(), BLACK);
        }
    }
}