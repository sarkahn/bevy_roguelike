use bevy::{math::VectorSpace, prelude::*};
use bevy_ascii_terminal::{terminal::ProgressBar, *};
use sark_pathfinding::taxi_dist;

use crate::{
    GameState, GameTerminal, UI_SIZE,
    combat::HitPoints,
    components::{BeginTargeting, LogMessage, Position, Redraw, Targeting, TargetingRange},
    iter_rect_points,
    map_state::PathingData,
    player::Player,
    visibility::MapView,
};

pub struct UiPlugin;

#[derive(Component)]
pub struct UiTerminal;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(start_targeting)
            .add_systems(
                Update,
                targeting_update.run_if(in_state(GameState::Targeting)),
            )
            .add_systems(Startup, setup)
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

fn start_targeting(e: On<BeginTargeting>, q_targeter: Query<&Position>, mut commands: Commands) {
    let source_pos = q_targeter
        .get(e.source)
        .expect("Targeter has no position")
        .0;
    let r = IRect::from_center_size(source_pos, IVec2::splat(e.range * 2 + 1));
    let points: Vec<IVec2> = iter_rect_points(r)
        .filter(|p| taxi_dist(*p, source_pos) <= e.range as usize)
        .collect();
    commands.entity(e.source).insert(Targeting {
        source: e.source,
        effect_haver: e.effect_haver,
        source_pos,
        range: e.range,
        points: points,
    });
    commands.set_state(GameState::Targeting);
    commands.write_message(Redraw);
}

fn targeting_update(
    targeting: Single<&Targeting>,
    time: Res<Time>,
    q_view: Query<&MapView>,
    pathing: Res<PathingData>,
    mut term: Single<&mut Terminal, With<GameTerminal>>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    let data = targeting.into_inner();

    if input.any_just_pressed(crate::input::CANCEL.iter().cloned()) {
        commands.entity(data.source).remove::<Targeting>();
        commands.set_state(GameState::Exploring);
        commands.write_message(Redraw);
    }

    let pulse = pingpong(time.elapsed_secs(), 1.0);

    let pulse_col = LinearRgba::from_u8_array([73, 85, 89, 255]);
    let target_col = LinearRgba::RED;

    let pulse_col = LinearRgba::BLACK.lerp(pulse_col, pulse);
    let target_col = LinearRgba::BLACK.lerp(target_col, pulse);

    let view = q_view
        .get(data.source)
        .expect("Attempting to target with a map view");

    for p in &data.points {
        if !view.get(*p) || pathing.0.is_obstacle(*p) {
            continue;
        }
        term.put_bg_color(*p, pulse_col);
    }
}

fn repeat(t: f32, len: f32) -> f32 {
    // math.clamp(t - math.floor(t / len) * len, 0, len);
    (t - (t / len).floor() * len).clamp(0., len)
}

fn pingpong(t: f32, len: f32) -> f32 {
    let t = repeat(t, len * 2.);
    len - (t - len).abs()
}
