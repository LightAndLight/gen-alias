mod gen;

use std::rc::Rc;

use gen::gen_email;
use gtk4::{gdk, gdk::pango, gio, glib, prelude::*};

const APP_ID: &str = "io.ielliott.gen-alias";

struct Env {
    host: String,
}

fn read_env() -> Env {
    let host = std::env::var("GEN_ALIAS_HOST").unwrap_or_else(|err| {
        match err {
            std::env::VarError::NotPresent => {
                eprintln!("error: GEN_ALIAS_HOST not set");
            }
            std::env::VarError::NotUnicode(_) => {
                eprintln!("error: GEN_ALIAS_HOST is not unicode");
            }
        }
        std::process::exit(1)
    });
    Env { host }
}

fn main() -> glib::ExitCode {
    let env = Rc::new(read_env());
    let app = gtk4::Application::builder().application_id(APP_ID).build();
    app.connect_activate(glib::clone!(
        #[weak]
        env,
        move |app| on_activate(env, app)
    ));
    app.set_accels_for_action("win.close", &["Escape"]);
    app.run()
}

fn on_activate(env: Rc<Env>, app: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::builder()
        .application(app)
        .title("gen-alias")
        .build();

    // https://github.com/gtk-rs/gtk/issues/949#issuecomment-581618386
    let font_size_pixels = match window.pango_context().font_description() {
        None => 16,
        Some(font_description) => font_description.size() / pango::SCALE,
    };

    let inputs_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .vexpand(false)
        .build();

    let input_entry_buffer = gtk4::EntryBuffer::new(None::<&str>);
    let input_entry = gtk4::Entry::builder()
        .buffer(&input_entry_buffer)
        .max_width_chars(16)
        .build();
    inputs_box.append(&input_entry);
    inputs_box.append(&gtk4::Label::new(Some(".XXXXXX")));
    inputs_box.append(&gtk4::Label::new(Some("@")));
    inputs_box.append(&gtk4::Label::new(Some(&env.host)));

    let generate_button = gtk4::Button::builder()
        .label("Generate & Copy")
        .margin_start(font_size_pixels)
        .build();
    inputs_box.append(&generate_button);

    let output_label = gtk4::Label::builder()
        .halign(gtk4::Align::Start)
        .selectable(true)
        .build();

    let outputs_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .vexpand(false)
        .build();
    outputs_box.append(&output_label);

    let main_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .vexpand(false)
        .valign(gtk4::Align::Center)
        .halign(gtk4::Align::Center)
        .build();
    main_box.append(&inputs_box);
    main_box.append(&outputs_box);

    window.set_child(Some(&main_box));
    window.add_action_entries([gio::ActionEntryBuilder::new("close")
        .activate(|window: &gtk4::ApplicationWindow, _, _| {
            window.close();
        })
        .build()]);

    input_entry.connect_activate(glib::clone!(
        #[weak]
        env,
        #[weak]
        input_entry_buffer,
        #[weak]
        output_label,
        move |_this| action_generate_and_copy(env, input_entry_buffer, output_label)
    ));
    generate_button.connect_clicked(glib::clone!(
        #[weak]
        env,
        #[weak]
        input_entry_buffer,
        #[weak]
        output_label,
        move |_this| action_generate_and_copy(env, input_entry_buffer, output_label)
    ));

    window.present();
}

fn action_generate_and_copy(
    env: Rc<Env>,
    input_entry_buffer: gtk4::EntryBuffer,
    output_label: gtk4::Label,
) {
    let mut output = String::new();
    gen_email(&mut output, &input_entry_buffer.text(), &env.host);
    output_label.set_label(&output);

    match gdk::Display::default() {
        None => {
            eprintln!("panic: no display found");
            std::process::exit(1)
        }
        Some(display) => {
            let clipboard = display.clipboard();
            clipboard.set_text(&output);
        }
    }
}
