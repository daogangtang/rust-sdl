extern crate sdl;
extern crate sdl_image;

use std::path::Path;


use sdl::video::{SurfaceFlag, VideoFlag};
use sdl::event::{Event, Key};

fn main() {
    sdl::init(&[sdl::InitFlag::Video]);
    sdl::wm::set_caption("rust-sdl demo - image", "rust-sdl");

    let screen = match sdl::video::set_video_mode(800, 600, 32,
						  &[SurfaceFlag::HWSurface],
						  &[VideoFlag::DoubleBuf]) {
	Ok(screen) => screen,
	Err(err) => panic!("failed to set video mode: {}", err)
    };

    let _ = sdl_image::init(&[sdl_image::InitFlag::PNG]);
    let path = Path::new("examples/handle_error.png");
    let image_surface = sdl_image::load(&path).unwrap();

    let _ = screen.blit(&image_surface);


    // // Note: You'll want to put this and the flip call inside the main loop
    // // but we don't as to not startle epileptics
    // for i in 0usize..10 {
    //	for j in 0usize..10 {
    //	    screen.fill_rect(Some(sdl::Rect {
    //		x: (i as i16) * 800 / 10,
    //		y: (j as i16) * 600 / 10,
    //		w: 800 / 10,
    //		h: 600 / 10
    //	    }), rng.gen::<sdl::video::Color>());
    //	}
    // }

    screen.flip();

    'main : loop {
	'event : loop {
	    match sdl::event::poll_event() {
		Event::Quit => break 'main,
		Event::None => break 'event,
		Event::Key(k, _, _, _)
		    if k == Key::Escape
			=> break 'main,
		_ => {}
	    }
	}
    }

    sdl_image::quit();
    sdl::quit();
}
