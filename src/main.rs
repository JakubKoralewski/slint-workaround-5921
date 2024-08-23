slint::include_modules!();
use slint::Model;

fn main() -> Result<(), slint::PlatformError> {
    let ui = Demo::new()?;

    let when_true = ["a", "b", "c"];
    let when_false = ["A", "B", "C"];
    ui.on_switch_toggled({
        let ui = ui.as_weak();
        move |is_switch_checked, old_index| {
            let ui = ui.unwrap();
            let model = ui.get_model();
            let model = model.as_any()
                .downcast_ref::<slint::VecModel<slint::SharedString>>()
                .expect("We know we set a VecModel earlier");
            let current_new_model = if is_switch_checked {
                when_true
            } else {
                when_false
            };
            #[cfg(never)]
            ui.set_model( // this doesn't help
                slint::ModelRc::new(
                    slint::VecModel::from_slice(
                        &current_new_model.map(|s| slint::SharedString::from(s))
                    )
                )
            );
            for index in 0..model.row_count() {
                model.set_row_data(
                    index,
                        slint::SharedString::from(
                        current_new_model[index]
                    )
                )
            }
            ui.set_selected_value(slint::SharedString::from(current_new_model[old_index as usize]));
        }
    });

    ui.run()
}
