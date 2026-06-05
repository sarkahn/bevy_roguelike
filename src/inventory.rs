use bevy::{
    ecs::{component::Component, entity::Entity},
    prelude::Deref,
};

#[derive(Component, Debug)]
#[relationship(relationship_target = HeldItems)]
pub struct ItemHeldBy(pub Entity);

/// Items Held by an entity. Shouldn't be modified - use [ItemHeldBy] instead
#[derive(Component, Debug, Deref)]
#[relationship_target(relationship = ItemHeldBy)]
pub struct HeldItems(Vec<Entity>);
