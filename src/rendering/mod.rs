extern crate winit;
extern crate image;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use image::DynamicImage;
use std::cell::RefCell;
use std::rc::Rc;

pub mod terminal;

pub fn run(terminal: Rc<RefCell<terminal::Terminal>>) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    terminal.borrow_mut().load_characters(); // Load characters from the font image.

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }

        // Get an iterator over the raw grid data from the terminal.
        let binding = terminal.borrow();
        let grid_iter = binding.get_raw_grid();

        // Convert the grid data to an image or OpenGL vertices and render it to the window.
        // You'll need to implement this part based on your rendering library.

        // For example, you might use a library like `wgpu` or `glow` to render the grid.
        // Here, we'll print the ASCII characters to the console for simplicity.
        for row in grid_iter {
            println!("{}", row.iter().collect::<String>());
        }
    });
}
