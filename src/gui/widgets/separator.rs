//! A separator between elements that draws a line and some space

use druid::{Color, Data, KeyOrValue, RenderContext, Size, Widget, kurbo::RoundedRect};

use crate::gui::theme;

/// Widget that displays a line filling the availible width
#[derive(Clone, Debug,)]
pub struct Separator {
    /// The color of this separator
    color: KeyOrValue<Color>,
    /// Width of the drawn separator
    width: f64,
}

impl Separator {
    /// Create a new separator with default color
    #[inline]
    pub fn new(width: f64) -> Self {
        Self {
            color: theme::COLOR_3.into(),
            width
        }
    }
    
    /// Builder method to set the color of the line
    pub fn with_color(mut self, color: impl Into<KeyOrValue<Color>>) -> Self {
        self.color = color.into();
        self
    }
}

impl<D: Data> Widget<D> for Separator {
    fn event(&mut self, _ctx: &mut druid::EventCtx, _event: &druid::Event, _data: &mut D, _env: &druid::Env) {
        
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, _data: &D, env: &druid::Env) {
        let color = self.color.resolve(env);
        let width = ctx.size().width;
        ctx.fill(RoundedRect::new(0., 0., width, 1., 10.), &color);
    }

    fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &D, _data: &D, _env: &druid::Env) {
        
    }

    fn layout(&mut self, _ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, _data: &D, _env: &druid::Env) -> Size {
        Size::new(bc.max().width, 1.)
    }

    fn lifecycle(&mut self, _ctx: &mut druid::LifeCycleCtx, _event: &druid::LifeCycle, _data: &D, _env: &druid::Env) {
        
    }   
}