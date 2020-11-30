use bevy::prelude::*;
use crate::GameState;

pub struct GameMessagePlugin;

struct GameMessageState {
    displayed: bool
}

impl Plugin for GameMessagePlugin
{
    fn build(&self, app: &mut AppBuilder)
    {
        app.add_startup_system(setup_ui)
            .add_resource(GameMessageState{ displayed: false })
            .add_system(update_ui);
            
    }
    // span 1 quad, re-draw very 60ms
}

fn spawn_message(
    commands: &mut Commands, 
    asset_server: Res<AssetServer>,
    message: String,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn(TextBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                right: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text {
            value: message,
            font: font.clone(),
            style: TextStyle {
                font_size: 80.0,
                color: Color::WHITE,
                alignment: TextAlignment{ 
                    horizontal: HorizontalAlign::Center, 
                    vertical: VerticalAlign::Top,
                },
            },
        },
        ..Default::default()
    });
}

fn setup_ui(commands: &mut Commands) {
    commands.spawn(UiCameraBundle::default());
}

fn update_ui(
    mut message_state: ResMut<GameMessageState>,
    game_state: Res<GameState>,
    commands: &mut Commands, 
    asset_server: Res<AssetServer>,
) {
    if message_state.displayed {
        return;
    }

    if *game_state == GameState::Lost {
        // spawn "Game Over"
        spawn_message(commands, asset_server, String::from("You have brought shame to your family..."));
        message_state.displayed = true;
    } else if *game_state == GameState::Won {
        // spawn "You Win"
        spawn_message(commands, asset_server, String::from("ANTICS ACHIEVED!!"));
        message_state.displayed = true;
    }
}