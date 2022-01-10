use bevy::prelude::*;
use bevy_ascii_terminal::*;

use crate::{
    bundle::MovingEntityBundle,
    map::Map,
    map_state::{MapActors, MapObstacles},
    monster::Monster,
    movement::{Movement, Position},
    visibility::{MapMemory, MapView, ViewRange}, events::AttackEvent, turn_system::{TakingATurn, Energy}, combat::{CombatantBundle, HitPoints, MaxHitPoints, Defense, AttackPower, TargetEvent, ActorEffect},
};

pub const PLAYER_SETUP_LABEL: &str = "PLAYER_SETUP_SYSTEM";

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PreStartup, spawn_player)
        //.add_startup_system(spawn_player.label(PLAYER_SETUP_LABEL))
        .add_system_to_stage(CoreStage::PreUpdate, player_input);
        
    }
}

fn spawn_player(
    mut commands: Commands
) {
    commands.spawn_bundle(PlayerBundle::default());
}

#[derive(Component, Default, Debug)]
pub struct Player;

#[derive(Debug, Bundle)]
pub struct PlayerBundle {
    #[bundle]
    pub move_bundle: MovingEntityBundle,
    #[bundle]
    pub combatant_bundle: CombatantBundle,
    pub player: Player,
    pub view: MapView,
    pub name: Name,
    pub memory: MapMemory,
    pub view_range: ViewRange,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            move_bundle: MovingEntityBundle::new(WHITE, '@', 12),
            combatant_bundle: CombatantBundle {
                hp: HitPoints(50),
                max_hp: MaxHitPoints(50),
                defense: Defense(2),
                attack: AttackPower(5),
            },
            player: Default::default(),
            view: Default::default(),
            name: Name::new("Player"),
            memory: Default::default(),
            view_range: ViewRange(5),
            
        }
    }
}

fn player_input(
    mut q_player: Query<(Entity, &AttackPower, &mut Position, &mut Energy, &mut Movement), (With<Player>, With<TakingATurn>)>,
    q_monsters: Query<&Name, With<Monster>>,
    input: Res<Input<KeyCode>>,
    mut obstacles: ResMut<MapObstacles>,
    mut actors: ResMut<MapActors>,
    mut event_attack: EventWriter<AttackEvent>,
    mut evt_attack: EventWriter<TargetEvent>,
) {
    if let Ok((entity, attack, mut pos, mut energy, mut movement)) = q_player.get_single_mut() {
        let input = read_movement(&input);

        if input.cmpeq(IVec2::ZERO).all() {
            return;
        }

        let curr = IVec2::from(pos.0);

        let next = curr + input;

        if obstacles.0[next] {
            if let Some(target) = actors.0[next] {
                if let Ok(name) = q_monsters.get(target) {
                    evt_attack.send( TargetEvent {
                        actor: entity,
                        target,
                        effect: ActorEffect::Damage(attack.0),
                    });

                    //println!("You bumped into {}", name.as_str());
                    // event_attack.send(AttackEvent {
                    //     attacker_name: "Player".to_string(),
                    //     defender_name: format!("the {}", name.to_string()),
                    // });
                    //println!("Player attacked a monster, ending their turn.");
                    energy.0 = 0;
                }
            }
            return;
        }

        //println!("Player moved, ending their turn");
        pos.0 = next.into();
        energy.0 = 0;
        actors.0[curr] = None;
        actors.0[next] = Some(entity);
        obstacles.0[curr] = false;
        obstacles.0[next] = true;
        movement.0 = input.into();
    }
}

fn read_movement(input: &Input<KeyCode>) -> IVec2 {
    let mut p = IVec2::ZERO;

    if input.just_pressed(KeyCode::Numpad1) || input.just_pressed(KeyCode::Z) {
        p.x = -1;
        p.y = -1;
    }
    if input.just_pressed(KeyCode::Numpad2) || input.just_pressed(KeyCode::X) || input.just_pressed(KeyCode::S) || input.just_pressed(KeyCode::Down) {
        p.y = -1;
    }
    if input.just_pressed(KeyCode::Numpad3) || input.just_pressed(KeyCode::C) {
        p.x = 1;
        p.y = -1;
    }
    if input.just_pressed(KeyCode::Numpad4) || input.just_pressed(KeyCode::A) || input.just_pressed(KeyCode::Left) {
        p.x = -1;
    }
    if input.just_pressed(KeyCode::Numpad6) || input.just_pressed(KeyCode::D) || input.just_pressed(KeyCode::Right) {
        p.x = 1;
    }
    if input.just_pressed(KeyCode::Numpad7) || input.just_pressed(KeyCode::Q) {
        p.x = -1;
        p.y = 1;
    }
    if input.just_pressed(KeyCode::Numpad8) || input.just_pressed(KeyCode::W) || input.just_pressed(KeyCode::Up) {
        p.y = 1;
    }
    if input.just_pressed(KeyCode::Numpad9) || input.just_pressed(KeyCode::E) {
        p.x = 1;
        p.y = 1;
    }
    p
}
