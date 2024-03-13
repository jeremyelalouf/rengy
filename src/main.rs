use winit::{
    event::*, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::WindowBuilder
};

fn run() {
    let event_loop = EventLoop::new();

    match event_loop {
        Ok(event_loop) => {
            let window = WindowBuilder::new().build(&event_loop).unwrap();

            let _ = event_loop.run(move |event, event_loop_target| match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: PhysicalKey::Code(KeyCode::Escape),
                                ..
                            },
                        ..
                    } => event_loop_target.exit(),
                    _ => {}
                },
                _ => {}
            });
        },
        _ => println!("Event loop not created"),
    }
}

fn main() {
    run();
}
