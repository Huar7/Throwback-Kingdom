use std::str::MatchIndices;

use macroquad::prelude::*;

mod character_entity;
use crate::character_entity::*;


mod tk_system;
use crate::tk_system::*;


const PAN_SPEED:f32 = 2.0;

enum GameState{
    Play,
    Pause,
    Menu,
    Setting,
    GameOver
}


#[macroquad::main("Throwback Kingdom")]
async fn main() {

    let mut accel = 0.0;
    let delta_time = get_frame_time();
    let fps = get_fps();
    
    let game_start_status = GameState::Play;

    let mut character_list : Vec<CharacterEntity> = Vec::new();
    character_list.push(CharacterEntity::new("King Anton".to_string(), 100, 300.0, screen_width()/2.0, screen_height()/2.0, CharacterType::PLAYER, SpriteTypes::Placeholder, character_list.len()));
    character_list.push(CharacterEntity::new("Waiter Alfred".to_string(), 100, 400.0, screen_width()/4.0, screen_height()/2.0, CharacterType::PLAYER, SpriteTypes::Placeholder, character_list.len()));
    character_list.push(CharacterEntity::new("Waiter Alfred".to_string(), 100, 800.0, screen_width()/-4.0, screen_height()/-2.0, CharacterType::PLAYER, SpriteTypes::Placeholder, character_list.len()));
    // character_list.remove(2);
    

    let mut character_main_id = 0;
    let mut camera_zoom_mode :f32 = 2.0;

    let mut ls_pos_cam:[f32;2] = [0.0,0.0]; // inisialisasi Posisi Camera Berdasarkan Player
    let mut final_rpg_position: [f32;2] = [0.0, 0.0];
    let mut current_mouse_position: (f32, f32) = (0.0, 0.0); // Mouse Position For Difference

    let mut game_mode: bool = true; // true = RPG, false = RTS

    loop {
        match game_start_status{
            GameState::Menu => {},

            GameState::Play => {

                // test mouse
                let mut mospos = mouse_position();


                

                
                clear_background(SKYBLUE);


                // Game Mode Switch
                if is_key_pressed(KeyCode::Tab){
                    game_mode = !game_mode; // Mengganti Mode dari RPG ke RTS atau sebaliknya
                    final_rpg_position = [character_list[character_main_id].x, character_list[character_main_id].y]; // untuk posisi camera terakhir dari player ketika transisi
                    if game_mode == false {camera_zoom_mode = 1.0};
                    character_list.remove(character_main_id);
                }

                match game_mode{
                    true => { //  RPG MODE //
                    camera_zoom_mode = 2.0;
                // Player Movement
                        let mut mvspeed = accel + f32::powf(accel,3.0 );
                        let movement_pressed = count_key_pressed(is_key_down(KeyCode::W), is_key_down(KeyCode::A), is_key_down(KeyCode::S), is_key_down(KeyCode::D)); 
                        let vector_pressed = get_vector(is_key_down(KeyCode::S), is_key_down(KeyCode::A), is_key_down(KeyCode::W), is_key_down(KeyCode::D)); // ini ada untuk mendapatkan nilai dari tombol2 yang dipencet (khusus untuk pergerakan)

                        if is_key_pressed(KeyCode::F){
                            character_main_id += 1;
                        }
                            character_main_id = character_main_id % character_list.len();

                        if is_key_pressed(KeyCode::P) {character_list = test_delete(character_list)}
                        

                        if movement_pressed > 0 {
                            accel += 1.5 * (delta_time*2.0);
                            accel = clamp(accel, 0.0, 0.685);
                        }
                        else {accel = 0.0}

                        if movement_pressed >= 2{mvspeed = mvspeed/1.5}
                        else{mvspeed = mvspeed};

                        character_list[character_main_id].x += vector_pressed[0] * (mvspeed * delta_time * character_list[character_main_id].speed);
                        character_list[character_main_id].y += vector_pressed[1] * (mvspeed * delta_time * character_list[character_main_id].speed);

                        ls_pos_cam = [character_list[character_main_id].x, character_list[character_main_id].y]; // ini untuk mendapatkan nilai ketika plyaer bergerak pada RPG mode
                        
                    },
                    false => { // RTS MODE //

                        
                        if is_mouse_button_pressed(MouseButton::Middle){
                            current_mouse_position = mouse_position();
                        }

                        if is_mouse_button_down(MouseButton::Middle){
                            final_rpg_position = [final_rpg_position[0] - (mospos.0 - current_mouse_position.0) * delta_time * PAN_SPEED, final_rpg_position[1] - (mospos.1 - current_mouse_position.1) * delta_time * PAN_SPEED]
                        }
        

                        ls_pos_cam = [final_rpg_position[0], final_rpg_position[1]];

                        
                    }
                    
                }

                
                // Camera

                set_camera(&Camera2D{
                    zoom: Vec2 {x: camera_zoom_mode / screen_width(), y: camera_zoom_mode/screen_height()},
                    target:Vec2{x: ls_pos_cam[0], y: ls_pos_cam[1]}, // Target akan diubah ke ls_pos_cam
                    ..Default::default()
                });
                


                // Placeholder Player
                for i in 0..character_list.len(){
                    match character_list[i].sprite{
                        SpriteTypes::Placeholder => {
                            draw_circle(character_list[i].x, character_list[i].y, 15.0, YELLOW)
                    },
                        _ => {}
                    }
                }; // bekerja sesuai ekspektasi
                
                draw_text( format!("{} {}",character_list[character_main_id].x,character_list[character_main_id].y).as_str() , (screen_width()/8.0) - 100.0, (screen_height()/8.0) - 50.0, 30.0, DARKGRAY );
                draw_text (format!("{}",fps).as_str(),screen_width() - 30.0,(screen_height()/8.0) - 50.0,30.0,DARKGRAY);
                
                set_default_camera();
                next_frame().await
            },

            GameState::Pause => {},

            GameState::Setting => {},

            GameState::GameOver=> {},
            GameState::Pause => {},

            GameState::Setting => {},

            GameState::GameOver=> {},
            
        }
    }
}


fn test_delete (mut data: Vec<CharacterEntity>) -> Vec<CharacterEntity> {
    println!("ok");
    data.remove(data.len() - 1);
    data
}
