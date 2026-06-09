use bevy::ecs::VariantDefaults;
use bevy::prelude::*;
use bevy_ascii_terminal::Terminal;

use crate::{
    GameState, GameTerminal,
    render::{Redraw, RenderSystem},
};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Last,
            animation_system
                .after(RenderSystem)
                .run_if(in_state(GameState::Animating)),
        );
    }
}

#[derive(Component, Debug, Clone, Default)]
pub struct AnimationTimer(pub f32);

#[derive(Component, Debug, Clone, VariantDefaults, Default)]
#[require(AnimationTimer)]
pub enum Animation {
    #[default]
    None,
    Missile {
        start: IVec2,
        end: IVec2,
        glyph: char,
        fg_color: Option<LinearRgba>,
        bg_color: Option<LinearRgba>,
        time_secs: f32,
    },
}

#[derive(Component, Debug, Clone, Default)]
pub struct PostAnimationState(pub GameState);

// Quick and dirty...
fn animation_system(
    mut q_anim: Query<(Entity, &Animation, &mut AnimationTimer)>,
    mut term: Single<&mut Terminal, With<GameTerminal>>,
    time: Res<Time>,
    q_post_state: Query<&PostAnimationState>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();
    for (e, anim, mut t) in q_anim.iter_mut() {
        match anim {
            Animation::None => {
                commands.entity(e).remove::<Animation>();
                commands.entity(e).remove::<AnimationTimer>();
                commands.write_message(Redraw);
                if let Ok(s) = q_post_state.get(e) {
                    commands.set_state(s.0);
                    commands.entity(e).remove::<PostAnimationState>();
                }
            }
            Animation::Missile {
                start,
                end,
                glyph,
                fg_color,
                bg_color,
                time_secs: time,
            } => {
                let start = start.as_vec2() + 0.5;
                let end = end.as_vec2() + 0.5;
                t.0 = ((t.0 + dt) / time).min(1.0);

                let p = start.lerp(end, t.0).as_ivec2();

                if let Some(t) = term.try_tile_mut(p) {
                    t.glyph = *glyph;
                    if let Some(fg) = fg_color {
                        t.fg_color = *fg;
                    }
                    if let Some(bg) = bg_color {
                        t.bg_color = *bg;
                    }
                }

                if t.0 >= 1.0 {
                    println!("ANIMATION IS COMPLETE");

                    commands.entity(e).remove::<Animation>();
                    commands.entity(e).remove::<AnimationTimer>();
                    if let Ok(s) = q_post_state.get(e) {
                        commands.set_state(s.0);
                        commands.entity(e).remove::<PostAnimationState>();
                    }
                    commands.write_message(Redraw);
                }
            }
        }
    }
}
