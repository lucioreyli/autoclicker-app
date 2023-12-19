use tauri::{App, Manager, PhysicalSize, Size};

pub fn setup_window(app: &mut App) {
    let window = app.get_window("main").unwrap();

    let is_development = cfg!(debug_assertions);
    window.set_always_on_top(!is_development).unwrap();

    window.set_resizable(false).unwrap();

    window
        .set_size(Size::Physical(PhysicalSize {
            width: 320,
            height: 200,
        }))
        .unwrap();
}
