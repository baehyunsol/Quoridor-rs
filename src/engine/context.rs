use super::sound::SoundAction;
use super::graphic::Graphic;
use super::inputs::Inputs;

pub trait Context {
    fn frame(self: Box<Self>, inputs: Inputs) -> (Box<dyn Context>, Vec<Graphic>, Vec<SoundAction>);
}
