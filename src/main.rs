#![feature(globs)]

extern crate graphics;
extern crate sdl2_window;
extern crate shader_version;
extern crate opengl_graphics;
extern crate event;
extern crate sdl2;
extern crate gfx;
extern crate gfx_graphics;
extern crate input;
extern crate current;

use event::{
    Events,
    Ups,
    MaxFps,
    WindowSettings,
    Window
};

use current::Set;

use sdl2_window::Sdl2Window;
use gfx::{Device, DeviceHelper};
use gfx_graphics::{
G2D,
};

use engine::ConwayEngine;
use world::World;

use std::cell::RefCell;
pub mod cell;
pub mod world;
pub mod engine;

fn main() {
    let opengl = shader_version::opengl::OpenGL_3_2;
    let window = Sdl2Window::new(
        opengl,
        WindowSettings {
            title: "Conway".to_string(),
            size: [600, 600],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0
        }
    );

    let mut world = world::HashWorld::new();

    world.set_cell(1, 0);
    world.set_cell(2, 1);
    world.set_cell(0, 2);
    world.set_cell(1, 2);
    world.set_cell(2, 2);

    let mut engine = engine::GrifLife::new(box world);

    let mut device = gfx::GlDevice::new(|s| unsafe {
        std::mem::transmute(sdl2::video::gl_get_proc_address(s))
    });
    let (w, h) = (600, 600);
    let frame = gfx::Frame::new(w as u16, h as u16);
    let mut renderer = device.create_renderer();
    
    let mut g2d = G2D::new(&mut device);

    let mut draw = false;
    let mut run = false;
    //number of generations per second, cannot exceed updates_per_second
    let gen_speed = 50i;
    let mut updates_since_gen = 0i;

    let mut offx = 0.0;
    let mut offy = 0.0;
    let mut move_scr = false;
    for e in Events::new(&RefCell::new(window)).set(Ups(120)).set(MaxFps(60)) {
        use event::{ RenderEvent, MouseCursorEvent, MouseRelativeEvent, PressEvent, ReleaseEvent, UpdateEvent};
        e.render(|_| {
            g2d.draw(&mut renderer, &frame, |c, g| {
                use graphics::*;
                c.rgb(1.0, 1.0, 1.0).draw(g);
                for (location, cell) in engine.world_ref().iter() {
                    let (state, (x, y)) = (*cell, *location);
                    if state == cell::Alive {
                        c.rect(x as f64 * 10.0 - offx, y as f64 * 10.0 - offy, 10.0, 10.0).rgb(1.0, 0.0, 0.0).draw(g);
                    }
                }
            });
            device.submit(renderer.as_buffer());
            renderer.reset();
        });

        e.press(|button| {
            if button == input::Mouse(input::mouse::Left) {
                draw = true;
            }
            if button == input::Keyboard(input::keyboard::Space) {
                run = !run;
            }
            if button == input::Mouse(input::mouse::Right) {
                move_scr = true;
            }
        });

        e.release(|button| {
            if button == input::Mouse(input::mouse::Left) {
                draw = false;
            }
            if button == input::Mouse(input::mouse::Right) {
                move_scr = false;
            }
        });
        
        e.update(|_| {
            updates_since_gen += 1;
            if updates_since_gen % (120 / gen_speed) == 0 {
                //make sure we are now drawing
                if !draw && run {
                    engine.next_generation();
                }
                updates_since_gen = 0;
            }
        });

        if move_scr {
            e.mouse_relative(|dx, dy| {
                offx += dx;
                offy += dy;
            });
        }

        if draw {
            e.mouse_cursor(|x, y| {
                let (x, y) = ((x as u32) / 10, (y as u32) / 10);
                if x < w && y < h {
                    engine.set_cell(x as int, y as int);
                }
            });
        }
    }
    
}
