use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Player {
    max_speed: f32,
}

#[derive(Component)]
pub struct Entity {

}

#[derive(Actionlike, Clone)]
pub enum Action {
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
        .add_startup_system(setup)
        .add_system(handle_input)
        .run();
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("icon.png"),
            ..Default::default()
        })
        .insert(Player {
            max_speed: 5.0,
        })
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

pub fn handle_input(mut query: Query<(&ActionState<Action>, &mut Transform, &Player)>) {
    let (action, mut transform, player) = query.single_mut();
    let mut d_pos = Vec2::new(0.0, 0.0);
    if action.just_pressed(Action::Shoot) {
        println!("Player just shot!");
    }
    if action.pressed(Action::Down) {
        d_pos.y -= 1.0;
    }
    if action.pressed(Action::Up) {
        d_pos.y += 1.0;
    }
    if action.pressed(Action::Right) {
        d_pos.x += 1.0;
    }
    if action.pressed(Action::Left) {
        d_pos.x -= 1.0;
    }

    d_pos = d_pos.normalize_or_zero();

    transform.translation.x += d_pos.x * player.max_speed;
    transform.translation.y += d_pos.y * player.max_speed;
}