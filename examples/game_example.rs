use blacksun::*;
use std::io;
use std::rc::Rc;

fn main() -> Result<(), io::Error> {
    let (mut main_window, event_loop) = blacksun::Window::new();
    let object = Rc::new(Object::FilledRect(filled_rect::FilledRect::new(
        rect::Rect::new(Vertex { p: [ 200.0, 200.0 ] }, 50, 50 ),
        color::Color::new(0, 255, 255, 255)
    )));
    let label = Rc::new(Object::Label(label::Label::new(
        Rc::clone(&object),
        color::Color::new(255, 0, 255, 255),
        label::Direction::Top,
        "Hello!",
        30.0,
        0.5
    )));

    main_window.add_to_draw_list(object);
    main_window.add_to_draw_list(label);

    #[allow(deprecated)]
    event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                // This event is sent by the OS when you close the Window, or request the program to quit via the taskbar.
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                glium::winit::event::WindowEvent::CursorMoved { position, .. } => {
                },
                glium::winit::event::WindowEvent::RedrawRequested => {
                    main_window.draw();
                    main_window.window.request_redraw()
                },
                _ => (),
            },
            _ => (),
        };
    })
    .unwrap();

    Ok(())
}
