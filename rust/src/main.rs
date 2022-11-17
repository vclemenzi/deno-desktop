use gtk::{prelude::*, Box, Orientation, Window, WindowType};
use webkit2gtk::{SettingsExt, WebContext, WebView, WebViewExt};
use std::fs;

fn main() {
    gtk::init().unwrap();

    let vbox = Box::new(Orientation::Vertical, 0);

    let webview = WebView::with_context(&WebContext::default().unwrap());

    let html = fs::read_to_string("../.out/index.html").expect("Should have been able to read the file");
    webview.load_html(&html, None);

    let settings = WebViewExt::settings(&webview).unwrap();
    settings.set_enable_developer_extras(true);

    vbox.pack_start(&webview, true, true, 0);

    let window = Window::new(WindowType::Toplevel);

    window.set_child(Some(&vbox));

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let config = fs::read_to_string("../app.config");

    let binding = config.unwrap();
    let lines = binding.lines();

    let mut window_title = "My App";
    let mut window_width = 800;
    let mut window_height = 600;

    // title = "My App" *Example*

    for line in lines {
        let mut parts = line.split(" = ");

        let key = parts.next().unwrap();
        let value = parts.next().unwrap();

        if key == "title" {
            window_title = value;
        } else if key == "width" {
            window_width = value.parse::<i32>().unwrap();
        } else if key == "height" {
            window_height = value.parse::<i32>().unwrap();
        }
    }

    window.set_size_request(window_width, window_height);
    window.set_title(window_title);

    gtk::main();
}
