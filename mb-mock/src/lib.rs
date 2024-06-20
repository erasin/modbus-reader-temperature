use mb::protocol::{FunRequest, FunResponse};

pub mod relay;
pub mod temperature;
pub mod voltage;

pub trait Mock {
    fn request(&self) -> FunRequest;
    fn response(&self) -> FunResponse;
}
