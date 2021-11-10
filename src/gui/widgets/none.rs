//! Empty widget

use druid::{Data, Size, Widget};

/// Widget that displays nothing and takes no space
pub struct NoWidget;

impl<D: Data> Widget<D> for NoWidget {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut D, env: &druid::Env) {
        
    }
    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &D, env: &druid::Env) -> druid::Size {
        Size::new(0., 0.)
    }
    fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &D, env: &druid::Env) {
        
    }
    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &D, env: &druid::Env) {
        
    }
    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &D, data: &D, env: &druid::Env) {
        
    }
}