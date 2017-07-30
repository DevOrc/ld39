extern crate cannon;

use super::Game;
use cannon::{Console};
use cannon::input::*;
use std::thread;
use std::sync::mpsc::{channel, Receiver};

pub struct InputSystem{
    rx: Receiver<Key>
}

impl InputSystem{
    pub fn poll(&self) -> Option<Key>{
        match self.rx.try_recv() {
            Err(_) => None,
            Ok(k) => Some(k)
        }
    }

    pub fn update(&self, game: &mut Game, key: Key){
        match key{
            Key::Left => game.player_pos.add_x(-1),
            Key::Right => game.player_pos.add_x(1),
            Key::Up => game.player_pos.add_y(-1),
            Key::Down => game.player_pos.add_y(1),
            _ => (),
        }
    }
}

pub fn init() -> InputSystem{
    let (tx, rx) = channel();

    thread::spawn(move ||{
        let mut console = Console::new();
        console.set_should_cls(false);

        loop{
            let input =  console.poll_input();

            if let Some(i) = input{
                let key_opt = match i.EventType{
                    1 => to_key(i.Event),
                    _ => None,
                };

                if let Some(key) = key_opt{
                    tx.send(key).unwrap_or_else(|err| {
                        panic!("Input System Channel Error: {}", err);
                    });
                }
            }
        }
    });

    InputSystem {rx: rx}
}

fn to_key(event: [u32;4]) -> Option<Key>{
    if event[0] == 1{
        return None;
    }

    num_to_key(event[2])
}
