// Overlay window for click-through logo on Wayland
// This is a minimal winit window that displays the logo and is input-transparent

use std::path::Path;
use std::sync::Arc;
use std::thread;
use image::GenericImageView;
use winit;

use winit;

#[cfg(target_os = "linux")]
pub fn spawn_overlay_window(logo_path: &Path, position: &str, margin: u32, max_logo_percent: f32, opacity: f32) {
    let logo_path = logo_path.to_owned();
    let position = position.to_owned();
    thread::spawn(move || {
        // winit imports are now at the top of the file
        // use winit::platform::unix::WindowExtUnix;
        // use std::ptr;
        // use wayland_sys::client::wl_surface;

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Desktop Logo Overlay")
            .with_transparent(true)
            .with_decorations(false)
            .build(&event_loop)
            .expect("Failed to create overlay window");

        // Set click-through (input-transparent) on Wayland
        // Click-through (input-transparent) logic for Wayland is not available in wayland-sys 0.31 public API.
        // This section is commented out. For true click-through, use a higher-level crate or compositor-specific protocol.
        // if let Some(raw_wl_surface) = window.wayland_surface() {
        //     unsafe {
        //         let wl_surface_ptr = raw_wl_surface as *mut wl_surface;
        //         wayland_sys::client::wl_surface_set_input_region(wl_surface_ptr, ptr::null_mut());
        //         wayland_sys::client::wl_surface_commit(wl_surface_ptr);
        //     }
        // }

        // Load logo image
        let img = match image::open(&logo_path) {
            Ok(img) => img,
            Err(_) => return,
        };
        let (w, h) = img.dimensions();
        let reference = w.min(h) as f32;
        let target = (reference * max_logo_percent).max(1.0);
        let scale = target / reference;
        let scaled_w = (w as f32 * scale).round().max(1.0) as u32;
        let scaled_h = (h as f32 * scale).round().max(1.0) as u32;
        let img = img.resize_exact(scaled_w, scaled_h, image::imageops::FilterType::Lanczos3);
        let rgba = img.to_rgba8();
        let raw = Arc::new(rgba.into_raw());

        // Use pixels crate for simple drawing
        let mut pixels = {
            let size = window.inner_size();
            pixels::Pixels::new(size.width, size.height, &window).expect("pixels init")
        };

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::RedrawRequested(_) => {
                    let frame = pixels.frame();
                    // Clear to transparent
                    for px in frame.chunks_exact_mut(4) {
                        px.copy_from_slice(&[0, 0, 0, 0]);
                    }
                    // Draw logo at position
                    let size = window.inner_size();
                    let (win_w, win_h) = (size.width, size.height);
                    let (x, y) = match position.as_str() {
                        "topleft" => (margin as u32, margin as u32),
                        "topright" => (win_w.saturating_sub(scaled_w + margin), margin as u32),
                        "bottomleft" => (margin as u32, win_h.saturating_sub(scaled_h + margin)),
                        _ => (win_w.saturating_sub(scaled_w + margin), win_h.saturating_sub(scaled_h + margin)),
                    };
                    for row in 0..scaled_h {
                        for col in 0..scaled_w {
                            let src_idx = ((row * scaled_w + col) * 4) as usize;
                            let dst_x = x + col;
                            let dst_y = y + row;
                            if dst_x < win_w && dst_y < win_h {
                                let dst_idx = ((dst_y * win_w + dst_x) * 4) as usize;
                                let alpha = (raw[src_idx + 3] as f32 * opacity) as u8;
                                frame[dst_idx..dst_idx + 4].copy_from_slice(&[
                                    raw[src_idx], raw[src_idx + 1], raw[src_idx + 2], alpha
                                ]);
                            }
                        }
                    }
                    pixels.render().unwrap();
                }
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::WindowEvent { event: WindowEvent::Resized(_), .. } => {
                    window.request_redraw();
                }
                _ => {}
            }
        });
    });
}

#[cfg(not(target_os = "linux"))]
pub fn spawn_overlay_window(_logo_path: &Path, _position: &str, _margin: u32, _max_logo_percent: f32, _opacity: f32) {
    // No-op on non-Linux
}
