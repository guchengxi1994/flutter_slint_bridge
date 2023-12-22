mod tests {
    use slint::ComponentHandle;
    #[test]
    fn test_tip() -> anyhow::Result<()> {
        let tip = crate::ui::Tip::new()?;
        let tip_handle = tip.as_weak();

        tip_handle
            .upgrade()
            .unwrap()
            .window()
            .set_position(slint::LogicalPosition::new(0., 0.));

        tip_handle.upgrade().unwrap().on_mouse_move({
            let tip_handle_clone = tip_handle.unwrap();
            move |delta_x, delta_y| {
                let logical_pos = tip_handle_clone
                    .window()
                    .position()
                    .to_logical(tip_handle_clone.window().scale_factor());
                tip_handle_clone
                    .window()
                    .set_position(slint::LogicalPosition::new(
                        logical_pos.x + delta_x,
                        logical_pos.y + delta_y,
                    ));
            }
        });

        tip_handle.upgrade().unwrap().on_close_window({
            let pin_win_clone = tip_handle.unwrap();
            move || {
                let _ = pin_win_clone.hide();
            }
        });

        tip.run()?;

        anyhow::Ok(())
    }
}
