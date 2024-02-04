use venx::plat::VenxPlat;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevyVenx))
        .add_startup_system(setup)
        .run();
}
fn setup(mut cmd: Commands) {
    // Main plat
    cmd.spawn((
        LoadBevyPlat("assets/demo.plat"),
        Transform::from_vec(vec3(0., 0., 0.)),
    ));
    // Floating island
    cmd.spawn((
        LoadBevyPlat("assets/island.plat"),
        Transform::from_vec(vec3(0., 0., 0.)),
    ));
}