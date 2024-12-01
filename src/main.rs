use bevy::{
    core_pipeline::experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin},
    pbr::{
        ScreenSpaceAmbientOcclusionBundle, ScreenSpaceAmbientOcclusionQualityLevel,
        ScreenSpaceAmbientOcclusionSettings,
    },
    prelude::*,
    render::camera::Exposure,
    window::WindowMode,
};
use bevy_rapier3d::prelude::*;

const PLANE_SIDE_LENGTH: f32 = 400.0;

#[derive(Component)]
struct CubeCounter(u32);

#[derive(Resource)]
struct InstancingAssetHandle {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

/// Return f32 between [-n, n)
fn random(n: u32) -> f32 {
    (rand::random::<f32>() - 0.5) * n as f32
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(16.0, 12.0, 24.0).looking_at(Vec3::ZERO, Vec3::Y),
            exposure: Exposure::INDOOR,
            ..Default::default()
        })
        .insert(ScreenSpaceAmbientOcclusionBundle {
            settings: ScreenSpaceAmbientOcclusionSettings {
                quality_level: ScreenSpaceAmbientOcclusionQualityLevel::Ultra,
            },
            ..default()
        })
        .insert(TemporalAntiAliasBundle::default());

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 16.0, 4.0),
        ..default()
    });

    commands
        .spawn(Collider::cuboid(
            PLANE_SIDE_LENGTH / 2.,
            0.0,
            PLANE_SIDE_LENGTH / 2.,
        ))
        .insert(PbrBundle {
            mesh: meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(PLANE_SIDE_LENGTH, PLANE_SIDE_LENGTH),
            ),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            transform: Transform::from_xyz(0.0, -2.0, 0.0),
            ..default()
        });

    commands.insert_resource(InstancingAssetHandle {
        mesh: meshes.add(Cuboid::default()),
        material: materials.add(StandardMaterial::from(Color::srgb(0.8, 0.7, 0.6))),
    });

    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Cubes: 0",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 36.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_justify(JustifyText::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(15.0),
            left: Val::Px(15.0),
            ..default()
        }),
        CubeCounter(0),
    ));
}

fn spawn_cube(
    mut commands: Commands,
    mut cube_counter: Query<(&mut Text, &mut CubeCounter)>,
    instantcing_asset_handle: Res<InstancingAssetHandle>,
) {
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(PbrBundle {
            mesh: instantcing_asset_handle.mesh.clone(),
            material: instantcing_asset_handle.material.clone(),
            transform: Transform::from_xyz(random(4), 20.0, random(4)),
            ..default()
        });

    let mut counter = cube_counter.get_single_mut().unwrap();
    counter.1 .0 += 1;
    counter.0.sections[0].value = format!("Cubes: {}", counter.1 .0);
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            TemporalAntiAliasPlugin,
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, spawn_cube)
        .insert_resource(Time::<Fixed>::from_seconds(0.1))
        .insert_resource(AmbientLight::default())
        .run();
}
