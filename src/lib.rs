pub mod renderer;
pub mod window;

mod traits;
mod types;

pub use window::WindowLike;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
