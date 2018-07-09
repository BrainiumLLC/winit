use super::ffi::{id, CGRect};

use objc::runtime::Object;

extern "C" {
    fn MTLCreateSystemDefaultDevice() -> id;
}

pub const VIEW_CLASS: &'static str = "MTKView";
pub const LAYER_CLASS: &'static str = "CAMetalLayer";

pub unsafe fn init_view_with_frame(this: &Object, bounds: CGRect) -> id {
    msg_send![this, initWithFrame:bounds device: MTLCreateSystemDefaultDevice()]
}