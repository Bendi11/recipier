//! A separator between elements that draws a line and some space

use druid::{kurbo::RoundedRect, Color, Data, KeyOrValue, RenderContext, Size, Widget};

use crate::gui::theme;

/// Widget that displays a line filling the availible width
#[derive(Clone, Debug)]
pub struct Separator {
    /// The color of this separator
    color: KeyOrValue<Color>,
    /// Width of the drawn separator
    width: f64,
    /// If the separator is vertical
    vertical: bool,
}

impl Separator {
    /// Create a new horizontal separator with default color
    #[inline]
    pub fn new(width: f64) -> Self {
        Self {
            color: theme::COLOR_3.into(),
            width,
            vertical: false,
        }
    }

    /// Builder method to render the separator as horizontal or vertical
    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }

    /// Builder method to set the color of the line
    pub fn with_color(mut self, color: impl Into<KeyOrValue<Color>>) -> Self {
        self.color = color.into();
        self
    }
}

impl<D: Data> Widget<D> for Separator {
    fn event(
        &mut self,
        _ctx: &mut druid::EventCtx,
        _event: &druid::Event,
        _data: &mut D,
        _env: &druid::Env,
    ) {
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, _data: &D, env: &druid::Env) {
        let color = self.color.resolve(env);
        let endpos = if self.vertical { ctx.size().height } else { ctx.size().width };

        const RATIO: f64 = 10.0;
        let spacing = endpos / RATIO;
        let offset = (spacing / 2.).min(5.);

        match self.vertical {
            true => ctx.fill(
                RoundedRect::new(0., offset, self.width, endpos - offset, 10.),
                &color,
            ),
            false => ctx.fill(
                RoundedRect::new(offset, 0., endpos - offset, self.width, 10.),
                &color,
            )
        }
    }

    fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &D, _data: &D, _env: &druid::Env) {
    }

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        _data: &D,
        _env: &druid::Env,
    ) -> Size {
        match self.vertical {
            true => Size::new(self.width, bc.max().height),
            false => Size::new(bc.max().width, self.width)
        }
        
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        _data: &D,
        _env: &druid::Env,
    ) {
    }
}
