use crate::sound::SoundAction;
use crate::graphic::Graphic;
use crate::inputs::Inputs;

pub trait Context {
    fn frame(self: Box<Self>, inputs: Inputs) -> (Box<dyn Context>, Vec<Graphic>, Vec<SoundAction>);
}
