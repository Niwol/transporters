use bevy::app::App;
use transporters::TransporterGamePlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(TransporterGamePlugin);
    app.run();
}
