use tauri::{App, Manager, PhysicalSize, Size};

pub fn setup_window(app: &mut App) -> Result<(), &'static str> {
    let window = app.get_window("main").unwrap();

    let always_on_top = cfg!(debug_assertions);
    window.set_always_on_top(!always_on_top).unwrap();

    window.set_resizable(false).unwrap();

    window
        .set_size(Size::Physical(PhysicalSize {
            width: 320,
            height: 200,
        }))
        .unwrap();

    Ok(())
}
