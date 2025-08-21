use blacksun:: { *, traits::*, shape::* };
use std::io;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

fn main() -> Result<(), io::Error> {
    let (mut main_window, event_loop) = blacksun::Window::new();

    let small = Cell::new(false);

    let object: Rc<dyn Drawable> = Rc::new(button::Button::new(Shape::Rect(rect::Rect::new(
        Vertex { p: [ 200.0, 200.0 ] }, 50, 50,
        color::Color::new(0, 255, 255, 255),
        false,
        5.0
    )), small));

    let label: Rc<dyn Drawable> = Rc::new(label::Label::new(
        Rc::clone(&object),
        color::Color::new(255, 0, 255, 255),
        label::Direction::Inside,
        "Hello!",
        30.0,
        0.0
    ));

    let text: Rc<dyn Drawable> = Rc::new(text::Text::new(
        rect::Rect::new(
            Vertex { p: [ 500.0, 500.0 ] }, 100, 50,
            color::Color::new(0, 255, 255, 255),
            true,
            5.0
        ),
        "Hello",
        color::Color::new(0, 0, 0, 255),
        30.0
    ));

    let text_1: Rc<dyn Drawable> = Rc::new(text::Text::new(
        rect::Rect::new(
            Vertex { p: [ 500.0, 400.0 ] }, 200, 50,
            color::Color::new(0, 255, 255, 255),
            true,
            5.0
        ),
        "Worthless",
        color::Color::new(0, 0, 0, 255),
        30.0
    ));

    main_window.add_to_draw_list(object);
    main_window.add_to_draw_list(label);
    main_window.add_to_draw_list(text);
    main_window.add_to_draw_list(text_1);
    main_window.run_loop(event_loop);

    Ok(())
}
