# win-overlay (-rs)
DirectX overlay written in Rust for various projects, wanted to easily create overlays across projects so decided to write my own implementation in Rust.

# dependencies
* winapi
* ~~lazy_static~~

# usage
```rust

//....
use win_overlay::Overlay;

pub fn main() {

    let overlay = Overlay::create_overlay(/**/);

    // Note: this enters a loop.
    overlay.draw(&|| {
        overlay.draw_filled_box(0, 0, 100, 100, D3DCOLOR_RGBA(255,0,0,255));    
    });

}
```