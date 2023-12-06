use tray_icon::Icon;

const ICON: &[u8] = include_bytes!("../../icon.png");

pub fn load_icon() -> anyhow::Result<Icon> {
    let (icon_rgba, icon_width, icon_height) = {
        let image =
            image::load_from_memory_with_format(ICON, image::ImageFormat::Png)?.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Ok(tray_icon::Icon::from_rgba(
        icon_rgba,
        icon_width,
        icon_height,
    )?)
}
