use super::MP_SENDER;

pub fn get_position(alignment: (i8, i8)) -> (i32, i32) {
    let (width, height) = crate::utils::get_screen_size();
    if width == -1 && height == -1 {
        return (0, 0);
    }

    match alignment {
        (-1, -1) => (0, 0),
        (0, -1) => ((0.5 * width as f32) as i32- /*default width*/ 100, 0),

        (1, -1) => (width- /*default width*/ 100, 0),
        (-1, 0) => (0, (0.5 * height as f32) as i32 - /*default height*/ 50),
        (0, 0) => (
            (0.5 * width as f32) as i32 - /*default width*/ 100,
            (0.5 * height as f32) as i32 - /*default height*/ 50,
        ),
        (1, 0) => (
            width- /*default width*/ 100,
            (0.5 * height as f32) as i32 - /*default height*/ 50,
        ),
        (-1, 1) => (0, height- /*default height*/ 100),
        (0, 1) => (
            (0.5 * width as f32) as i32- /*default width*/ 100,
            height- /*default height*/ 100,
        ),
        (1, 1) => (width- /*default width*/ 200, height- /*default height*/ 100),

        _ => (0, 0),
    }
}

pub fn send_dart_message(message: String) -> anyhow::Result<()> {
    let r = MP_SENDER.lock().unwrap();
    (*r).clone().unwrap().send(message)?;
    anyhow::Ok(())
}
