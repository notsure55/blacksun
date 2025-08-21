use blacksun::*;
use std::io;
use std::rc::Rc;

struct Button {

}

impl Draw for Button {

}

fn main() -> Result<(), io::Error> {
    let (mut main_window, event_loop) = blacksun::Window::new();

    let object: Rc<dyn Draw> = Rc::new(filled_rect::FilledRect::new(
        rect::Rect::new(Vertex { p: [ 200.0, 200.0 ] }, 50, 50),
        color::Color::new(0, 255, 255, 255)
    ));

    let label: Rc<dyn Draw> = Rc::new(label::Label::new(
        Rc::clone(&object),
        color::Color::new(255, 0, 255, 255),
        label::Direction::Top,
        "Hello!",
        30.0,
        0.3
    ));

    main_window.add_to_draw_list(object);
    main_window.add_to_draw_list(label);
    main_window.run_loop(event_loop);

    Ok(())
}
