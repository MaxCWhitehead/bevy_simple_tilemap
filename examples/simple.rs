use bevy::{
    math::{ivec3, vec2},
    prelude::*,
    window::WindowResolution,
};

use bevy_simple_tilemap::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1280.0, 720.0).with_scale_factor_override(1.0),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(SimpleTileMapPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, input_system)
        .run();
}

fn input_system(
    mut camera_transform_query: Query<&mut Transform, With<Camera2d>>,
    mut tilemap_visible_query: Query<&mut Visibility, With<TileMap>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    const MOVE_SPEED: f32 = 1000.0;
    const ZOOM_SPEED: f32 = 10.0;

    if let Some(mut tf) = camera_transform_query.iter_mut().next() {
        if keyboard_input.pressed(KeyCode::X) {
            tf.scale -= Vec3::splat(ZOOM_SPEED) * time.delta_seconds();
        } else if keyboard_input.pressed(KeyCode::Z) {
            tf.scale += Vec3::splat(ZOOM_SPEED) * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::A) {
            tf.translation.x -= MOVE_SPEED * time.delta_seconds();
        } else if keyboard_input.pressed(KeyCode::D) {
            tf.translation.x += MOVE_SPEED * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::S) {
            tf.translation.y -= MOVE_SPEED * time.delta_seconds();
        } else if keyboard_input.pressed(KeyCode::W) {
            tf.translation.y += MOVE_SPEED * time.delta_seconds();
        }

        if keyboard_input.just_pressed(KeyCode::V) {
            // Toggle visibility
            let mut visibility = tilemap_visible_query.iter_mut().next().unwrap();

            if *visibility == Visibility::Hidden {
                *visibility = Visibility::Visible;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    // Load tilesheet texture and make a texture atlas from it
    let texture_handle = asset_server.load("textures/tilesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, vec2(16.0, 16.0), 4, 1, Some(vec2(1.0, 1.0)), None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let tiles = vec![
        (
            ivec3(-1, 0, 0),
            Some(Tile {
                sprite_index: 0,
                ..Default::default()
            }),
        ),
        (
            ivec3(1, 0, 0),
            Some(Tile {
                sprite_index: 1,
                ..Default::default()
            }),
        ),
        (
            ivec3(0, -1, 0),
            Some(Tile {
                sprite_index: 2,
                ..Default::default()
            }),
        ),
        (
            ivec3(0, 1, 0),
            Some(Tile {
                sprite_index: 3,
                ..Default::default()
            }),
        ),
    ];

    let mut tilemap = TileMap::default();
    tilemap.set_tiles(tiles);

    // Set up tilemap
    let tilemap_bundle = TileMapBundle {
        tilemap,
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform {
            scale: Vec3::splat(3.0),
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    };

    // Spawn camera
    commands.spawn(Camera2dBundle::default());

    // Spawn tilemap
    commands.spawn(tilemap_bundle);
}
