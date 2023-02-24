#[cfg(feature = "actix-web")]
mod web;

#[cfg(feature = "gtk")]
mod gui;

fn main() {
    #[cfg(feature = "actix-web")]
    web::main();

    #[cfg(feature = "gtk")]
    gui::main();
}
