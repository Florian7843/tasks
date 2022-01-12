use adw::Window;
use gtk::prelude::*;
use gtk::Application;

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk-rs.example")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Init `gtk::Builder` from file
    let builder = gtk::Builder::from_string(include_str!("ui/window.ui"));

    // Get window and button from `gtk::Builder`
    let window: Window = builder
        .object("window")
        .expect("Could not get object `window` from builder.");

    // Set application
    window.set_application(Some(app));

    window.present();
}
