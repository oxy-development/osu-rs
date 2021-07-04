// #![feature(collections)]
// #![feature(convert)]
// #![feature(time)]
// #![feature(duration)]
// #![feature(wait_timeout)]
// #![feature(no_std)]
// #![feature(link_args)]
// #![feature(std_misc)]
// #![feature(slice_patterns)]
// #![feature(path_ext)]
// #![allow(unused_features)]
// #![feature(test)]

//internal
//Beatmaps mod. Was renamed due to name collision in external and internal names
mod map;
use map::*;

mod game;
use game::*;

use game::stated_game_app::GameState;
use game::stated_game_app::StateMachine;

use gfx::traits::{Device, Stream, StreamFactory};
use std::cell::RefCell;
use std::cmp::max;
use std::f32::consts::PI;
use std::f32::INFINITY;
use std::rc::Rc;

use sdl2_window::Sdl2Window;
use piston::window::{ AdvancedWindow, WindowSettings };
use piston::input::{Button, Key, Input, Event};
use std::ops::Deref;
use piston::event_loop::EventLoop;
use piston::event_loop::Events;
use piston::event_loop::WindowEvents;

fn main() {
    let mut Game = game::container::Container::create();
    let ups = Game.config.video.ups_limit;
    let max_fps = Game.config.video.fps_limit;
    
    let window_settings=WindowSettings::new("osu_rs", [Game.config.video.width, Game.config.video.height])
        .exit_on_esc(true).vsync(Game.config.video.enable_vsync).samples(0);
    let mut window = Sdl2Window::new(window_settings).map_err(|e|eprint!("{}",e)).unwrap();

    let mut fps_counter = fps_counter::FPSCounter::new();
    let mut fps = fps_counter.tick();
    
    let mut events = window.events()
        .ups(ups)
        .max_fps(max_fps);
    let mut should_close = false;
    
    while !should_close {
        let event= events.next(&mut window);
        if event.is_none() {
            should_close=true;
            continue;
        }
            match event.unwrap() {
                Event::Render(_) => {
                    //Only rendering GameState -> Screen should be placed there
                    fps = fps_counter.tick();
                }
                Event::AfterRender(_) => {
                    // device.cleanup();
                }
                Event::Input(Input::Press(Button::Keyboard(Key::C))) => {
                    Game.app.set_state(GameState::Initial);
                }
                Event::Update(_) => {
                    //Game loop logics should be placed there
                    let title = format!("osu-rs @ {}FPS @ {:?}", fps, Game.app.current_state);
                    window.set_title(title);
                }
                _ => {}
            }
    }

}
