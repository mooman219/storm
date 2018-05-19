use storm::input::message::*;

//the controller trait is meant as a interface for the common functions that each controller has to have
//examples - Input Handler
// Each contorller must have a way of handling the current input events
pub trait Controller {
    fn input_handler(&mut self, input: InputFrame);
}