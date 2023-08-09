use bevy::{prelude::*, window::PrimaryWindow};
use debug::DebugPlugin;
mod debug;

pub const PLAYER_SIZE: f32 = 32.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            DebugPlugin,
        )) // prevents blurry sprites
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}
#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Player {
    run_first: usize,
    run_last: usize,
    stopped_first: usize,
    stopped_last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_atlas_handle = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("sprites/gabe-idle-run-new.png"),
        Vec2::new(24.0, 24.0),
        6,
        2,
        None,
        None,
    ));
    // Use only the subset of sprites in the sheet that make up the run animation
    let player = Player {
        stopped_first: 6,
        stopped_last: 7,
        run_first: 1,
        run_last: 5,
    };
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(player.stopped_first),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        player,
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
    ));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut Transform,
        &mut TextureAtlasSprite,
        &Player,
        &mut AnimationTimer,
    )>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for (mut transform, mut sprite, player, mut timer) in &mut query {
        let mut direction = Vec3::ZERO;

        // teclas pressionadas
        for key in keyboard_input.get_pressed() {
            match key {
                // KeyCode::Up | KeyCode::W => direction.y += 1.0,
                KeyCode::Left | KeyCode::A => {
                    direction.x -= 1.0;
                    sprite.flip_x = true;
                    //animação quando o player estiver correndo
                    timer.tick(time.delta());
                    if timer.just_finished() {
                        // se o sprite atual for o ultimo sprite da animação, volta para o primeiro sprite
                        // no caso so quero a piscada que fica entre 6..7
                        sprite.index =
                            if sprite.index >= player.run_last || sprite.index < player.run_first {
                                player.run_first
                            } else {
                                sprite.index + 1
                            };
                    }
                }
                // KeyCode::Down | KeyCode::S => direction.y -= 1.0,
                KeyCode::Right | KeyCode::D => {
                    direction.x += 1.0;
                    sprite.flip_x = false;
                    //animação quando o player estiver correndo
                    timer.tick(time.delta());
                    if timer.just_finished() {
                        // se o sprite atual for o ultimo sprite da animação, volta para o primeiro sprite
                        // no caso so quero a piscada que fica entre 6..7
                        sprite.index =
                            if sprite.index >= player.run_last || sprite.index < player.run_first {
                                player.run_first
                            } else {
                                sprite.index + 1
                            };
                    }
                }
                _ => {}
            }
        }

        //animação quando o player estiver parado
        if keyboard_input.get_pressed().len() == 0 {
            timer.tick(time.delta());
            if timer.just_finished() {
                // se o sprite atual for o ultimo sprite da animação, volta para o primeiro sprite
                // no caso so quero a piscada que fica entre 6..7
                sprite.index =
                    if sprite.index >= player.stopped_last || sprite.index < player.stopped_first {
                        player.stopped_first
                    } else {
                        sprite.index + 1
                    };
            }
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * 800.0 * time.delta_seconds();

        let half_player_size = PLAYER_SIZE / 2.0;

        let x_min = -window.width() / 2.0 + half_player_size;
        let x_max = window.width() / 2.0 - half_player_size;
        let y_min = -window.height() / 2.0 + half_player_size;
        let y_max = window.height() / 2.0 - half_player_size;

        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}
