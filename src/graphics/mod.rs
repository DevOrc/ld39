extern crate cannon;

mod util;
pub mod consts;

use cannon::*;
use self::util::rect;
use super::{Powerup, Field};
use super::util::Position;

pub struct Graphics{
    console: Console,
    status: String
}

impl Graphics{
    pub fn new() -> Graphics{
        let mut console = Console::new();
        console.set_should_cls(true);

        Graphics {console: console, status: "Welcome to LD 39".to_string()}
    }

    pub fn draw_main_menu(&mut self){
        draw_main_menu_top(&mut self.console);
        draw_main_menu_center(&mut self.console);
        draw_main_menu_info(&mut self.console);
    }

    pub fn redraw_background(&mut self){
        self.console.set_color(color::BLACK, color::BLACK);
        self.console.clear_screen();

        draw_field(&mut self.console);
        draw_status_bar(&mut self.console, &self.status);
        draw_menu(&mut self.console);
    }

    pub fn draw_end_game(&mut self, time: u64){
        let start_x = (consts::WORLD_WIDTH / 2) - 7;
        let start_y = (consts::WORLD_HEIGHT / 2) - 3;
        self.console.set_color(color::BLACK, color::BLUE);
        rect(&mut self.console, start_x - 1, start_y -1, start_x + 14, start_y + 6);
        self.console.set_color(color::BLACK, color::LIGHT_GRAY);
        rect(&mut self.console, start_x, start_y, start_x + 13, start_y + 5);
        self.console.set_cursor_position(start_x + 2, start_y + 1);
        self.console.write("Game Over!");
        self.console.set_cursor_position(start_x + 2, start_y + 2);
        self.console.write(&format!("Time: {}", time / 1000));
        self.console.set_cursor_position(start_x + 2, start_y + 3);
        self.console.write("R = Restart");
        self.console.set_cursor_position(start_x + 2, start_y + 4);
        self.console.write("Q = Quit");
    }

    pub fn set_status(&mut self, status: &str){
        self.status = status.to_string();
        draw_status_bar(&mut self.console, &self.status);
    }

    pub fn draw_powerups(&mut self, field: &Field){
        for x in 0..consts::WORLD_WIDTH{
            for y in 0..consts::WORLD_HEIGHT{
                if let Some(powerup) = field[x as usize][y as usize]{
                    match powerup {
                        Powerup::Charge => draw_charge(&mut self.console, x, y),
                        Powerup::Destroyed => draw_ash(&mut self.console, x, y),
                        Powerup::Cleanup => draw_cleanup(&mut self.console, x, y),
                        Powerup::Reset => draw_reset(&mut self.console, x, y),
                    }
                }
            }
        }
    }

    pub fn draw_menu_ui(&mut self, time: u64, ash_rate: u16, powerups: u16){
        draw_stats(&mut self.console, time, ash_rate, powerups);
        draw_instructions(&mut self.console);
        draw_title(&mut self.console);
        self.console.set_cursor_position(0,0);
    }

    pub fn remove_ash(&mut self, x: i16, y: i16){
        self.console.set_color(color::BROWN, color::BROWN);
        self.console.write_character(x, y + consts::MENU_HEIGHT + 1, 32);
    }

    pub fn update_charge(&mut self, charge: i16){
        self.console.set_color(color::BLACK, color::LIGHT_GRAY);
        rect(&mut self.console, 0, consts::WORLD_HEIGHT + 1, consts::WORLD_WIDTH, consts::WORLD_HEIGHT + 2);
        self.console.set_color(color::WHITE, color::YELLOW);
        rect(&mut self.console, 0, consts::WORLD_HEIGHT + 1, charge * 2, consts::WORLD_HEIGHT + 2);
        self.console.set_color(color::BLACK, color::YELLOW);
        self.console.set_cursor_position(0, consts::WORLD_HEIGHT + 1);
        self.console.write("Power\nLevel");
        self.console.set_cursor_position(0,0);
    }

    pub fn draw_player(&mut self, player: &Position){
        self.console.set_color(color::BLUE, color::BROWN);
        self.console.set_cursor_position(player.x, player.y + consts::MENU_HEIGHT);
        self.console.write("@");
        self.console.set_cursor_position(0,0);
    }

    pub fn move_player(&mut self, player: &Position, prev_player_pos: &Position){
        self.console.set_color(color::DARK_RED, color::BROWN);
        self.console.write_character(prev_player_pos.x,
            prev_player_pos.y + consts::MENU_HEIGHT, 32);
        self.draw_player(player);
    }
}

pub fn draw_main_menu_center(console: &mut Console){
    console.set_color(color::BLACK, color::LIGHT_GRAY);
    draw_centered_string(console, "In a world far in the future a series of events causes massive", 7);
    draw_centered_string(console, "volcanic events. Nearby a volcano, Kuklonfusta, you hear an", 8);
    draw_centered_string(console, "eruption begin ! Kuklonfust puffs out smoke covering the sky,", 9);
    draw_centered_string(console, "disabling all solar panels in the area. You must keep on collecting", 10);
    draw_centered_string(console, "energy to stay alive! But beware! The volcanic magma is starting to fall...", 11);
    draw_centered_string(console, "Controls:", consts::TOTAL_HEIGHT - 10);
    draw_centered_string(console, "Use Arrow Keys To Move", consts::TOTAL_HEIGHT - 9);
    draw_centered_string(console, "Move over powerups to use them", consts::TOTAL_HEIGHT - 8);
    draw_centered_string(console, "To Start the Game Press Enter", consts::TOTAL_HEIGHT - 7);
    draw_centered_string(console, "To Quit the Game Press Escape", consts::TOTAL_HEIGHT - 6);
}

pub fn draw_main_menu_info(console: &mut Console){
    console.set_color(color::BLACK, color::LIGHT_GRAY);
    draw_centered_string(console,
        "Powerups:", 15);
    draw_centered_string(console,
        "₽: Increases the power in your battery (Bottom of the Screen)", 16);
    draw_centered_string(console,
        "C: Removes 66% of the magama. But it will double the rate the volcano spews lava", 17);
    draw_centered_string(console,
        "R: Calms the volcano", 18);
    draw_centered_string(console,
        "Note: Walking on Magma crimples your battery!", 21);
}

pub fn draw_centered_string(console: &mut Console, string: &str, y: i16){
    let chars: Vec<char> = string.chars().collect();
    let width = consts::WORLD_WIDTH;
    console.set_cursor_position((width / 2) - ((chars.len() / 2) as i16), y);
    console.write(string);
}

pub fn draw_main_menu_top(console: &mut Console){
    console.set_color(color::BLACK, color::BLUE);
    rect(console, 0, 0, consts::WORLD_WIDTH, consts::TOTAL_HEIGHT);
    console.set_color(color::BLACK, color::LIGHT_GRAY);
    rect(console, 1, 1, consts::WORLD_WIDTH - 1, consts::TOTAL_HEIGHT - 4);
    draw_centered_string(console, "Ash Dash", 2);
    draw_centered_string(console, "A Game for LD 39", 3);
    console.set_color(color::BLACK, color::DARK_GRAY);
}

pub fn draw_stats(console: &mut Console, time: u64, ash_rate: u16, powerups: u16){
    console.set_color(color::BLACK, color::LIGHT_GRAY);
    console.set_cursor_position(0,1);
    console.write(&format!("Time: {} \nAsh Rate: {}\nPowerups Collected: {}", time / 1000, ash_rate, powerups));
}

pub fn draw_title(console: &mut Console){
    console.set_color(color::RED, color::LIGHT_GRAY);
    console.set_cursor_position((consts::WORLD_WIDTH / 2) - 4, 0);
    console.write("ASH DASH");
    console.set_cursor_position((consts::WORLD_WIDTH / 2) - 15, consts::MENU_HEIGHT - 1);
    console.write("Press Escape To Close The Game");
}

pub fn draw_instructions(console: &mut Console){
    console.set_cursor_position(consts::WORLD_WIDTH - 31, 0);
    console.write("Collect Cs to cleanup the Lava!");
    console.set_cursor_position(consts::WORLD_WIDTH - 31, 1);
    console.write("Collect ₽s to gain more power!");
    console.set_cursor_position(consts::WORLD_WIDTH - 31, 2);
    console.write("Collect Rs to slow the volcano!");
    console.set_cursor_position(consts::WORLD_WIDTH - 31, 4);
    console.write("Lava cuts you battery in half!");
    //console.set_cursor_position(consts::WORLD_WIDTH - 37, 5);
}

fn draw_charge(console: &mut Console, x: i16, y: i16){
    console.set_cursor_position(x, y + consts::MENU_HEIGHT + 1);
    console.set_color(color::YELLOW, color::BROWN);
    console.write("₽");
}

fn draw_reset(console: &mut Console, x: i16, y: i16){
    console.set_cursor_position(x, y + consts::MENU_HEIGHT + 1);
    console.set_color(color::YELLOW, color::BROWN);
    console.write("R");
}

fn draw_cleanup(console: &mut Console, x: i16, y: i16){
    console.set_cursor_position(x, y + consts::MENU_HEIGHT + 1);
    console.set_color(color::YELLOW, color::BROWN);
    console.write("C");
}

fn draw_ash(console: &mut Console, x: i16, y: i16){
    //console.set_cursor_position(x, y + consts::MENU_HEIGHT + 1);
    console.set_color(color::DARK_RED, color::DARK_RED);
    console.write_character(x, y + consts::MENU_HEIGHT + 1, 32)
}

fn draw_field(console: &mut Console){
    console.set_color(color::BLACK, color::BROWN);
    rect(console, 0, 0, consts::WORLD_WIDTH, consts::WORLD_HEIGHT);
}

fn draw_status_bar(console: &mut Console, message: &str){
    let height = consts::WORLD_HEIGHT;
    let width = consts::WORLD_WIDTH;

    console.set_color(color::BLACK, color::LIGHT_GRAY);
    rect(console, 0, height + 1, width, height + consts::STATUS_BAR_HEIGHT);

    console.set_color(color::BLACK, color::DARK_BLUE);
    rect(console, 0, height, width, height);//Top
    rect(console, 0, height + 3, width, height + 3);//Middle

    console.set_color(color::BLACK, color::LIGHT_GRAY);
    console.set_cursor_position(2, height + 4);
    console.write(message);
}

fn draw_menu(console: &mut Console){
    let width = consts::WORLD_WIDTH;

    console.set_color(color::BLACK, color::DARK_BLUE);
    rect(console, 0, consts::MENU_HEIGHT, width, consts::MENU_HEIGHT);

    console.set_color(color::BLACK, color::LIGHT_GRAY);
    rect(console, 0, 0, width, consts::MENU_HEIGHT - 1);
}
