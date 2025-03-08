use bevy::prelude::*;
use crate::gui::RobotSprite;

#[derive(Debug, Clone, Copy)]
pub enum ResourceType {
    Energy,
    Mineral,
}

#[derive(Component)]
pub struct ResourceEntity {
    pub resource_type: ResourceType,
    pub x: f32,
    pub y: f32,
}

#[derive(Resource, Default)]
pub struct StationState {
    pub energy: u32,
    pub minerals: u32,
}

pub fn spawn_resources(mut commands: Commands) {
    let cell_size = 20.0;
    let resource_positions = vec![(4.0, 2.0), (6.0, 7.0)];

    for (x, y) in resource_positions {
        let pos_x = x * cell_size - (20.0 * cell_size) / 2.0 + cell_size / 2.0;
        let pos_y = y * cell_size - (10.0 * cell_size) / 2.0 + cell_size / 2.0;

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::ORANGE, 
                    custom_size: Some(Vec2::splat(cell_size)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(pos_x, pos_y, 0.5),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(ResourceEntity {
                resource_type: ResourceType::Energy,
                x,
                y,
            });
    }
}

pub fn check_collection(
    mut commands: Commands,
    mut station: ResMut<StationState>,
    mut robot_query: Query<(&mut RobotSprite, &mut Sprite)>,
    resource_query: Query<(Entity, &ResourceEntity)>,
) {
    for (mut robot, mut robot_sprite) in robot_query.iter_mut() {
        for (res_entity, resource) in resource_query.iter() {
            if (robot.x - resource.x).abs() < 0.01 && (robot.y - resource.y).abs() < 0.01 {
                match resource.resource_type {
                    ResourceType::Energy => {
                        station.energy += 1;
                        println!(
                            "Robot {} a collecté de l'énergie. Total = {}",
                            robot.id, station.energy
                        );
                    }
                    ResourceType::Mineral => {
                        station.minerals += 1;
                        println!(
                            "Robot {} a collecté du minerai. Total = {}",
                            robot.id, station.minerals
                        );
                    }
                }
                commands.entity(res_entity).despawn();

                robot_sprite.color = Color::RED;
            }
        }
    }
}