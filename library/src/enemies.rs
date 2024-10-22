use bevy::prelude::*;
use rand::Rng;


use crate::{cuscuta_resources::*, player::*};

/* struct to query for */
#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
} 

/* Should soon be deprecated. Need to base
 * this off of server information...*/
pub fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x: f32 = rng.gen_range(-MAX_X..MAX_X);
        let random_y: f32 = rng.gen_range(-MAX_Y..MAX_Y);

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 900.),
                texture: asset_server.load("enemies/skelly.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(rng.gen::<f32>(), rng.gen::<f32>()).normalize(),
            },
        ));
    }

}
pub fn server_spawn_enemy(
    mut commands: Commands
){
    let mut rng = rand::thread_rng();
    commands.spawn((
        Enemy{
            direction: Vec2::new(rng.gen::<f32>(), rng.gen::<f32>()).normalize()
        },
    ));
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    player_query: Query<(&Transform, &NetworkId), (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
    client_id: Res<ClientId>,
) {
    for (player_transform, player_id) in player_query.iter(){
        if player_id.id == client_id.id{
            for (mut transform, _enemy) in enemy_query.iter_mut() {
                let direction_to_player = player_transform.translation - transform.translation;
                let normalized_direction = direction_to_player.normalize();
                transform.translation += normalized_direction * ENEMY_SPEED * time.delta_seconds();
            }
        }
    } 
}