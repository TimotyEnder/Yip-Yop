use std::any::Any;

use crate::screenspace::elements::drawable::Drawable;

pub trait GameObject: Drawable + Any {
    fn start(&mut self);
    fn update(&mut self, delta_time: &f64);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
pub trait GameObjectImpl {
    fn on_start(&mut self);
    fn on_update(&mut self, delta_time: &f64);
}
// Macro that implements GameObject with automatic as_any
#[macro_export]
macro_rules! impl_gameobject {
    ($type:ty) => {
        use std::any::Any;
        use $crate::gameobject::{GameObject, GameObjectImpl};

        impl GameObject for $type {
            fn start(&mut self) {
                GameObjectImpl::on_start(self); // Calls user's implementation
            }

            fn update(&mut self, delta_time: &f64) {
                GameObjectImpl::on_update(self, delta_time); // Calls user's implementation
            }

            fn as_any(&self) -> &dyn Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
        }
    };
}
