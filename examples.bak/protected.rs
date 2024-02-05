use venx::plat::VenxPlat;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevyVenx))
        .add_startup_system(setup)
        .run();
}
fn setup(mut cmd: Commands) {
    cmd.spawn((
        LoadBevyPlat("assets/demo.plat").with_password("super-secure-password-123"),
        Transform::from_vec(vec3(0., 0., 0.)),
    ));
}
