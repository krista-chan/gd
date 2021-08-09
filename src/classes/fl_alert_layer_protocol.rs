use super::fl_alert_layer::FLAlertLayer;

pub unsafe trait FLAlertLayerProtocol {
    extern "thiscall" fn __fl_alert_clicked(
        this: *const dyn FLAlertLayerProtocol,
        alert_layer: *const FLAlertLayer,
        second_button: bool,
    ) where
        Self: Sized;
}
