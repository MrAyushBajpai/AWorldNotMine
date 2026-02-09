use bevy::prelude::*;

pub mod story;
pub mod items;
pub mod ui;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, setup_text))
           .add_systems(Update, advance_text);
    }
}

fn advance_text(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Text>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for mut text in &mut query {
            *text = Text::new("A strange power lets you summon objects...");
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Text::new("You wake in a world that is not yours."),
        TextFont {
            font: asset_server.load("fonts/Merienda.ttf"),
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
    ));
}
