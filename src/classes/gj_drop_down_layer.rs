use std::marker::PhantomData;

use rcocos2d_sys::{cocos2d_CCLayer, cocos2d_CCMenu, cocos2d_CCPoint};
use struct_pad::{PadU16, PadU32, PadU8};

#[repr(C)]
pub struct GJDropDownLayer {
    obj_end_position: cocos2d_CCPoint,
    obj_start_positon: cocos2d_CCPoint,
    button_menu: *mut cocos2d_CCMenu,
    list_layer: PhantomData<u8>, // *mut GJListLayer
    controller_enabled: bool,
    layer: *mut cocos2d_CCLayer,
    hidden: bool,
    __pad1: PadU32,
    __pad2: PadU16,
    __pad3: PadU8,
}

impl GJDropDownLayer {
    // TODO: Port methods from https://github.com/poweredbypie/gd.h/blob/160af2a902bb8fc4a94a3e72c9cb7fec5e4d46aa/layers_scenes_transitions_nodes/GJDropDownLayer.h
}
