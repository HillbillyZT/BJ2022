use bevy::{
    prelude::*,
    time::FixedTimestep
};
use leafwing_input_manager::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

const FRAME_RATE: f32 = 60.0;
const TIME_STEP: f32 = 1.0 / FRAME_RATE;
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct Player {
    max_speed: f32,
}

#[derive(Reflect, Component, Deref, DerefMut, Default)]
#[reflect(Component)]
struct Velocity(Vec2);

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct Health {
    max_hp: f32,
    current_hp: f32,
}

#[derive(Actionlike, Clone)]
enum Action {
    Right,
    Left,
    Up,
    Down,
    Shoot,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InputManagerPlugin::<Action>::default())
        .register_type::<Player>()
        .register_type::<Health>()
        .register_type::<Velocity>()
        .add_startup_system(setup)
        .add_system(handle_input)
        .add_system_set(SystemSet::new()
            .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
            .with_system(apply_velocity)
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("icon.png"),
            ..Default::default()
        })
        .insert(Player {
            max_speed: 300.0,
        })
        .insert( Health {
            max_hp: 10.0,
            current_hp: 10.0,
        })
        .insert(Velocity(Vec2::ZERO))
        .insert_bundle(InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: InputMap::new([
                (KeyCode::D, Action::Right),
                (KeyCode::A, Action::Left),
                (KeyCode::W, Action::Up),
                (KeyCode::S, Action::Down),
                (KeyCode::Space, Action::Shoot)
            ]),
        });
}

fn handle_input(mut query: Query<(&ActionState<Action>, &mut Velocity, &Player)>) {
    let (action, mut velocity, player) = query.single_mut();
    let mut d_v = Vec2::new(0.0, 0.0);
    if action.just_pressed(Action::Shoot) {
        println!("Player just shot!");
    }
    if action.pressed(Action::Down) {
        d_v.y -= 1.0;
    }
    if action.pressed(Action::Up) {
        d_v.y += 1.0;
    }
    if action.pressed(Action::Right) {
        d_v.x += 1.0;
    }
    if action.pressed(Action::Left) {
        d_v.x -= 1.0;
    }

    d_v = d_v.normalize_or_zero();

    velocity.x = d_v.x * player.max_speed;
    velocity.y = d_v.y * player.max_speed;
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}