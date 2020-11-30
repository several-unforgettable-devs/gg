use crate::GameState;
use bevy::prelude::*;

pub struct GameMessagePlugin;



struct PauseMessageMarker;

struct GameMessageState {
    displayed: bool,
    pause_menu_displayed: bool,
}

impl Plugin for GameMessagePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_ui)
            .add_resource(GameMessageState { displayed: false, pause_menu_displayed: false })
            .add_system(update_ui);
    }
    // span 1 quad, re-draw very 60ms
}

fn spawn_title_message(commands: &mut Commands, 
    asset_server: &Res<AssetServer>, 
    message: String,
) {
    let position = Rect {
        top: Val::Px(5.0),
        left: Val::Px(5.0),
        right: Val::Px(5.0),
        ..Default::default()
    };
    spawn_message(commands, asset_server, message, position, false);
}

fn spawn_pause_message(commands: &mut Commands, 
    asset_server: &Res<AssetServer>, 
) {
    let position = Rect {
        top: Val::Percent(40.0),
        ..Default::default()
    };
    spawn_message(commands, 
        asset_server,
        String::from("click the screen to unpause"), 
        position,
        true);
}

fn spawn_message(commands: &mut Commands, 
    asset_server: &Res<AssetServer>, 
    message: String,
    position: Rect<Val>,
    is_pause_message:bool,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let spawner = commands.spawn(TextBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
            position_type: PositionType::Absolute,
            position: position,
            ..Default::default()
        },
        text: Text {
            value: message,
            font: font.clone(),
            style: TextStyle {
                font_size: 80.0,
                color: Color::WHITE,
                alignment: TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Top,
                },
            },
        },
        ..Default::default()
    });

    if is_pause_message {
        spawner.with(PauseMessageMarker);
    }
}

fn setup_ui(
    commands: &mut Commands
) {
    commands.spawn(UiCameraBundle::default());
}

fn update_ui(
    mut message_state: ResMut<GameMessageState>,
    game_state: Res<GameState>,
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut pause_msg_query: Query<(&PauseMessageMarker, Entity)>
) {

    if !message_state.displayed {
        if *game_state == GameState::Lost {
            // spawn "Game Over"
            spawn_title_message(
                commands,
                &asset_server,
                String::from("You have brought shame to your family..."),
            );
            message_state.displayed = true;
        } else if *game_state == GameState::Won {
            // spawn "You Win"
            spawn_title_message(commands, &asset_server, String::from("ANTICS ACHIEVED!!"));
            message_state.displayed = true;
        }
    }
    


    let add_pause_menu:bool = !message_state.pause_menu_displayed && *game_state == GameState::Paused;
    let remove_pause_menu:bool = message_state.pause_menu_displayed && *game_state != GameState::Paused;

    if add_pause_menu {
        spawn_pause_message(commands, &asset_server);
        message_state.pause_menu_displayed = true;
    }

    if remove_pause_menu {
        for (_, entity) in pause_msg_query.iter_mut() {
            commands.despawn(entity);
            message_state.pause_menu_displayed = false;
        }
    }

}
