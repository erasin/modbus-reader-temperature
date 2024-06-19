pub mod relay;
pub mod temperature;
pub mod voltage;

pub trait Mock {
    fn request(&self) -> Vec<u8>;
    fn response(&self) -> Vec<u8>;
    // fn from_request(request: &[u8]) -> Self;
}
