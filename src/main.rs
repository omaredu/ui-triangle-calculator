extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

struct Triangulo {
    base: f32,
    altura: f32
}

impl Triangulo {
    fn area (&self) -> f32
    {
        return (self.base * self.altura) / 2.0;
    }
}

fn main() {
    if gtk::init().is_err()
    {
        println!("Error initializing GTK.");
        return;
    }

    let ui_string = include_str!("./res/ui.glade");
    let ui_builder = gtk::Builder::from_string(ui_string);

    let window: gtk::Window = ui_builder.get_object("main-window").unwrap();
    let about: gtk::Button = ui_builder.get_object("about-button").unwrap();
    let calculate_button: gtk::Button = ui_builder.get_object("calculate-button").unwrap();
    let about_popover: gtk::PopoverMenu = ui_builder.get_object("info-popover").unwrap();
    let result_label: gtk::Label = ui_builder.get_object("result-label").unwrap();

    //text input
    let altura_input: gtk::Entry = ui_builder.get_object("altura-input").unwrap();
    let base_input: gtk::Entry = ui_builder.get_object("base-input").unwrap();

    about.connect_clicked(move |_| {
        about_popover.show();
    });

    calculate_button.connect_clicked(move |_| {
        let altura: f32 = altura_input.get_text().parse().unwrap();
        let base: f32 = base_input.get_text().parse().unwrap();

        let triangulo = Triangulo {
            base,
            altura
        };

        result_label.set_text(triangulo.area().to_string().as_str());
    });

    window.connect_destroy(|_| {
        std::process::exit(0);
    });

    window.show_all();
    gtk::main();
}
