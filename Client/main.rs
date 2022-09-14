use std::str::FromStr;
use std::time::Duration;

use bevy::prelude::*;

use bevy::text::Text2dSize;
use bincode::{DefaultOptions};
use serde::{Deserialize, Serialize};

use bevy_slinet::connection::MaxPacketSize;
use bevy_slinet::client::ClientPlugin;
use bevy_slinet::packet_length_serializer::LittleEndian;
use bevy_slinet::protocols::tcp::TcpProtocol;
use bevy_slinet::serializers::bincode::BincodeSerializer;
use bevy_slinet::server::{NewConnectionEvent, ServerPlugin};
use bevy_slinet::{client, server, ClientConfig, ServerConfig};


#[derive(Component)]
struct Paddle;
#[derive(Component)]
struct Player1;
#[derive(Component)]
struct Player2;
#[derive(Component)]
struct B;


#[derive(Serialize, Deserialize, Debug, Default,Clone, Copy)]
struct Ball {
    x:f32,
    y:f32,
    velocity: Vec2,
}
#[derive(Serialize, Deserialize, Debug, Default,Clone, Copy)]
struct POne {
    x:f32,
    y:f32,
}
#[derive(Serialize, Deserialize, Debug, Default,Clone, Copy)]
struct PTwo {
    x:f32,
    y:f32,
}

#[derive(Serialize, Deserialize, Debug, Default,Copy,Clone)]
struct Score {
    p1:u32,
    p2:u32,
}

#[derive(Serialize, Deserialize, Debug, Default,Clone, Copy)]
struct GameState {
    p1:POne,
    p2:PTwo,
    ball:Ball,
    score:Score,
}
struct Config;

impl ServerConfig for Config {
    type ClientPacket = ClientPacket;
    type ServerPacket = ServerPacket;
    type Protocol = TcpProtocol;
    type Serializer = BincodeSerializer<DefaultOptions>;
    type LengthSerializer = LittleEndian<u32>;
}

impl ClientConfig for Config {
    type ClientPacket = ClientPacket;
    type ServerPacket = ServerPacket;
    type Protocol = TcpProtocol;
    type Serializer = BincodeSerializer<DefaultOptions>;
    type LengthSerializer = LittleEndian<u32>;
}
#[derive(Serialize, Deserialize,Clone, Copy, Debug)]
enum One {
    Up,
    Down,
    None,
}
#[derive(Serialize, Deserialize,Clone, Copy, Debug)]
enum Two {
    Up,
    Down,
    None,
}
#[derive(Serialize, Deserialize,Clone, Copy, Debug)]
enum Player {
    One(One),
    Two(Two),
    Spectator,
}

#[derive(Serialize, Deserialize, Debug)]
enum ClientPacket {
    Player(Player),
}

#[derive(Serialize, Deserialize, Debug)]
enum ServerPacket {
    GameState(POne,PTwo,Ball,Score),
    Clients(u32),
}

const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

fn main() {
    let client =
        App::new()
            .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .insert_resource(GameState{..Default::default()})
            .insert_resource(MaxPacketSize(100))
            .insert_resource(Player::Spectator)
            .insert_resource(WindowDescriptor{title: String::from("Pong"), ..Default::default()})
            .add_plugins(DefaultPlugins)
            .add_startup_system(start)
            .add_system(move_paddle)
            .add_system(display_ball)
            .add_system(display_player1)
            .add_system(display_player2)
            .add_plugin(ClientPlugin::<Config>::connect("2601:600:8d80:f0d0:b510:82f0:e1b2:cf1:3000"))
            .add_system(client_packet_receive_system)
            .add_system(client_connection_system)
            .run();
    
    //client.join().unwrap();
}

fn start(
    mut commands: Commands, 
) {
        commands.spawn_bundle(OrthographicCameraBundle::new_2d());
        commands.spawn_bundle(UiCameraBundle::default());

        commands
            .spawn()
            .insert(Paddle)
            .insert_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(100.0, 1.0, 0.0),
                    scale: Vec3::new(20.0, 100.0, 0.0),
                    ..default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0,1.0,1.0),
                ..default()
            },
            ..default()
        }).insert(Player1);

        commands
        .spawn()
        .insert(Paddle)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-100.0, 1.0, 0.0),
                scale: Vec3::new(20.0, 100.0, 0.0),
                ..default()
        },
        sprite: Sprite {
            color: Color::rgb(1.0,1.0,1.0),
            ..default()
        },
        ..default()
        }).insert(Player2);

        commands
        .spawn()
        .insert(B)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(10.0, 10.0, 0.0),
                ..default()
        },
        sprite: Sprite {
            color: Color::rgb(1.0,1.0,1.0),
            ..default()
        },
        ..default()
    });

    commands
    .spawn()
    .insert_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 350.0, 0.0),
            scale: Vec3::new(2000.0, 1.0, 0.0),
            ..default()
    },
    sprite: Sprite {
        color: Color::rgb(1.0,1.0,1.0),
        ..default()
    },
    ..default()
    });

    commands
    .spawn()
    .insert_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, -350.0, 0.0),
            scale: Vec3::new(2000.0, 1.0, 0.0),
            ..default()
    },
    sprite: Sprite {
        color: Color::rgb(1.0,1.0,1.0),
        ..default()
    },
    ..default()
    });

    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Score: ".to_string(),
                    style: TextStyle {
                        font_size: 100.0,
                        color: Color::rgb(1.0,1.0,1.0),
                        ..default()
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font_size: 100.0,
                        color: Color::rgb(1.0,1.0,1.0),
                        ..default()
                    },
                },
            ],
            ..default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            },
            ..default()
        },
        ..default()
    });


}

fn client_connection_system(
    mut commands: Commands, 
    mut events: EventReader<client::ConnectionEstablishEvent<Config>>
) {

}
fn client_packet_receive_system(
    mut events: EventReader<client::PacketReceiveEvent<Config>>,
    mut game: ResMut<GameState>,
    mut player: ResMut<Player>
) {
    for event in events.iter() {
        match &event.packet {
            ServerPacket::GameState(p1,p2,ball,score) => {
                game.p1 = *p1;
                game.p2 = *p2;
                game.ball = *ball;
                game.score = *score;
            },
            ServerPacket::Clients(c) => {
                println!("recieved client");
                if *c == (1 as u32) {
                    *player = Player::One(One::None);
                }
                else if *c == (2 as u32) {
                    *player = Player::Two(Two::None);
                }
                else {
                    *player = Player::Spectator;
                }
            },
        }
        event
            .connection
            .send(ClientPacket::Player(*player))
            .unwrap();
    }
}

fn move_paddle(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: ResMut<Player>
) {
    if keyboard_input.pressed(KeyCode::Up) {
        match *player {
            Player::One(_) => *player = Player::One(One::Up),
            Player::Two(_) => *player = Player::Two(Two::Up),
            _ => *player = Player::Spectator,
        }
    }
    else if keyboard_input.pressed(KeyCode::Down) {
        match *player {
            Player::One(_) => *player = Player::One(One::Down),
            Player::Two(_) => *player = Player::Two(Two::Down),
            _ => *player = Player::Spectator,
        }
    }
    else {
        match *player {
            Player::One(_) => *player = Player::One(One::None),
            Player::Two(_) => *player = Player::Two(Two::None),
            _ => *player = Player::Spectator,
        }
    }
}

fn display_player1(
    mut query: Query<&mut Transform, (With<Paddle>, With<Player1>)>,
    game_state: Res<GameState>
) {
    let mut paddle_transform = query.single_mut();
    paddle_transform.translation.x = game_state.p1.x;
    paddle_transform.translation.y = game_state.p1.y;
}

fn display_player2(
    mut query: Query<&mut Transform, (With<Paddle>, With<Player2>)>,
    game_state: Res<GameState>
) {
    let mut paddle_transform = query.single_mut();
    paddle_transform.translation.x = game_state.p2.x;
    paddle_transform.translation.y = game_state.p2.y;
}

fn display_ball(
    mut query: Query<&mut Transform, With<B>>,
    game_state: Res<GameState>,
) {
    let mut paddle_transform = query.single_mut();
    paddle_transform.translation.x = game_state.ball.x;
    paddle_transform.translation.y = game_state.ball.y;
}

fn display_score(
    game: Res<GameState>
) {
    
}