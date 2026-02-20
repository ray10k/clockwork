use std::iter::zip;

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

/// One hand of a clock (per example, the hour hand or the minute hand.)
#[derive(Copy,Clone,Default)]
pub struct Hand {
    ///Length of the hand, expressed as a fraction of the radius of the watch.
    length:f32, 
    ///Width of the hand, expressed in pixels.
    width:u8,
    ///Color of the hand.
    color:Color,
}

impl Hand {
    pub fn new(length:f32,width:u8,color:Color) -> Self {
        assert!(width > 0,"A zero-width hand would be invisible.");
        assert!(color.alpha()> 0.0,"A zero-alpha-color hand would be invisible.");
        Self {
            length:length, width:width,color:color
        }
    }
}

/// The hands of a clock. Will render on a transparent background.
#[derive(Component)]
pub struct Watchface<const H:usize> {
    ///Visual data about how to render the hands of the clock, back-to-front order.
    hands:[Hand;H],
    ///Progress of each hand's turn, as a fraction of a full turn.
    pub turn:[f64;H],
    ///Handle to the image data being updated.
    image:Handle<Image>
}


pub fn make_watchface<const N:usize>(hands:&[Hand;N],size:u32, mut images:ResMut<Assets<Image>>) -> Watchface<N> {
    assert!(size>0,"Tried to make a 0x0 watchface.");
    assert!(hands.len()>0,"Tried to make a watchface without hands.");
    let mut new_hands:[Hand;N] = [default::<Hand>();N];
    for i in 0..N {
        new_hands[i] = hands[i];
    }
    let turns = [0.0f64;N];
    let image_size = Extent3d { width: size, height: size, depth_or_array_layers: 1 };
    let image = Image::new_fill(
        image_size, 
        TextureDimension::D2, 
        &(Srgba::NONE.to_u8_array()), 
        TextureFormat::Rgba8UnormSrgb, 
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD);
    let image_handle = images.add(image);
    
    Watchface::<N>{
        hands: new_hands, 
        turn: turns,
        image: image_handle}
    
}

pub fn watchface_system<const H:usize>(
    watchfaces: Query<&Watchface<H>>,
    mut images : ResMut<Assets<Image>>,
) {
    for current_watch in watchfaces {
        //Step 1: fetch the associated image.
        let watch_image = images.get_mut(&current_watch.image).expect("Could not fetch watch image.");
        //Step 2: clear the image.
        watch_image.clear(&(Srgba::NONE.to_u8_array()));
        //Step 3: draw the hands, back-to-front.
        for (hand,turn) in zip(current_watch.hands.iter(),current_watch.turn.iter()) {

        }
    }
}