use avian3d::prelude::*;
use bevy::{
    core_pipeline::experimental::taa::{TemporalAntiAliasPlugin, TemporalAntiAliasing},
    pbr::{ScreenSpaceAmbientOcclusion, ScreenSpaceAmbientOcclusionQualityLevel},
    prelude::*,
    render::camera::Exposure,
};

const PLANE_SIDE_LENGTH: f32 = 400.0;

#[derive(Component)]
struct CubeCounter(u32);

#[derive(Resource)]
struct ElaspedTime(u128);

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
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(16.0, 12.0, 24.0).looking_at(Vec3::ZERO, Vec3::Y),
        ScreenSpaceAmbientOcclusion {
            quality_level: ScreenSpaceAmbientOcclusionQualityLevel::Ultra,
            ..default()
        },
        Msaa::Off,
        Exposure::INDOOR,
        TemporalAntiAliasing::default(),
    ));

    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(PLANE_SIDE_LENGTH / 2., 0.1, PLANE_SIDE_LENGTH / 2.),
        Mesh3d(
            meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(PLANE_SIDE_LENGTH, PLANE_SIDE_LENGTH),
            ),
        ),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, -2.0, 0.0),
    ));

    commands.insert_resource(InstancingAssetHandle {
        mesh: meshes.add(Cuboid::default()),
        material: materials.add(StandardMaterial::from(Color::srgb(0.8, 0.7, 0.6))),
    });

    commands.spawn((
        Text::new("Cubes: 0"),
        TextFont {
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            font_size: 36.0,
            ..default()
        },
        TextColor::from(Color::WHITE),
        TextLayout {
            justify: JustifyText::Center,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(15.0),
            left: Val::Px(15.0),
            ..default()
        },
        CubeCounter(0),
    ));
}

fn spawn_cube(
    mut commands: Commands,
    mut cube_counter: Query<(&mut Text, &mut CubeCounter)>,
    mut elasped_time: ResMut<ElaspedTime>,
    time: Res<Time>,
    instantcing_asset_handle: Res<InstancingAssetHandle>,
) {
    elasped_time.0 += time.delta().as_micros();
    if elasped_time.0 < 100000 {
        return;
    } else {
        elasped_time.0 -= 100000;
    }
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        Mesh3d(instantcing_asset_handle.mesh.clone()),
        MeshMaterial3d(instantcing_asset_handle.material.clone()),
        Transform::from_xyz(random(4), 20.0, random(4)),
    ));

    let mut counter = cube_counter.get_single_mut().unwrap();
    counter.1 .0 += 1;
    counter.0 .0 = format!("Cubes: {}", counter.1 .0);
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            TemporalAntiAliasPlugin,
            PhysicsPlugins::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, spawn_cube)
        .insert_resource(ElaspedTime(0))
        .insert_resource(AmbientLight::default())
        .run();
}
