use tao::{
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    platform::windows::EventLoopBuilderExtWindows,
};
use tray_icon::{
    menu::{AboutMetadata, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    Icon, TrayIconBuilder, TrayIconEvent,
};

const ICON: &[u8] = include_bytes!("../../icon.png");

fn load_icon() -> anyhow::Result<Icon> {
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

pub fn create_tray_event_loop(tooltip: Option<String>) -> anyhow::Result<()> {
    // init tray
    let icon = load_icon()?;

    let mut builder: EventLoopBuilder<()> = EventLoopBuilder::<()>::with_user_event();
    #[cfg(target_os = "windows")]
    builder.with_any_thread(true);

    let event_loop: EventLoop<()> = builder.build();

    let tray_menu = Menu::new();

    let quit_i = MenuItem::new("Quit", true, None);
    let _ = tray_menu.append_items(&[
        &PredefinedMenuItem::about(
            None,
            Some(AboutMetadata {
                name: Some("tao".to_string()),
                copyright: Some("Copyright tao".to_string()),
                ..Default::default()
            }),
        ),
        &PredefinedMenuItem::separator(),
        &quit_i,
    ]);

    let mut tray_icon = Some(
        TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip(tooltip.unwrap_or("Default".to_owned()))
            .with_icon(icon)
            .build()
            .unwrap(),
    );

    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();

    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if let Ok(event) = menu_channel.try_recv() {
            if event.id == quit_i.id() {
                tray_icon.take();
                *control_flow = ControlFlow::Exit;
            }
            println!("{event:?}");
        }

        if let Ok(event) = tray_channel.try_recv() {
            println!("{event:?}");
        }
    })
}
