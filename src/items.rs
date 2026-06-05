use std::range::Range;

use bevy::{
    ecs::component::Component,
    prelude::IVec2,
    scene::{Scene, bsn},
};
use bevy_ascii_terminal::color;

use crate::{
    combat::Dice,
    components::{Position, Renderable},
};

#[derive(Component, Default, Clone, Debug)]
pub struct Item;

#[derive(Component, Default, Clone, Debug)]
pub struct HealEffect(Dice);

#[derive(Component, Default, Clone, Debug)]
pub struct DamageEffect(Dice);

#[derive(Component, Default, Clone, Debug)]
pub struct Drinkable;

#[derive(Component, Default, Clone, Debug)]
pub struct Castable;

pub fn minor_healing_potion_pos(p: IVec2) -> impl Scene {
    bsn! {
        #MinorHealingPotion
        Item
        HealEffect(Dice { dice: 4, faces: 3 })
        Drinkable
        Renderable { glyph: '¡', fg_color: color::css::BLUE }
        Position(p)
    }
}

pub fn scroll_of_magic_missile_pos(p: IVec2) -> impl Scene {
    bsn! {
        #ScrollOfMagicMissile
        Item
        DamageEffect(Dice { dice: 4, faces: 4})
        Renderable { glyph: ')', fg_color: color::css::ORANGE }
        Position(p)
    }
}
