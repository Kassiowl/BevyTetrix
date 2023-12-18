
use bevy::prelude::*;



#[derive(Component)]
pub enum TetrominoType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z
}

#[derive(Bundle)]
struct Tetromino
{
    sprite: SpriteBundle,
    tetromino_type: TetrominoType
}

#[derive(Bundle)]
struct TetrominoGhost
{
    sprite: SpriteBundle,
    tetromino_type: TetrominoType
}


pub fn render_tetromino(commands: &mut Commands, asset_server: &Res<AssetServer>, tetromino_type: TetrominoType)
{
    
    let i_texture = asset_server.load("tetromino/Shape Blocks/I.png");
    let i_texture_ghost = asset_server.load("tetromino/Ghost/I.png");

    let j_texture = asset_server.load("tetromino/Shape Blocks/J.png");
    let j_texture_ghost = asset_server.load("tetromino/Ghost/J.png");

    let l_texture = asset_server.load("tetromino/Shape Blocks/L.png");
    let l_texture_ghost = asset_server.load("tetromino/Ghost/L.png");

    
    let o_texture = asset_server.load("tetromino/Shape Blocks/O.png");
    let o_texture_ghost = asset_server.load("tetromino/Ghost/O.png");

    let s_texture = asset_server.load("tetromino/Shape Blocks/S.png");
    let s_texture_ghost = asset_server.load("tetromino/Ghost/S.png");

    let z_texture = asset_server.load("tetromino/Shape Blocks/Z.png");
    let z_texture_ghost = asset_server.load("tetromino/Ghost/Z.png");

    match tetromino_type {
        TetrominoType::I => {
                commands.spawn(Tetromino {
                    tetromino_type: TetrominoType::I,
                    sprite: SpriteBundle{
                        texture: i_texture.clone(),
                        transform: Transform::from_xyz(0., 380., 1.),
                        visibility: Visibility::Visible,
                        ..default()
                    }
                });
                commands.spawn(TetrominoGhost {
                    tetromino_type: TetrominoType::I,
                    sprite: SpriteBundle{
                        texture: i_texture_ghost,
                        transform: Transform::from_xyz(0., -380., 1.),
                        visibility: Visibility::Visible,
                        ..default()
                    }
                });
            },
        TetrominoType::J => {
                commands.spawn(Tetromino {
                    tetromino_type: TetrominoType::J,
                    sprite: SpriteBundle{
                        texture: j_texture,
                        transform: Transform::from_xyz(0., 380., 1.),
                        visibility: Visibility::Hidden,
                        ..default()
                    }
                });
                commands.spawn(TetrominoGhost {
                    tetromino_type: TetrominoType::J,
                    sprite: SpriteBundle{
                        texture: j_texture_ghost,
                        transform: Transform::from_xyz(0., -380., 1.),
                        visibility: Visibility::Hidden,
                        ..default()
                    }
                });  
        }

        TetrominoType::L =>{
                commands.spawn(Tetromino {
                    tetromino_type: TetrominoType::L,
                    sprite: SpriteBundle{
                        texture: l_texture,
                        transform: Transform::from_xyz(0., 380., 1.),
                        visibility: Visibility::Hidden,
                        ..default()
                    }
                });
                commands.spawn(TetrominoGhost {
                    tetromino_type: TetrominoType::L,
                    sprite: SpriteBundle{
                        texture: l_texture_ghost,
                        transform: Transform::from_xyz(0., -380., 1.),
                        visibility: Visibility::Hidden,
                        ..default()
                    }
                });  
        }

        TetrominoType::O => {
                commands.spawn(Tetromino {
                    tetromino_type: TetrominoType::O,
                    sprite: SpriteBundle{
                        texture: o_texture,
                        transform: Transform::from_xyz(0., 380., 1.),
                        visibility: Visibility::Hidden,
                        ..default()
                    }
                });
                commands.spawn(TetrominoGhost {
                    tetromino_type: TetrominoType::O,
                    sprite: SpriteBundle{
                        texture: o_texture_ghost,
                        transform: Transform::from_xyz(0., -380., 1.),
                        visibility: Visibility::Hidden,
                        ..default()
                    }
                });       
        }

        TetrominoType::S => {
                commands.spawn(Tetromino {
                    tetromino_type: TetrominoType::S,
                    sprite: SpriteBundle{
                        texture: s_texture,
                        transform: Transform::from_xyz(0., 380., 1.),
                        visibility: Visibility::Hidden,
                        ..default()
                    }
                });
                commands.spawn(TetrominoGhost {
                    tetromino_type: TetrominoType::S,
                    sprite: SpriteBundle{
                        texture: s_texture_ghost,
                        transform: Transform::from_xyz(0., -380., 1.),
                        visibility: Visibility::Hidden,
                        ..default()
                    }
                });         
        }

        TetrominoType::Z =>{

            commands.spawn(Tetromino {
                tetromino_type: TetrominoType::Z,
                sprite: SpriteBundle{
                    texture: z_texture,
                    transform: Transform::from_xyz(0., 380., 1.),
                    visibility: Visibility::Hidden,
                    ..default()
                }
            });
            commands.spawn(TetrominoGhost {
                tetromino_type: TetrominoType::Z,
                sprite: SpriteBundle{
                    texture: z_texture_ghost,
                    transform: Transform::from_xyz(0., -380., 1.),
                    visibility: Visibility::Hidden,
                    ..default()
                }
            });  
        }
        _ => {

        }
    }
    
}

