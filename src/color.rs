use num;

pub trait Color: Clone + PartialEq {
    type Tag;
    type ChannelsTuple;

    fn num_channels() -> u32;
    fn from_tuple(values: Self::ChannelsTuple) -> Self;
    fn to_tuple(self) -> Self::ChannelsTuple;
}

pub trait PolarColor: Color {
    type Angular;
    type Cartesian;
}

pub trait HomogeneousColor: Color {
    type ChannelFormat;

    fn from_slice(values: &[Self::ChannelFormat]) -> Self;
    fn as_slice(&self) -> &[Self::ChannelFormat];
    fn broadcast(value: Self::ChannelFormat) -> Self;
    fn clamp(self, min: Self::ChannelFormat, max: Self::ChannelFormat) -> Self;
}

pub trait Color3: Color {}
pub trait Color4: Color {}

pub trait Lerp {
    type Position: num::Float;
    fn lerp(&self, right: &Self, pos: Self::Position) -> Self;
}

pub trait Invert {
    fn invert(self) -> Self;
}

pub trait Bounded {
    fn normalize(self) -> Self;
    fn is_normalized(&self) -> bool;
}

/*pub fn color_cast<To, From>(from: &From) -> To 
        where From: Color + Color3,
              To: Color<Tag=From::Tag> + Color3,
              To::Component: num::NumCast,
              From::Component: num::NumCast,
{

    let to_scale = To::Component::max() - To::Component::min();
    let from_scale = From::Component::max() - From::Component::min();
   
    let shift = cast::<_,f64>(To::Component::min()).unwrap() 
        - cast::<_,f64>(From::Component::min()).unwrap();
    let factor: f64 = cast::<_,f64>(to_scale).unwrap() 
        / cast::<_,f64>(from_scale).unwrap();

    let mut out = [To::Component::default(); 3];
    let vals = from.as_slice();

    for i in 0..3 {
        out[i] = cast::<_, To::Component>(
            cast::<_,f64>(vals[i]).unwrap()*factor + shift).unwrap();
    }

    To::from_slice(&out)
}*/
