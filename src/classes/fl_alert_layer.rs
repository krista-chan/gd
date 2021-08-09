use std::{ffi::CString, marker::PhantomData, os::raw::c_char};

use super::fl_alert_layer_protocol::FLAlertLayerProtocol;
use getfn::symbol_fn;
use rcocos2d_sys::{
    cocos2d_CCEvent, cocos2d_CCLayer, cocos2d_CCMenu, cocos2d_CCObject, cocos2d_CCTouch,
    cocos2d_ccColor3B,
};
use struct_pad::{PadU32, PadU64};

#[repr(C)]
pub struct FLAlertLayer {
    button_menu: *const cocos2d_CCMenu,
    controller_connected: i32,
    target: *const cocos2d_CCObject,
    __pad1: PadU64,
    layer: *const cocos2d_CCLayer,
    z_order_2: i32,
    button_no_elasticity: bool,
    c_color_2: *const cocos2d_ccColor3B,
    button1: *const PhantomData<u8>,         // ButtonSprite
    button2: *const PhantomData<u8>,         // ButtonSprite
    scrolling_layer: *const PhantomData<u8>, // ScrollingLayer
    __pad2: PadU32,                          // pad m_nJoystickConnected
    button_border: Option<bool>,
    button_no_action: bool,
}

impl FLAlertLayer {
    symbol_fn! {
        ("GeometryDash.exe" + 0x23750) pub unsafe extern "thiscall" fn Self::__on_enter(this: *const FLAlertLayer);
        ("GeometryDash.exe" + 0x236F0) pub unsafe extern "thiscall" fn Self::__register_with_touch_dispatcher(this: *const FLAlertLayer);
        ("GeometryDash.exe" + 0x233C0) pub unsafe extern "thiscall" fn Self::__cc_touch_began(this: *const FLAlertLayer, touch: *const cocos2d_CCTouch, event: *const cocos2d_CCEvent) -> bool;
        ("GeometryDash.exe" + 0x23510) pub unsafe extern "thiscall" fn Self::__cc_touch_moved(this: *const FLAlertLayer, touch: *const cocos2d_CCTouch, event: *const cocos2d_CCEvent);
        ("GeometryDash.exe" + 0x23450) pub unsafe extern "thiscall" fn Self::__cc_touch_ended(this: *const FLAlertLayer, touch: *const cocos2d_CCTouch, event: *const cocos2d_CCEvent);
        ("GeometryDash.exe" + 0x234C0) pub unsafe extern "thiscall" fn Self::__cc_touch_cancelled(this: *const FLAlertLayer, touch: *const cocos2d_CCTouch, event: *const cocos2d_CCEvent);
        ("GeometryDash.exe" + 0x23750) pub unsafe extern "thiscall" fn Self::__key_down(this: *const FLAlertLayer, key: u16);
        ("GeometryDash.exe" + 0x232C0) pub unsafe extern "thiscall" fn Self::__key_back_clicked(this: *const FLAlertLayer);
        ("GeometryDash.exe" + 0x23560) pub unsafe extern "thiscall" fn Self::__show(this: *const FLAlertLayer);
        ("GeometryDash.exe" + 0x224B0) pub unsafe extern "thiscall" fn Self::__FLAlertLayer() -> *const FLAlertLayer;
        ("GeometryDash.exe" + 0x225c0) pub unsafe extern "thiscall" fn Self::__destructor(this: *const FLAlertLayer);
        ("GeometryDash.exe" + 0x224B0) pub unsafe extern "thiscall" fn Self::__on_button_1(this: *const FLAlertLayer, button: *const cocos2d_CCObject);
        ("GeometryDash.exe" + 0x224B0) pub unsafe extern "thiscall" fn Self::__on_button_2(this: *const FLAlertLayer, button: *const cocos2d_CCObject);
        ("GeometryDash.exe" + 0x22680) pub unsafe extern "thiscall" fn Self::__create(this: *const FLAlertLayer, target: *const dyn FLAlertLayerProtocol, title: *const c_char, button_1: *const c_char, button_2: *const c_char, caption: CString) -> *const FLAlertLayer;
        ("GeometryDash.exe" + 0x22680) pub unsafe extern "thiscall" fn Self::__create_with_width(this: *const FLAlertLayer, target: *const dyn FLAlertLayerProtocol, title: *const c_char, button_1: *const c_char, button_2: *const c_char, width: f32, caption: CString) -> *const FLAlertLayer;
    }
}
