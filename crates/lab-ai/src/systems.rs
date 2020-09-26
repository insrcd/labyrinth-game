use lab_core::prelude::*;
use lab_entities::prelude::*;

/// Super Basic right now, Move all NPCs in the scene every n seconds
pub fn npc_move_system(
    mut commands: Commands,
    time: Res<Time>,
    settings: Res<WorldSettings>,
    mut query: Query<(Entity, &NonPlayer, &mut Timer, &mut Transform)>,
) {
    for (entity, _np, mut timer, trans) in &mut query.iter() {
        timer.tick(time.delta_seconds);
        if timer.finished {
            let old_translation = trans.translation();
            let direction = rand::random::<CardinalDirection>();
            let scale = trans.scale().x();
            match direction {
                CardinalDirection::West => {
                    *trans.translation().x_mut() -= settings.base_npc_speed * scale
                } // replace with npc speed
                CardinalDirection::North => {
                    *trans.translation().y_mut() += settings.base_npc_speed * scale
                }
                CardinalDirection::South => {
                    *trans.translation().y_mut() -= settings.base_npc_speed * scale
                }
                CardinalDirection::East => {
                    *trans.translation().x_mut() += settings.base_npc_speed * scale
                }
                CardinalDirection::None => {}
            }

            commands.insert(
                entity,
                (Movement {
                    start: old_translation,
                    end: trans.translation(),
                    direction: old_translation - trans.translation(),
                    legal: Some(true),
                },),
            );

            timer.reset();
        }
    }
}
