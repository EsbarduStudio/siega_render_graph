use bevy::{
    app::{Plugin, PluginGroup, PluginGroupBuilder},
    asset::AddAsset,
    prelude::{App, AssetServer, Commands, IntoSystem, Res},
    render2::{
        camera::OrthographicCameraBundle, render_graph::RenderGraph, render_phase::DrawFunctions,
        RenderApp, RenderStage,
    },
    sprite2::*,
};

pub struct SiegaGraphPlugin;

impl Plugin for SiegaGraphPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<TextureAtlas>().register_type::<Sprite>();
        let render_app = app.sub_app(RenderApp);
        render_app
            .init_resource::<ExtractedSprites>()
            .add_system_to_stage(RenderStage::Extract, extract_atlases)
            .add_system_to_stage(RenderStage::Extract, extract_sprites)
            .add_system_to_stage(RenderStage::Prepare, prepare_sprites)
            .add_system_to_stage(RenderStage::Queue, queue_sprites)
            .init_resource::<SpriteShaders>()
            .init_resource::<SpriteMeta>();
        let draw_sprite = DrawSprite::new(&mut render_app.world);
        render_app
            .world
            .get_resource::<DrawFunctions>()
            .unwrap()
            .write()
            .add(draw_sprite);
        let mut graph = render_app.world.get_resource_mut::<RenderGraph>().unwrap();
        graph.add_node("sprite", SpriteNode);
        graph
            .add_node_edge("sprite", bevy::core_pipeline::node::MAIN_PASS_DEPENDENCIES)
            .unwrap();
    }
}

pub struct SiegaDefaultPlugins;

impl PluginGroup for SiegaDefaultPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(bevy::log::LogPlugin::default());
        group.add(bevy::core::CorePlugin::default());
        group.add(bevy::transform::TransformPlugin::default());
        group.add(bevy::diagnostic::DiagnosticsPlugin::default());
        group.add(bevy::input::InputPlugin::default());
        group.add(bevy::window::WindowPlugin::default());
        group.add(bevy::asset::AssetPlugin::default());
        group.add(bevy::scene::ScenePlugin::default());

        group.add(bevy::render2::RenderPlugin::default());
        group.add(bevy::core_pipeline::CorePipelinePlugin::default());
        group.add(bevy::pbr2::PbrPlugin::default());
        group.add(bevy::gltf2::GltfPlugin::default());
        group.add(bevy::winit::WinitPlugin::default());

        group.add(SiegaGraphPlugin);
    }
}

fn main() {
    App::new()
        .add_plugins(SiegaDefaultPlugins)
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
