use browser::buffer::PixelBuffer;
use browser::text;
use std::num::NonZeroU32;
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};

include!("winit_app.rs");

fn main() {
    let mut fps_label = text::Label::new("FPS: 0".to_string(), (0.0, 0.0));
    let lorem_label = text::Label::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Non odio euismod lacinia at quis risus sed vulputate odio. Vel pharetra vel turpis nunc. Rhoncus mattis rhoncus urna neque viverra justo. Euismod nisi porta lorem mollis aliquam ut. Quam elementum pulvinar etiam non quam lacus. In nisl nisi scelerisque eu ultrices. Tortor consequat id porta nibh. Dis parturient montes nascetur ridiculus mus mauris vitae. Est ultricies integer quis auctor elit sed vulputate. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna".to_string(), (0.0, 35.0));
    let mut dt = std::time::Instant::now();
    let event_loop = EventLoop::new().unwrap();

    let app = winit_app::WinitAppBuilder::with_init(|elwt| {
        let window = winit_app::make_window(elwt, |w| w);

        let context = softbuffer::Context::new(window.clone()).unwrap();
        let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

        (window, surface)
    })
    .with_event_handler(move |state, event, elwt| {
        let (window, surface) = state;
        elwt.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent {
                window_id,
                event: WindowEvent::RedrawRequested,
            } if window_id == window.id() => {
                if let (Some(width), Some(height)) = {
                    let size = window.inner_size();
                    (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                } {
                    surface.resize(width, height).unwrap();

                    let mut buffer = surface.buffer_mut().unwrap();
                    let mut screen = PixelBuffer::new(&mut buffer, height.into(), width.into());
                    screen.clear(0xFFFFFF);
                    text::draw(&mut screen, &fps_label);
                    text::draw(&mut screen, &lorem_label);
                    let fps: u32 = (1.0 / dt.elapsed().as_secs_f64()) as u32;
                    fps_label.set_text(format!("FPS: {}", fps));
                    buffer.present().unwrap();
                }
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            Event::WindowEvent {
                event:
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                logical_key: Key::Named(NamedKey::Escape),
                                ..
                            },
                        ..
                    },
                window_id,
            } if window_id == window.id() => {
                elwt.exit();
            }
            _ => {}
        }
        dt = std::time::Instant::now();
    });

    winit_app::run_app(event_loop, app);
}
