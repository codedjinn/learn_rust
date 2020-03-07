
mod app;

extern crate glium;

fn main() {

    #[allow(unused_improts)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let windw_builder = glutin::window::WindowBuilder::new();
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(windw_builder, context_builder, &event_loop).unwrap();

    let app = app:App::new();

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
                    std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => {},
                _ => return,
            },
            _ => return
        }

        app.update(&next_frame_time);
        
        let mut target = display.draw();
        
        target.clear_color(0.39, 0.58, 0.92, 1.0);

        app.draw(&next_frame_time, &target);

        target.finish().unwrap();
    });

}