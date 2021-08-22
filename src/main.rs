use glium::{glutin, Surface};
use std::{error::Error, time};

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop)?;

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = time::Instant::now() + time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }
                _ => {}
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => {}
            },
            _ => {}
        }

        let mut target = display.draw();
        target.clear_color(1.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();
    })
}
