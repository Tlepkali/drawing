# Drawing

This project demonstrates a simple raster graphics engine written in Rust.
It draws random primitives and a small seat map and now exports both a PNG
and SVG image.

## GUI experiments

Several GUI libraries were evaluated for integrating interactive seat map
display:

- **egui/eframe** – immediate mode GUI, easy to set up. Example: `cargo run --example egui_seat_map`.
- **iced** – Elm-inspired retained GUI library. Example: `cargo run --example iced_seat_map`.
- **ggez** – game oriented framework with good drawing primitives. Example: `cargo run --example ggez_seat_map`.
- **pixels** – minimal pixel frame buffer useful for custom renderers. Example:
  `cargo run --example pixels_seat_map`.

The existing drawing code operates on an in-memory image and simple shapes. The
`pixels` crate maps well to this design because it lets us upload our buffer
directly to the screen. `egui` and `iced` require reimplementing drawing using
their own APIs, while `ggez` adds heavier game-oriented dependencies. Therefore,
`pixels` fits best for quickly showing the current seat map output.
