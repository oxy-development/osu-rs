#![feature(collections)]
#![feature(convert)]
#![feature(time)]
#![feature(duration)]
#![feature(wait_timeout)]
#![feature(no_std)]
#![feature(link_args)]
#![feature(std_misc)]
#![feature(slice_patterns)]
#![feature(path_ext)]
#![allow(unused_features)]
#![feature(test)]

//global
extern crate toml;
extern crate rustc_serialize;
extern crate time;

//assets
extern crate flate2;

//graphics
extern crate piston;
extern crate fps_counter;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate sdl2;
extern crate sdl2_window;

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

use sdl2_window::Sdl2Window as Window;
use piston::window::{ AdvancedWindow, WindowSettings };
use piston::event::*;
use piston::input::{ Button, Key, Input };

fn main() {
    let mut Game = game::container::Container::create();
    let ups = Game.config.video.ups_limit;
    let max_fps = Game.config.video.fps_limit;

    let window: Window = WindowSettings::new("osu_rs", [Game.config.video.width, Game.config.video.height])
        .exit_on_esc(true).vsync(Game.config.video.enable_vsync).samples(0).into();

    let mut fps_counter = fps_counter::FPSCounter::new();
    let mut fps = fps_counter.tick();

    let ref window = Rc::new(RefCell::new(window));
    let ref Game = Rc::new(RefCell::new(Game));
    for e in window.clone().events()
        .ups(ups)
        .max_fps(max_fps) {
            match e {
                Event::Render(_) => {
                    //Only rendering GameState -> Screen should be placed there
                    fps = fps_counter.tick();
                }
                Event::AfterRender(_) => {
                    // device.cleanup();
                }
                Event::Input(Input::Press(Button::Keyboard(Key::C))) => {
                    Game.borrow_mut().app.set_state(GameState::Initial);
                }

                Event::Update(_) => {
                    //Game loop logics should be placed there
                    let title = format!("osu-rs @ {}FPS @ {:?}", fps, Game.borrow().app.current_state);
                    window.borrow_mut().set_title(title);
                }
                _ => {}
            }
    }

}
