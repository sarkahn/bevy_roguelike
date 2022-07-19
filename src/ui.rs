use bevy::prelude::*;
use bevy_ascii_terminal::{*, ui::*};
use interpolation::Lerp;

use crate::{UI_SIZE, VIEWPORT_SIZE, events::AttackEvent, combat::{HitPoints, MaxHitPoints}, player::Player};

pub struct UiPlugin;

#[derive(Component)]
pub struct UiTerminal;

/// Log for terminal messages to be displayed to the user.
#[derive(Default)]
pub struct PrintLog {
    /// History of logged messages
    log: Vec<String>,
}

impl PrintLog {
    pub fn push(&mut self, message: String) {
        self.log.push(message);
    }
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
        .add_system(handle_attacks)
        .add_system(handle_print)
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

    term.terminal.draw_border(BorderGlyphs::single_line());

    commands.spawn_bundle(term).insert(UiTerminal);
}

fn handle_attacks(
    _print_log: ResMut<PrintLog>,
    mut event_attacked: EventReader<AttackEvent>,
) {
    for _ev in event_attacked.iter() {
        //print_log.push(format!("{} attacked {}", ev.attacker_name, ev.defender_name));
    }
}

fn handle_print(
    mut print_log: ResMut<PrintLog>,
    mut q_term: Query<&mut Terminal, With<UiTerminal>>,
    q_player: Query<(&HitPoints, &MaxHitPoints), With<Player>>,
) {
    if print_log.is_changed() {
        let len = print_log.log.len();
        if len > 6 {
            print_log.log.drain(0..len - 6);
        }
        let mut term = q_term.single_mut();

        term.clear();
        let border = BorderGlyphs::from_string(
            "╞═╡
             │ │
             └─┘"
        );
        term.draw_border(border);
        for (i,text) in print_log.log.iter().rev().enumerate().take(6) {
            let (t, min,max) = (i as f32 / 6.0, 0.15, 1.0);
            let alpha = f32::lerp(&min, &max, &t);
            let y = term.side_index(Side::Top) as i32 - 1 - i as i32;
            let fg_color = Color::rgba(1.0, 1.0, 1.0, 1.0 - alpha);
            term.put_string([1,y], text.fg(fg_color));
        }

        if let Ok((hp, max)) = q_player.get_single() {
            let hp_string = format!("HP: {} / {}", hp.0.to_string(), max.0.to_string());
            let y = term.side_index(Side::Top) as i32;
            let bar_width = term.width() as i32 - 20;
            let bar_x = term.width() as i32 - bar_width - 1;
            let hp_x = bar_x - hp_string.len() as i32 - 1;

            let fg_color = Color::YELLOW;
            term.put_string([hp_x, y], hp_string.as_str().fg(fg_color));

            let bar = UiProgressBar::new(hp.0, max.0).color_fill(
                ColorFill::EmptyOrFilled(Color::rgb(0.05, 0.05, 0.05),Color::RED));
            term.draw_progress_bar([bar_x, y], bar_width as usize, &bar)
            //term.draw_horizontal_bar_color([bar_x, y], bar_width, hp.0, max.0, Color::RED, Color::rgb(0.05, 0.05, 0.05));
        }
        

    }
}

