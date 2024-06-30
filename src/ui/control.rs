use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use crate::ui::component::UiComponent;

pub struct UiControl {
    name: String,
    components: Vec<Box<dyn UiComponent>>,
    children: Vec<Self>,
}
