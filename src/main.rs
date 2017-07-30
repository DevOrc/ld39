extern crate cannon;
extern crate rand;
extern crate chroniker;

pub mod input;
pub mod graphics;
pub mod util;

use rand::{Rng, thread_rng, ThreadRng};
use cannon::Console;
use chroniker::Timer;
use cannon::input::Key;
use graphics::Graphics;
use input::*;
use graphics::consts as settings;
use util::*;

pub type Field = [[Option<Powerup>; settings::WORLD_HEIGHT as usize]; settings::WORLD_WIDTH as usize];

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Powerup{
    Cleanup,
    Charge,
    Reset,
    Destroyed
}

enum State{
    MainMenu,
    Quit,
    Game
}

pub struct Game{
    player_pos: Position,
    items: Field
}

impl Game{
    fn new() -> Game{
        Game {
            player_pos: Position{x: 15, y: 15},
            items:  [[None; settings::WORLD_HEIGHT as usize]; settings::WORLD_WIDTH as usize]
        }
    }
}

#[cfg(windows)]
fn main() {
    let mut console = Console::new();//Passed
    let mut graphics = Graphics::new();//Passed
    let input = input::init();//Passed
    let mut state = State::MainMenu;
    console.set_should_cls(false);

    'main: loop{
        state = match state{
            State::Game => run_game(&mut console, &mut graphics, &input),
            State::MainMenu => run_menu(&mut console, &mut graphics, &input),
            State::Quit => break 'main,
        }
    }



}

fn run_menu(console: &mut Console, graphics: &mut Graphics, input: &InputSystem) -> State{
    let mut size = console.get_console_size();
    graphics.draw_main_menu();
    loop{
        if let Some(k) = input.poll(){
            match k {
                Key::Escape => return State::Quit,
                Key::Enter => return State::Game,
                _ => (),
            }
        }

        if size != console.get_console_size(){
            size = console.get_console_size();
            graphics.draw_main_menu();
        }
    }
}

fn run_game(console: &mut Console, graphics: &mut Graphics, input: &InputSystem) -> State{
    let mut size = console.get_console_size();//Created
    let mut game = Game::new();//Created
    let mut charge = 10;//Created
    let mut timer: Timer = Timer::new();//Created
    let start_time = chroniker::current_time_millis();//Created
    let mut rng = thread_rng();//Created
    let mut ash_rate = 0;
    let mut powerups_collected = 0;

    for _ in 0..25{
        spawn_charge(&mut game.items, &mut rng);
    }

    for _ in 0..10{
        spawn_cleanup(&mut game.items, &mut rng);
    }

    for _ in 0..10{
        spawn_reset(&mut game.items, &mut rng);
    }

    game.items[3][3] = Some(Powerup::Charge);
    graphics.redraw_background();
    graphics.draw_player(&game.player_pos);
    graphics.draw_powerups(&game.items);

    loop {
        if let Some(k) = input.poll(){
            if k == Key::Escape{
                return State::Quit;
            }else{
                let prev_player_pos = game.player_pos.clone();
                input.update(&mut game, k);
                if prev_player_pos != game.player_pos{
                    graphics.move_player(&game.player_pos, &prev_player_pos);
                }
                if let Some(powerup) = game.items[game.player_pos.x as usize][(game.player_pos.y - 1)as usize]{
                    powerups_collected += 1;
                    match powerup{
                        Powerup::Charge => {
                            charge += 5;
                            spawn_charge(&mut game.items, &mut rng);
                            ()},
                        Powerup::Destroyed => charge /= 2,
                        Powerup::Reset => {ash_rate = 0; spawn_reset(&mut game.items, &mut rng); ()},
                        Powerup::Cleanup => {
                            cleanup(&mut game.items, &mut rng, graphics);
                            ash_rate *= 2;
                            ()
                            }
                    }
                }
            }
            game.items[game.player_pos.x as usize][(game.player_pos.y - 1) as usize] = None;
        }

        if charge > 70{
            charge = 70;
        }

        if charge < 1{
            graphics.set_status(&format!("GAME OVER! Time: {} seconds",
                (chroniker::current_time_millis() - start_time) / 1000));
            let end_time = chroniker::current_time_millis();
            size = console.get_console_size();
            console.set_console_size(size.width, size.height -1);//Updates buffer?
            graphics.redraw_background();
            graphics.draw_player(&game.player_pos);
            graphics.draw_powerups(&game.items);
            graphics.draw_menu_ui(end_time - start_time, ash_rate, powerups_collected);
            graphics.draw_end_game(end_time - start_time);

            loop{
                if let Some(k) = input.poll(){
                    if k == Key::Escape || k == Key::Q{
                        return State::MainMenu;
                    }else if k == Key::R{
                        return State::Game;
                    }
                }

                if size != console.get_console_size(){
                    size = console.get_console_size();
                    console.set_console_size(size.width, size.height -1);//Updates buffer?
                    graphics.redraw_background();
                    graphics.draw_player(&game.player_pos);
                    graphics.draw_powerups(&game.items);
                    graphics.draw_menu_ui(end_time - start_time, ash_rate, powerups_collected);
                    graphics.draw_end_game(end_time - start_time);
                }
            }
            break;
        }

        if timer.elapsed_millis() > 1000{
            charge -= 2;
            graphics.draw_powerups(&game.items);
            graphics.update_charge(charge);
            graphics.draw_menu_ui(chroniker::current_time_millis() - start_time, ash_rate, powerups_collected);
            timer.reset();
            rng = thread_rng();
            ash_rate += 1;
            for _ in 0..(ash_rate){
                drop_ash(&mut game.items, &mut rng);
            }
        }

        if console.get_console_size() != size{
            size = console.get_console_size();
            console.set_console_size(size.width, size.height -1);//Updates buffer?
            graphics.redraw_background();
            graphics.draw_player(&game.player_pos);
            graphics.draw_powerups(&game.items);
        }
    }
    State::MainMenu
}

fn drop_ash(field: &mut Field, rng: &mut ThreadRng){
    let x =  rng.gen_range(0, settings::WORLD_WIDTH) as usize;
    let y =  rng.gen_range(0, settings::WORLD_HEIGHT - 6) as usize;

    if field[x][y] == Some(Powerup::Destroyed){
        drop_ash(field, rng);
    }else{
        field[x][y] = Some(Powerup::Destroyed);
    }
}

fn spawn_cleanup(field: &mut Field, rng: &mut ThreadRng){
    let x =  rng.gen_range(0, settings::WORLD_WIDTH) as usize;
    let y =  rng.gen_range(0, settings::WORLD_HEIGHT - 6) as usize;

    if field[x][y] == None{
        field[x][y] = Some(Powerup::Cleanup);
    }else{
        spawn_cleanup(field, rng);
    }
}

fn cleanup(field: &mut Field, rng: &mut ThreadRng, graphics: &mut graphics::Graphics){
    spawn_cleanup(field, rng);

    for x in 0..settings::WORLD_WIDTH{
        for y in 0..settings::WORLD_HEIGHT{
            if let Some(powerup) = field[x as usize][y as usize]{
                if powerup == Powerup::Destroyed{
                    if rng.gen_range(0,100) <= 66{
                        field[x as usize][y as usize] = None;
                        graphics.remove_ash(x, y);
                    }
                }
            }
        }
    }
}

fn spawn_charge(field: &mut Field, rng: &mut ThreadRng){
    let x =  rng.gen_range(0, settings::WORLD_WIDTH) as usize;
    let y =  rng.gen_range(0, settings::WORLD_HEIGHT - 6) as usize;

    if field[x][y] == None{
        field[x][y] = Some(Powerup::Charge);
    }else{
        spawn_charge(field, rng);
    }
}

fn spawn_reset(field: &mut Field, rng: &mut ThreadRng){
    let x =  rng.gen_range(0, settings::WORLD_WIDTH) as usize;
    let y =  rng.gen_range(0, settings::WORLD_HEIGHT - 6) as usize;

    if field[x][y] == None{
        field[x][y] = Some(Powerup::Reset);
    }else{
        spawn_charge(field, rng);
    }
}

#[cfg(not(windows))]
fn main() {
    println!("This game only works on Windows! Sorry! 5s Please!");
}
