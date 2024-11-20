use bevy::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{collision::*, cuscuta_resources::*, player::*};

/* Set for skeleton enemy */
const SK_NAME: &str = "Skelebob";
const SK_PATH: &str = "enemies/skelly.png";
const SK_SPRITE_H: u32 = 1;
const SK_SPRITE_W: u32 = 1;
const SK_MAX_SPEED: f32 = 160.;
const SK_SPOT_DIST: f32 = 192.;
const SK_HEALTH: u32 = 2;
const SK_SIZE: u32 = 32;

/* Set for berry rat */
const BR_NAME: &str = "Berry";
const BR_PATH: &str = "enemies/berry_rat.png";
const BR_SPRITE_H: u32 = 1;
const BR_SPRITE_W: u32 = 2;
const BR_MAX_SPEED: f32 = 160.;
const BR_SPOT_DIST: f32 = 256.;
const BR_HEALTH: u32 = 2;
const BR_SIZE: u32 = 32;

/* Set for ninja */
const N_NAME: &str = "Ninja";
const N_PATH: &str = "enemies/ninja.png";
const N_SPRITE_H: u32 = 1;
const N_SPRITE_W: u32 = 1;
const N_MAX_SPEED: f32 = 160.;
const N_SPOT_DIST: f32 = 320.;
const N_HEALTH: u32 = 1;
const N_SIZE: u32 = 32;

/* Set for splat monkey */
const SP_NAME: &str = "Splatty";
const SP_PATH: &str = "enemies/splatmonkey.png";
const SP_SPRITE_H: u32 = 1;
const SP_SPRITE_W: u32 = 2;
const SP_MAX_SPEED: f32 = 100.;
const SP_SPOT_DIST: f32 = 120.;
const SP_HEALTH: u32 = 3;
const SP_SIZE: u32 = 32;

/* Set for boss */
const B_NAME: &str = "Boss";
const B_PATH: &str = "enemies/boss.png";
const B_SPRITE_H: u32 = 1;
const B_SPRITE_W: u32 = 1;
const B_MAX_SPEED: f32 = 130.;
const B_SPOT_DIST: f32 = 1000.;
const B_HEALTH: u32 = 10;
const B_SIZE: u32 = 64;

/* Cute lil enum that allows us ezpz enemy match */
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum EnemyKind {
    Skeleton(Enemy),
    BerryRat(Enemy),
    Ninja(Enemy),
    SplatMonkey(Enemy),
    Boss(Enemy),
}

/* Constuctors for enemies, using const declared above */
impl EnemyKind {
    fn skeleton() -> Self {
        EnemyKind::Skeleton(Enemy::new(
            String::from(SK_NAME),
            String::from(SK_PATH),
            SK_SPRITE_H,
            SK_SPRITE_W,
            SK_MAX_SPEED,
            SK_SPOT_DIST,
            SK_HEALTH,
            SK_SIZE,
        ))
    }
    fn berry() -> Self {
        EnemyKind::BerryRat(Enemy::new(
            String::from(BR_NAME),
            String::from(BR_PATH),
            BR_SPRITE_H,
            BR_SPRITE_W,
            BR_MAX_SPEED,
            BR_SPOT_DIST,
            BR_HEALTH,
            BR_SIZE,
        ))
    }
    fn ninja() -> Self {
        EnemyKind::Ninja(Enemy::new(
            String::from(N_NAME),
            String::from(N_PATH),
            N_SPRITE_H,
            N_SPRITE_W,
            N_MAX_SPEED,
            N_SPOT_DIST,
            N_HEALTH,
            N_SIZE,
        ))
    }
    fn splatmonkey() -> Self {
        EnemyKind::SplatMonkey(Enemy::new(
            String::from(SP_NAME),
            String::from(SP_PATH),
            SP_SPRITE_H,
            SP_SPRITE_W,
            SP_MAX_SPEED,
            SP_SPOT_DIST,
            SP_HEALTH,
            SP_SIZE,
        ))
    }
    fn boss() -> Self {
        EnemyKind::Boss(Enemy::new(
            String::from(B_NAME),
            String::from(B_PATH),
            B_SPRITE_H,
            B_SPRITE_W,
            B_MAX_SPEED,
            B_SPOT_DIST,
            B_HEALTH,
            B_SIZE,
        ))
    }
}

/* What an enemy really is */
#[derive(Component, Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Enemy {
    /* he is he */
    pub name: String,
    /* assets/ local path */
    pub filepath: String,
    /* dimensions of sprite array */
    pub sprite_row: u32,
    pub sprite_column: u32,
    /* yk. fast */
    pub max_speed: f32,
    /* how far they can see*/
    pub spot_distance: f32,
    /* health */
    pub health: u32,
    /* size */
    pub size: u32,
}

/* generic constructor for Enemy, can be used by enum
 * constructors up top */
impl Enemy {
    pub fn new(
        name: String,
        filepath: String,
        row: u32,
        column: u32,
        max_speed: f32,
        spot_distance: f32,
        health: u32,
        size: u32,
    ) -> Self {
        Self {
            name: name,
            filepath: filepath,
            sprite_row: row,
            sprite_column: column,
            max_speed: max_speed,
            spot_distance: spot_distance,
            health: health,
            size: size,
        }
    }
}

#[derive(Component, Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct EnemyMovement {
   pub direction: Vec2,
   pub axis: i32,
    pub lastseen: Vec3,
}
impl EnemyMovement {
    pub fn new(d: Vec2, a: i32, seen: Vec3) -> Self {
        Self {
            direction: d,
            axis: a,
            lastseen: seen,
        }
    }
}

/* struct to Server to Query On */
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Bundle)]
pub struct ServerEnemyBundle {
    id: EnemyId,
    motion: EnemyMovement,
    pub timer: EnemyTimer,
    transform: Transform,

}
#[derive(Component, Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct EnemyTimer {
    time: Timer,
}

/* client don't need much teebs */
#[derive(Component, Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ClientEnemy {
    pub id: EnemyId,
    pub movement: Vec<EnemyMovement>,
}

/* used by server to keep track of how many we got AND keep
 * track of individual monster types */
#[derive(Resource, Component, Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct EnemyId {
    pub id: u32,
    pub kind: EnemyKind,
}
impl EnemyId{
    pub fn get_id(&mut self) -> u32 {
        self.id
    }
}

impl EnemyId {
    pub fn new(id: u32, kind: EnemyKind) -> Self {
        Self { id: id, kind: kind }
    }
    /* returns id, increments */
    pub fn get_plus(&mut self) -> u32 {
        self.id += 1;
        self.id - 1
    }
}

/* Should soon be deprecated. Need to base
 * this off of server information...*/
pub fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut enemy_id: ResMut<EnemyId>,
) {
    let mut rng = rand::thread_rng();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x: f32 = rng.gen_range((-MAX_X + 64.)..(MAX_X - 64.));
        let random_y: f32 = rng.gen_range((-MAX_Y + 64.)..(MAX_Y - 64.));

        commands.spawn((
            // SpriteBundle {
            //     transform: Transform::from_xyz(random_x, random_y, 900.),
            //     texture: asset_server.load("enemies/skelly.png"),
            //     ..default()
            // },
            ServerEnemyBundle {
                transform: Transform::from_xyz(random_x, random_y, 900.),
                id: EnemyId::new(enemy_id.get_plus(), EnemyKind::skeleton()),
                motion: EnemyMovement::new(
                    Vec2::new(rng.gen::<f32>(), rng.gen::<f32>()).normalize(),
                    1,
                    Vec3::new(99999., 0., 0.),
                ),
                timer: EnemyTimer {
                    time: Timer::from_seconds(3.0, TimerMode::Repeating),
                },
            },
        ));
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &mut EnemyTimer, &mut EnemyMovement)>,
    mut player_query: Query<
        (&mut Transform, &Player, &mut Health),
        (With<Player>),
    >,
    wall_query: Query<(&Transform, &Wall), (Without<Player>, Without<EnemyTimer>)>,
    time: Res<Time>,
) {
    // for every enemy
    for (mut transform, mut timer, mut movement, ) in enemy_query.iter_mut() {
        // checking which player each enemy should follow (if any are in range)
        let mut player_transform: Transform = Transform::from_xyz(0., 0., 0.); //to appease the all-knowing compiler
                                                                               // checking which player is closest
        let mut longest: f32 = 99999999999.0;
        // for every player
        for (mut pt, p, mut ph) in player_query.iter_mut() {
            // find hypotenuse to get distance to player
            let xdis = (pt.translation.x - transform.translation.x).abs()
                * (pt.translation.x - transform.translation.x).abs();
            let ydis = (pt.translation.y - transform.translation.y).abs()
                * (pt.translation.y - transform.translation.y).abs();
            if ydis + xdis < ENEMY_SPOT_DISTANCE * ENEMY_SPOT_DISTANCE {
                let mut blocked = false;
                //line of sight
                for a in 0..20 {
                    //linear interpolation using mini hitboxes along line
                    let dec = (a as f32) / 20.;
                    let xnew = transform.translation.x
                        + dec * (pt.translation.x - transform.translation.x);
                    let ynew = transform.translation.y
                        + dec * (pt.translation.y - transform.translation.y);
                    let pointaabb = Aabb::new(Vec3::new(xnew, ynew, 0.), Vec2::splat(1.));
                    for (wt, w) in wall_query.iter() {
                        //checking if any line hitbox collides with any wall
                        //if wt.translation.z == pt.translation.z || wt.translation.z == pt.translation.z - 0.1 {
                        let wallaabb = Aabb::new(wt.translation, Vec2::splat(TILE_SIZE as f32));
                        if pointaabb.losintersect(&wallaabb) {
                            blocked = true;
                        }
                        //}
                    }
                }
                if blocked == true {
                    continue;
                }

                // making sure enemy chases closest enemy
                if ydis + xdis < longest {
                    longest = ydis + xdis;
                    player_transform = *pt;
                }
            }

            // handling if enemy has hit player
            let enemy_aabb = Aabb::new(transform.translation, Vec2::splat(TILE_SIZE as f32));
            let player_aabb = Aabb::new(pt.translation, Vec2::splat(TILE_SIZE as f32));
            if enemy_aabb.intersects(&player_aabb) {
                ph.current -= 5.;

                // knockback applied to player
                let direction_to_player = player_transform.translation - transform.translation;
                let normalized_direction = direction_to_player.normalize();
                //let opp_direction = Vec3::new(normalized_direction.x * -1., normalized_direction.y * -1., normalized_direction.z);
                pt.translation.x += normalized_direction.x * 64.;
                pt.translation.y += normalized_direction.y * 64.;
                player_transform.translation = pt.translation;
            }
        }
        timer.time.tick(time.delta());
        // if none in range, patrol and move to next enemy
        if longest == 99999999999.0 {
            // change direction every so often
            if timer.time.finished() {
                movement.axis = movement.axis * -1;
            }

            let normalized_direction: Vec3;
            //before patrol, try to go to last seen if have one
            if movement.lastseen.x != 99999. {
                let direction_to_player = movement.lastseen - transform.translation;
                normalized_direction = direction_to_player.normalize();
                // once the enemy gets close enough to position, go back to patrolling (to avoid getting stuck on a corner)
                if (movement.lastseen.x - transform.translation.x).abs() < 20.
                    || (movement.lastseen.y - transform.translation.y).abs() < 20.
                {
                    movement.lastseen.x = 99999.
                }
            } else {
                normalized_direction =
                    Vec3::new(1. * movement.axis as f32, 0. * movement.axis as f32, 0.);
            }

            //collision detection
            //let mut collide = false;
            let xtemp = transform.translation.x
                + normalized_direction.x * ENEMY_SPEED / 2. * time.delta_seconds();
            let ytemp = transform.translation.y
                + normalized_direction.y * ENEMY_SPEED / 2. * time.delta_seconds();
            let mut xmul: f32 = 1.;
            let mut ymul: f32 = 1.;
            let tempaabb = Aabb::new(Vec3::new(xtemp, ytemp, 0.), Vec2::splat(TILE_SIZE as f32));

            // wall collision handling
            for (wt, w) in wall_query.iter() {
                //if wt.translation.z == player_transform.translation.z || wt.translation.z == player_transform.translation.z - 0.1 {
                let wallaabb = Aabb::new(wt.translation, Vec2::splat(TILE_SIZE as f32));
                if tempaabb.intersects(&wallaabb) {
                    //collide = true;
                    let tempxaabb = Aabb::new(Vec3::new(xtemp + 16., ytemp, 0.), Vec2::splat(1.));
                    let tempx2aabb = Aabb::new(Vec3::new(xtemp - 16., ytemp, 0.), Vec2::splat(1.));
                    if tempxaabb.losintersect(&wallaabb) || tempx2aabb.losintersect(&wallaabb) {
                        xmul = 0.;
                    }
                    let tempyaabb = Aabb::new(Vec3::new(xtemp, ytemp + 16., 0.), Vec2::splat(1.));
                    let tempy2aabb = Aabb::new(Vec3::new(xtemp, ytemp - 16., 0.), Vec2::splat(1.));
                    if tempyaabb.losintersect(&wallaabb) || tempy2aabb.losintersect(&wallaabb) {
                        ymul = 0.;
                    }
                }
                //}
            }
            //if collide == true{continue;}

            transform.translation.x +=
                normalized_direction.x * ENEMY_SPEED / 2. * time.delta_seconds() * xmul;
            transform.translation.y +=
                normalized_direction.y * ENEMY_SPEED / 2. * time.delta_seconds() * ymul;
            continue;
        }

        // finding direction to move
        let direction_to_player = player_transform.translation - transform.translation;
        let normalized_direction = direction_to_player.normalize();

        // saving last seen position
        movement.lastseen = player_transform.translation;

        // making sure enemies do not collide with one another
        /*for (mut transform, _enemy) in enemy_query.iter_mut() {
        if othert.translation.x != transform.translation.x && othert.translation.y != transform.translation.y{
            let enemy_aabb = Aabb::new(transform.translation + normalized_direction, Vec2::splat(TILE_SIZE as f32));
            let other_aabb = Aabb::new(othert.translation, Vec2::splat(TILE_SIZE as f32));
            if enemy_aabb.intersects(&other_aabb){
                continue;
            }
        }  **/

        //wall collision detection
        //let mut collide = false;
        let xtemp =
            transform.translation.x + normalized_direction.x * ENEMY_SPEED * time.delta_seconds();
        let ytemp =
            transform.translation.y + normalized_direction.y * ENEMY_SPEED * time.delta_seconds();
        let mut xmul: f32 = 1.;
        let mut ymul: f32 = 1.;
        let tempaabb = Aabb::new(Vec3::new(xtemp, ytemp, 0.), Vec2::splat(TILE_SIZE as f32));
        //wall collision handling
        for (wt, w) in wall_query.iter() {
            //if wt.translation.z == player_transform.translation.z || wt.translation.z == player_transform.translation.z - 0.1 {
            let wallaabb = Aabb::new(wt.translation, Vec2::splat(TILE_SIZE as f32));
            if tempaabb.intersects(&wallaabb) {
                //collide = true;
                let tempxaabb = Aabb::new(Vec3::new(xtemp + 16., ytemp, 0.), Vec2::splat(1.));
                let tempx2aabb = Aabb::new(Vec3::new(xtemp - 16., ytemp, 0.), Vec2::splat(1.));
                if tempxaabb.losintersect(&wallaabb) || tempx2aabb.losintersect(&wallaabb) {
                    xmul = 0.;
                }
                let tempyaabb = Aabb::new(Vec3::new(xtemp, ytemp + 16., 0.), Vec2::splat(1.));
                let tempy2aabb = Aabb::new(Vec3::new(xtemp, ytemp - 16., 0.), Vec2::splat(1.));
                if tempyaabb.losintersect(&wallaabb) || tempy2aabb.losintersect(&wallaabb) {
                    ymul = 0.;
                }
            }
            //}
        }
        //if collide == true{continue;}

        //transform.translation += normalized_direction * ENEMY_SPEED * time.delta_seconds();
        transform.translation.x +=
            normalized_direction.x * ENEMY_SPEED * time.delta_seconds() * xmul;
        transform.translation.y +=
            normalized_direction.y * ENEMY_SPEED * time.delta_seconds() * ymul;
    }
}
