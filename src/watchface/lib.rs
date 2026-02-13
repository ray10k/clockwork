use bevy::prelude::*;

/// One hand of a clock (per example, the hour hand or the minute hand.)
#[derive(Copy,Clone,Default)]
pub struct Hand {
    ///Length of the hand, expressed as a fraction of the radius of the watch.
    length:f64, 
    ///Color of the hand.
    color:Color,
}

/// The hands of a clock. Will render on a transparent background.
#[derive(Component)]
pub struct Watchface<const H:usize> {
    ///Visual data about how to render the hands of the clock, front-to-back order.
    hands:[Hand;H],
    ///Progress of each hand's turn, as a fraction of a full turn.
    pub turn:[f64;H]
}


pub fn make_watchface<const N:usize>(hands:&[Hand;N],size:f32) -> (Watchface<N>,Sprite) {
    assert!(size>0.0,"Tried to make a 0x0 watchface.");
    assert!(hands.len()>0,"Tried to make a watchface without hands.");
    let mut new_hands:[Hand;N] = [default::<Hand>();N];
    for i in 0..N {
        new_hands[i] = hands[i];
    }
    let turns = [0.0f64;N];
    ( 
    Watchface{
        hands: new_hands, 
        turn: turns}, 
    Sprite::from_color(Color::NONE, Vec2::new(1.0+size*2.0,1.0+size*2.0))
    )
}

pub fn watchface_system<const H:usize>(mut query: Query<&mut Sprite, With<Watchface<H>>>) {
    for mut sprite in query {
        
    }
}