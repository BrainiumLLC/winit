use super::ffi::{id, CGRect};

use objc::runtime::Object;

pub const VIEW_CLASS: &'static str = "GLKView";
pub const LAYER_CLASS: &'static str = "CAMetalLayer";

pub unsafe fn init_view_with_frame(this: &Object, bounds: CGRect) -> id {
    msg_send![this, initWithFrame:bounds]
}