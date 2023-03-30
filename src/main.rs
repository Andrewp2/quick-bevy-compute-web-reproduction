use bevy::render::RenderPlugin;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::{
    prelude::*,
    render::{
        render_resource::*,
        settings::{Backends, WgpuSettings},
    },
};
use plugin_group::{PostRenderPlugin, PreRenderPlugin};

pub mod plugin_group;

/*
const webgpu_classes = Object.getOwnPropertyNames(window)
    .filter(k => k.startsWith("GPU") && typeof window[k] === 'function')
    .map(k => k);

const is_webgpu_obj = o => o && o.constructor && webgpu_classes.some(webgpu_class => o.constructor.name === webgpu_class);

function dropObject(idx) {
    if (idx < 132) return;
    if (is_webgpu_obj(heap[idx])) return;
    heap[idx] = heap_next;
    heap_next = idx;
}
 */

#[bevy_main]
async fn bevy_main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::BLACK));
    app.add_plugins(PreRenderPlugin);
    let b = app
        .add_plugin_async(RenderPlugin {
            wgpu_settings: WgpuSettings {
                backends: Some(Backends::BROWSER_WEBGPU),
                ..Default::default()
            },
        })
        .await
        .expect("Add render plugin");
    b.add_plugins(PostRenderPlugin);
    b.add_startup_system(setup);
    b.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Circle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
        ..default()
    });

    // Rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
        ..default()
    });

    // Quad
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(50., 100.)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
        transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
        ..default()
    });

    // Hexagon
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Transform::from_translation(Vec3::new(150., 0., 0.)),
        ..default()
    });
}
