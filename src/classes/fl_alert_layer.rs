use std::{ffi::CString, marker::PhantomData, os::raw::c_char};

use crate::all_to_cstring;

use super::fl_alert_layer_protocol::FLAlertLayerProtocol;
use getfn::symbol_fn;
use rcocos2d_sys::{
    cocos2d_CCEvent, cocos2d_CCLayer, cocos2d_CCMenu, cocos2d_CCObject, cocos2d_CCTouch,
    cocos2d_ccColor3B,
};
use struct_pad::{PadU32, PadU64};

#[repr(C)]
pub struct FLAlertLayer {
    button_menu: *mut cocos2d_CCMenu,
    controller_connected: i32,
    target: *mut cocos2d_CCObject,
    __pad1: PadU64,
    layer: *mut cocos2d_CCLayer,
    z_order_2: i32,
    button_no_elasticity: bool,
    c_color_2: *mut cocos2d_ccColor3B,
    button1: *mut PhantomData<u8>,         // ButtonSprite
    button2: *mut PhantomData<u8>,         // ButtonSprite
    scrolling_layer: *mut PhantomData<u8>, // ScrollingLayer
    __pad2: PadU32,                          // pad m_nJoystickConnected
    button_border: Option<bool>,
    button_no_action: bool,
}

impl FLAlertLayer {
    symbol_fn! {
        ("GeometryDash.exe" + 0x23750) pub unsafe extern "thiscall" fn Self::__on_enter(this: *mut FLAlertLayer);
        ("GeometryDash.exe" + 0x236F0) pub unsafe extern "thiscall" fn Self::__register_with_touch_dispatcher(this: *mut FLAlertLayer);
        ("GeometryDash.exe" + 0x233C0) pub unsafe extern "thiscall" fn Self::__cc_touch_began(this: *mut FLAlertLayer, touch: *mut cocos2d_CCTouch, event: *mut cocos2d_CCEvent) -> bool;
        ("GeometryDash.exe" + 0x23510) pub unsafe extern "thiscall" fn Self::__cc_touch_moved(this: *mut FLAlertLayer, touch: *mut cocos2d_CCTouch, event: *mut cocos2d_CCEvent);
        ("GeometryDash.exe" + 0x23450) pub unsafe extern "thiscall" fn Self::__cc_touch_ended(this: *mut FLAlertLayer, touch: *mut cocos2d_CCTouch, event: *mut cocos2d_CCEvent);
        ("GeometryDash.exe" + 0x234C0) pub unsafe extern "thiscall" fn Self::__cc_touch_cancelled(this: *mut FLAlertLayer, touch: *mut cocos2d_CCTouch, event: *mut cocos2d_CCEvent);
        ("GeometryDash.exe" + 0x23750) pub unsafe extern "thiscall" fn Self::__key_down(this: *mut FLAlertLayer, key: i32);
        ("GeometryDash.exe" + 0x232C0) pub unsafe extern "thiscall" fn Self::__key_back_clicked(this: *mut FLAlertLayer);
        ("GeometryDash.exe" + 0x23560) pub unsafe extern "thiscall" fn Self::__show(this: *mut FLAlertLayer);
        ("GeometryDash.exe" + 0x224B0) pub unsafe extern "thiscall" fn Self::__FLAlertLayer() -> *mut FLAlertLayer;
        ("GeometryDash.exe" + 0x225c0) pub unsafe extern "thiscall" fn Self::__destructor(this: *mut FLAlertLayer);
        ("GeometryDash.exe" + 0x224B0) pub unsafe extern "thiscall" fn Self::__on_button_1(this: *mut FLAlertLayer, button: *mut cocos2d_CCObject);
        ("GeometryDash.exe" + 0x224B0) pub unsafe extern "thiscall" fn Self::__on_button_2(this: *mut FLAlertLayer, button: *mut cocos2d_CCObject);
        ("GeometryDash.exe" + 0x22680) pub unsafe extern "thiscall" fn Self::__create(this: *mut FLAlertLayer, target: *mut dyn FLAlertLayerProtocol, title: *const c_char, button_1: *const c_char, button_2: *const c_char, caption: CString) -> *mut FLAlertLayer;
        ("GeometryDash.exe" + 0x22680) pub unsafe extern "thiscall" fn Self::__create_with_width(this: *mut FLAlertLayer, target: *mut dyn FLAlertLayerProtocol, title: *const c_char, button_1: *const c_char, button_2: *const c_char, width: f32, caption: CString) -> *mut FLAlertLayer;
    }

    pub fn on_enter(&mut self) {
        unsafe { Self::__on_enter(self) }
    }

    pub fn register_with_touch_dispatcher(&mut self) {
        unsafe { Self::__register_with_touch_dispatcher(self) }
    }

    pub fn cc_touch_began(&mut self, touch: &mut cocos2d_CCTouch, event: &mut cocos2d_CCEvent) -> bool {
        unsafe { Self::__cc_touch_began(self, touch, event) }
    }

    pub fn cc_touch_moved(&mut self, touch: &mut cocos2d_CCTouch, event: &mut cocos2d_CCEvent) {
        unsafe { Self::__cc_touch_moved(self, touch, event) }
    }

    pub fn cc_touch_ended(&mut self, touch: &mut cocos2d_CCTouch, event: &mut cocos2d_CCEvent) {
        unsafe { Self::__cc_touch_ended(self, touch, event) }
    }

    pub fn cc_touch_cancelled(&mut self, touch: &mut cocos2d_CCTouch, event: &mut cocos2d_CCEvent) {
        unsafe { Self::__cc_touch_moved(self, touch, event) }
    }

    pub fn key_down(&mut self, key: i32) {
        unsafe { Self::__key_down(self, key) }
    }

    pub fn key_back_clicked(&mut self) {
        unsafe { Self::__key_back_clicked(self) }
    }

    pub fn show(&mut self) {
        unsafe { Self::__show(self) }
    }

    pub fn new() -> &'static FLAlertLayer {
        unsafe { Self::__FLAlertLayer().as_ref().unwrap() }
    }



    pub fn create(
        &mut self,
        target: Box<dyn FLAlertLayerProtocol>,
        title: &str,
        button_1: &str,
        button_2: &str,
        caption: String,
    ) -> &'static FLAlertLayer {
        unsafe {
            let strs = all_to_cstring!(title, button_1, button_2, caption);

            let ret = Self::__create(
                self,
                Box::<_>::into_raw(target),
                strs[0].as_ptr(),
                strs[1].as_ptr(),
                strs[2].as_ptr(),
                strs[3].to_owned(),
            );
            asm!("add esp, 0x20");

            ret.as_ref().unwrap()
        }
    }

    pub fn create_with_width(
        &mut self,
        target: Box<dyn FLAlertLayerProtocol>,
        title: &str,
        button_1: &str,
        button_2: &str,
        width: f32,
        caption: String,
    ) -> &'static FLAlertLayer {
        unsafe {
            let strs = all_to_cstring!(title, button_1, button_2, caption);

            let ret = Self::__create_with_width(
                self,
                Box::<_>::into_raw(target),
                strs[0].as_ptr(),
                strs[1].as_ptr(),
                strs[2].as_ptr(),
                width,
                strs[3].to_owned(),
            );
            asm!("add esp, 0x20");

            ret.as_ref().unwrap()
        }
    }
}

impl Drop for FLAlertLayer {
    fn drop(&mut self) {
        unsafe { Self::__destructor(self) }
    }
}
