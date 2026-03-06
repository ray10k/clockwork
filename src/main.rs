use watchface::*;
use bevy::prelude::*;

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_hz(100.0))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, tick)
        .add_systems(FixedUpdate,watchface_system::<3>)
        .run();
}

fn setup(mut commands: Commands, images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2d);
    let hands = [
        Hand::new(0.9, 1, Color::BLACK),
        Hand::new(0.7,1,Color::srgb(1.0, 0.0, 0.0)),
        Hand::new(0.3,1,Color::srgb(0.0, 1.0, 0.0))];
    let watch = make_watchface::<3>(&hands,128,images);
    let watch_img = watch.image.clone();
    commands.spawn((watch,Transform::from_xyz(10.0, 10.0, 0.0),Sprite::from_image(watch_img)));
}

fn tick(mut commands: Commands) {

}