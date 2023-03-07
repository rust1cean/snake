use crate::config::{HALF_HEIGHT, HALF_WIDTH};
use bevy::prelude::*;

#[derive(Component)]
pub struct ToTranslation;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::position_translation::<ToTranslation>);
    }
}

impl GridPlugin {
    /// Moves the reference frame from the center to the lower left corner.
    pub fn position_translation<T: Component>(
        mut cmd: Commands,
        mut entities: Query<(Entity, &mut Transform), With<T>>,
    ) {
        entities.for_each_mut(|(entity, mut transform)| {
            // Get entity scale
            let Transform { scale, .. } = *transform;

            // Update entity position
            transform.translation.x = transform.translation.x - HALF_WIDTH + (scale.x / 2.);
            transform.translation.y = transform.translation.y - HALF_HEIGHT + (scale.y / 2.);

            // Remove query marker
            cmd.entity(entity).remove::<T>();
        });
    }
}
