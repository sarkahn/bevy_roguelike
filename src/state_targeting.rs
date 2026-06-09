use bevy::{math::VectorSpace, prelude::*};
use bevy_ascii_terminal::{Terminal, TerminalCamera, TerminalTransform, color};
use sark_pathfinding::taxi_dist;

use crate::{
    GameState, GameTerminal,
    components::Position,
    input,
    items::UseItem,
    iter_rect_points,
    map_state::{MapActors, PathingData},
    render::Redraw,
    state_animation::{Animation, PostAnimationState},
    visibility::MapView,
};

pub struct TargetingPlugin;

impl Plugin for TargetingPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(start_targeting).add_systems(
            Update,
            targeting_update.run_if(in_state(GameState::Targeting)),
        );
    }
}

#[derive(Component, Debug, Clone, Default)]
pub struct TargetingRange(pub i32);

#[derive(Event)]
pub struct BeginTargeting {
    pub source: Entity,
    pub range: i32,
    pub effect_haver: Entity,
}

#[derive(Component, Debug, Clone)]
pub struct Targeting {
    pub source: Entity,
    pub effect_haver: Entity,
    pub source_pos: IVec2,
    pub range: i32,
    pub points: Vec<IVec2>,
}

fn start_targeting(e: On<BeginTargeting>, q_targeter: Query<&Position>, mut commands: Commands) {
    let source_pos = q_targeter
        .get(e.source)
        .expect("Targeter has no position")
        .0;
    let r = IRect::from_center_size(source_pos, IVec2::splat(e.range * 2 + 1));
    let points: Vec<IVec2> = iter_rect_points(r)
        .filter(|p| p.manhattan_distance(source_pos) <= e.range as u32)
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
    actors: Res<MapActors>,
    term: Single<(&mut Terminal, &TerminalTransform), With<GameTerminal>>,
    cam: Single<&TerminalCamera>,
    input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    mut key_cursor_pos: Local<IVec2>,
    mut q_anim: Query<&mut Animation>,
    mut state: ResMut<State<GameState>>,
) {
    let targeting = targeting.into_inner();

    if input.any_just_pressed(crate::input::CANCEL.iter().cloned()) {
        commands.entity(targeting.source).remove::<Targeting>();
        commands.set_state(GameState::Exploring);
        commands.write_message(Redraw);
    }

    let pulse = pingpong(time.elapsed_secs(), 1.0);

    let pulse_col = LinearRgba::from_u8_array([73, 85, 89, 255]);
    let target_col = color::css::ORANGE_RED;

    let pulse_col = LinearRgba::BLACK.lerp(pulse_col, pulse);
    let target_col = LinearRgba::BLACK.lerp(target_col, pulse);

    let view = q_view
        .get(targeting.source)
        .expect("Attempting to target with a map view");

    let (mut term, transform) = term.into_inner();

    for p in &targeting.points {
        if !view.grid.get(*p) {
            continue;
        }
        if actors.0.contains_key(p) {
            term.put_bg_color(*p, target_col);
        } else if !pathing.0.is_obstacle(*p) {
            term.put_bg_color(*p, pulse_col);
        }
    }

    let in_bounds = |p: IVec2| {
        view.grid.get(p)
            && taxi_dist(p, targeting.source_pos) as i32 <= targeting.range
            && (!pathing.0.is_obstacle(p) || actors.0.contains_key(&p))
    };

    let cursor_col = LinearRgba::WHITE.lerp(LinearRgba::BLACK, pulse);
    let mut selected_pos = None;
    if let Some(mouse_pos) = cam.cursor_world_pos()
        && let Some(mouse_pos) = transform.world_to_tile(mouse_pos)
        && in_bounds(mouse_pos)
    {
        if mouse_input.just_pressed(MouseButton::Left) {
            selected_pos = Some(mouse_pos);
        } else {
            term.put_bg_color(mouse_pos, cursor_col);
        }
    } else {
        if input.any_just_pressed(input::UP.iter().cloned()) {
            key_cursor_pos.y += 1;
        }

        if input.any_just_pressed(input::DOWN.iter().cloned()) {
            key_cursor_pos.y -= 1;
        }

        if input.any_just_pressed(input::RIGHT.iter().cloned()) {
            key_cursor_pos.x += 1;
        }

        if input.any_just_pressed(input::LEFT.iter().cloned()) {
            key_cursor_pos.x -= 1;
        }

        if !in_bounds(*key_cursor_pos) {
            *key_cursor_pos = targeting.source_pos;
        }

        term.put_bg_color(*key_cursor_pos, cursor_col);
        if input.any_just_pressed(input::ACCEPT.iter().cloned()) {
            selected_pos = Some(*key_cursor_pos);
        }
    }

    if let Some(selected_pos) = selected_pos {
        if let Some(target) = actors.get(&selected_pos) {
            // // we would query for other types of effects/ground rtargeting, aoes here

            commands.entity(targeting.source).remove::<Targeting>();
            commands.write_message(Redraw);

            // TODO: Handle disabled animations feom config
            if let Ok(mut anim) = q_anim.get_mut(targeting.effect_haver) {
                match anim.as_mut() {
                    Animation::None => {
                        commands
                            .entity(targeting.effect_haver)
                            .remove::<Animation>();
                        commands.trigger(UseItem {
                            user: targeting.source,
                            item: targeting.effect_haver,
                            targets: vec![*target],
                        });
                        commands.set_state(GameState::Exploring);
                    }
                    Animation::Missile {
                        start,
                        end,
                        glyph: _,
                        fg_color: _,
                        bg_color: _,
                        time_secs: _,
                    } => {
                        *start = targeting.source_pos;
                        *end = selected_pos;
                    }
                }
                commands.set_state(GameState::Animating);
                commands
                    .entity(targeting.effect_haver)
                    .insert(PostAnimationState(GameState::Exploring));
            } else {
                commands.set_state(GameState::Exploring);
            }

            commands.trigger(UseItem {
                user: targeting.source,
                item: targeting.effect_haver,
                targets: vec![*target],
            });
            // // TODO: Should be a way to not have to do all this mess on every single state change
        }
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
