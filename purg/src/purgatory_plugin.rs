use bevy::render::render_graph::base::BaseRenderGraphConfig;
use bevy_app::{PluginGroup, PluginGroupBuilder};

pub struct PurgatoryPlugins;

impl PluginGroup for PurgatoryPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(bevy_log::LogPlugin::default());
        group.add(bevy_core::CorePlugin::default());
        group.add(bevy_transform::TransformPlugin::default());
        group.add(bevy_diagnostic::DiagnosticsPlugin::default());
        group.add(bevy_input::InputPlugin::default());
        group.add(bevy_window::WindowPlugin::default());
        group.add(bevy_asset::AssetPlugin::default());
        group.add(bevy_scene::ScenePlugin::default());

        group.add(bevy_render::RenderPlugin{
            base_render_graph_config: Some(BaseRenderGraphConfig{
                add_2d_camera: true,
                add_3d_camera: true,
                add_main_depth_texture: true,
                add_main_pass: true,
                connect_main_pass_to_swapchain: true,
                connect_main_pass_to_main_depth_texture: true,
            })
        });

        #[cfg(feature = "bevy_sprite")]
        group.add(bevy_sprite::SpritePlugin::default());

        group.add(bevy_pbr::PbrPlugin::default());

        #[cfg(feature = "bevy_ui")]
        group.add(bevy_ui::UiPlugin::default());

        #[cfg(feature = "bevy_text")]
        group.add(bevy_text::TextPlugin::default());

        #[cfg(feature = "bevy_audio")]
        group.add(bevy_audio::AudioPlugin::default());

        group.add(bevy_gilrs::GilrsPlugin::default());

        #[cfg(feature = "bevy_gltf")]
        group.add(bevy_gltf::GltfPlugin::default());

        group.add(bevy_winit::WinitPlugin::default());

        group.add(bevy_wgpu::WgpuPlugin::default());
    }
}

pub struct PurgatoryMinimalPlugins;

impl PluginGroup for PurgatoryMinimalPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(bevy_core::CorePlugin::default());
        group.add(bevy_app::ScheduleRunnerPlugin::default());
    }
}
