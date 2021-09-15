use bevy::{
    prelude::{App, AssetServer, Commands, IntoSystem, Res},
    render2::camera::OrthographicCameraBundle,
    sprite2::PipelinedSpriteBundle,
    PipelinedDefaultPlugins,
};

fn main() {
    App::new()
        .add_plugins(PipelinedDefaultPlugins)
        .add_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture = asset_server.load("esbardu_solu.png");
    commands.spawn_bundle(PipelinedSpriteBundle {
        texture,
        ..Default::default()
    });
}
