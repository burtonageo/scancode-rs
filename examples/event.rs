use glutin::{
    event::{ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder,
};
use scancode::Scancode;
use std::{collections::BTreeMap, error::Error, mem};

fn main() -> Result<(), Box<(dyn Error + Send + Sync + 'static)>> {
    let mut errors = BTreeMap::new();

    let event_loop = EventLoop::new();

    let window = unsafe {
        let wb = WindowBuilder::new().with_title("Scancode Example");

        ContextBuilder::new()
            .build_windowed(wb, &event_loop)?
            .make_current()
            .map_err(|e| e.1)?
    };

    let window_id = window.window().id();

    event_loop.run(move |event, _window_target, control_flow| match event {
        Event::WindowEvent {
            event,
            window_id: id,
        } if window_id == id => match event {
            WindowEvent::KeyboardInput { input, .. } => {
                let KeyboardInput {
                    scancode,
                    state,
                    virtual_keycode: vk,
                    ..
                } = input;

                if state == ElementState::Pressed {
                    print!("pressed")
                } else {
                    print!("released")
                }

                if let Some(code) = Scancode::new(scancode) {
                    print!(" {} -> {:?}", scancode, code);
                } else {
                    eprintln!("*** ERROR: UNKNOWN SCANCODE ***");
                    errors.insert(scancode, format!("{:?}", vk));
                }

                if let Some(vk) = vk {
                    println!(" (virtual keycode {:?})", vk);
                } else {
                    println!(" (virtual keycode UNKNOWN)");
                }
            }
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => (),
        },
        Event::RedrawRequested(id) if window_id == id => {
            match window.swap_buffers() {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("An error occurred while swapping buffers: {}", e);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            let frame_errors = mem::take(&mut errors);
            if !frame_errors.is_empty() {
                println!("Unhandled scancodes:");

                for (k, v) in frame_errors.into_iter() {
                    println!("{}: {}", k, v);
                }
            }
        }
        _ => (),
    })
}
