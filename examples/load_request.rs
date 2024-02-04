use venx::plat::VenxPlat;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevyVenx))
        .add_startup_system(setup)
        .run();
}
fn setup(mut cmd: Commands) {
    cmd.spawn((LoadBevyPlat("assets/demo.plat")));
}
