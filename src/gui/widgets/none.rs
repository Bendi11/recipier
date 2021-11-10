//! Empty widget

use druid::{Data, Size, Widget};

/// Widget that displays nothing and takes no space
pub struct NoWidget;

impl<D: Data> Widget<D> for NoWidget {
    fn event(
        &mut self,
        _ctx: &mut druid::EventCtx,
        _event: &druid::Event,
        _data: &mut D,
        _env: &druid::Env,
    ) {
    }
    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        _bc: &druid::BoxConstraints,
        _data: &D,
        _env: &druid::Env,
    ) -> druid::Size {
        Size::new(0., 0.)
    }
    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        _data: &D,
        _env: &druid::Env,
    ) {
    }
    fn paint(&mut self, _ctx: &mut druid::PaintCtx, _data: &D, _env: &druid::Env) {}
    fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &D, _data: &D, _env: &druid::Env) {
    }
}
