use crate::graphics::rendering::frame::Frame;
use std::io::Result;

pub trait Renderer {
    fn present(&mut self, frame: &Frame) -> Result<()>;
    fn begin(&mut self) -> Result<()>;
    fn end(&mut self) -> Result<()>;
    fn clear(&mut self) -> Result<()>;
}
