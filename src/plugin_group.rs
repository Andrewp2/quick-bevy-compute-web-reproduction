use bevy::{
    a11y::AccessibilityPlugin,
    app::PluginGroupBuilder,
    audio::AudioPlugin,
    core_pipeline::CorePipelinePlugin,
    diagnostic::DiagnosticsPlugin,
    gltf::GltfPlugin,
    input::InputPlugin,
    log::{Level, LogPlugin},
    pbr::PbrPlugin,
    prelude::{
        AnimationPlugin, AssetPlugin, FrameCountPlugin, GilrsPlugin, HierarchyPlugin, ImagePlugin,
        PluginGroup, TaskPoolPlugin, TypeRegistrationPlugin,
    },
    scene::ScenePlugin,
    sprite::SpritePlugin,
    text::TextPlugin,
    time::TimePlugin,
    transform::TransformPlugin,
    ui::UiPlugin,
    window::{PresentMode, Window, WindowMode, WindowPlugin, WindowResolution},
    winit::WinitPlugin,
};

pub struct PreRenderPlugin;

impl PluginGroup for PreRenderPlugin {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();

        builder = builder.add(LogPlugin {
            level: Level::WARN,
            filter: "wgpu=WARN,bevy_render=WARN,bevy_ecs=WARN".into(),
        });

        builder = builder
            .add(TaskPoolPlugin::default())
            .add(TypeRegistrationPlugin::default())
            .add(FrameCountPlugin::default())
            .add(TimePlugin::default())
            .add(TransformPlugin::default())
            .add(HierarchyPlugin::default())
            .add(DiagnosticsPlugin::default())
            .add(InputPlugin::default())
            .add(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    canvas: Some("#mycanvas".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .add(AccessibilityPlugin)
            .add(AssetPlugin {
                watch_for_changes: true,
                ..Default::default()
            })
            .add(ScenePlugin::default())
            .add(WinitPlugin::default());
        builder
    }
}
pub struct PostRenderPlugin;

impl PluginGroup for PostRenderPlugin {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();
        builder = builder.add(ImagePlugin::default_nearest());
        // NOTE: Load ImagePlugin after renderer initialization so that it knows about the supported
        // compressed texture formats
        #[cfg(not(target_arch = "wasm32"))]
        {
            //builder = builder.add(PipelinedRenderingPlugin::default());
        }

        // default = ["animation", "bevy_asset", "bevy_audio", "bevy_gilrs", "bevy_scene", "bevy_winit", "bevy_core_pipeline", "bevy_pbr",
        // "bevy_gltf", "bevy_render", "bevy_sprite", "bevy_text", "bevy_ui", "png", "hdr", "ktx2", "zstd", "vorbis", "x11", "filesystem_watcher",
        // "android_shared_stdcxx", "tonemapping_luts"]

        builder = builder
            .add(CorePipelinePlugin::default())
            .add(SpritePlugin::default())
            .add(TextPlugin::default())
            .add(UiPlugin::default())
            .add(PbrPlugin::default())
            .add(GltfPlugin::default());
        //.add(AudioPlugin::default())
        //.add(GilrsPlugin::default())
        //.add(AnimationPlugin::default());
        builder
    }
}
