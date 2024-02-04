use venx::plat::VenxPlat;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevyVenx))
        .add_startup_system(setup)
        .run();
}
fn setup(mut cmd: Commands) {
    cmd.spawn((
        OrConvertFromMcaRequest("saves/25_typed.plat"),
        LoadBevyPlat("saves/25_typed.plat"),
    ));
}
