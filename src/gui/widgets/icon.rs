//! Widgets that contain one bezier path to draw with optional colors

use druid::{kurbo::BezPath, Affine, Color, Data, KeyOrValue, RenderContext, Size, Widget};

use crate::gui::theme;
use lazy_static::lazy_static;

use super::RecipierWidget;

lazy_static! {
    pub static ref SAVE_ICON: Icon = Icon {
        path: BezPath::from_svg(include_str!("../../../assets/icons/save-path.txt")).unwrap(),
        scale: 1.,
        size: Size::new(16., 16.),
        flex: true,
        color: theme::COLOR_4.into(),
    };

    pub static ref RECYCLE_ICON: Icon = Icon {
        path: BezPath::from_svg(include_str!("../../../assets/icons/recycle-path.txt")).unwrap(),
        scale: 1.,
        size: Size::new(16., 16.),
        flex: true,
        color: theme::COLOR_4.into(),
    };

    pub static ref PEN_ICON: Icon = Icon {
        path: BezPath::from_svg(include_str!("../../../assets/icons/pen-path.txt")).unwrap(),
        scale: 1.,
        size: Size::new(16., 16.),
        flex: true,
        color: theme::COLOR_4.into(),
    };

    pub static ref RIGHT_ARROW_ICON: Icon = Icon {
        path: BezPath::from_svg(include_str!("../../../assets/icons/right-arrow-path.txt")).unwrap(),
        scale: 1.,
        size: Size::new(16., 16.),
        flex: true,
        color: theme::COLOR_4.into(),
    };

    pub static ref PLUS_ICON: Icon = Icon {
        path: BezPath::from_svg(include_str!("../../../assets/icons/plus-path.txt")).unwrap(),
        scale: 1.,
        size: Size::new(16., 16.),
        flex: true,
        color: theme::COLOR_4.into(),
    };

    pub static ref SEARCH_ICON: Icon = Icon {
        path: BezPath::from_svg(include_str!("../../../assets/icons/search-path.txt")).unwrap(),
        scale: 1.,
        size: Size::new(16., 16.),
        flex: true,
        color: theme::COLOR_4.into(),
    };

    pub static ref BOWL_ICON: Icon = Icon {
        path: BezPath::from_svg(include_str!("../../../assets/icon-path.txt")).unwrap(),
        scale: 1.,
        size: Size::new(16., 16.),
        flex: true,
        color: theme::COLOR_4.into(),
    };
}

/// Icon that holds a bezier path, color, and size to render
#[derive(Clone)]
pub struct Icon {
    /// The data to render
    path: BezPath,
    /// Scale factor of the icon
    scale: f64,
    /// The size of the svg
    size: Size,
    /// What color to render the data in
    color: KeyOrValue<Color>,
    /// If the widget should scale dynamically
    flex: bool,
}

impl Icon {
    /// Builder method to enable / disable flexible sizing
    pub fn flex(mut self, flex: bool) -> Self {
        self.flex = flex;
        self
    }

    /// Make this widget highlight when the mouse cursor hovers over it
    pub fn highlight_on_hover<D: Data>(self) -> impl Widget<D> {
        self.on_hover(
            |ctx, _, this, _env| {
                this.set_color(theme::COLOR_3);
                ctx.request_paint();
            },
            |ctx, _, this, _env| {
                this.set_color(theme::COLOR_4);
                ctx.request_paint();
            },
        )
    }

    /// Builder method to set the rendering color of this icon
    pub fn with_color(mut self, color: impl Into<KeyOrValue<Color>>) -> Self {
        self.color = color.into();
        self
    }

    /// Builder method to set the scale of this icon
    pub fn with_scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }

    /// Mmethod to enable / disable flexible sizing
    pub fn set_flex(&mut self, flex: bool) {
        self.flex = flex;
    }

    /// Method to set the rendering color of this icon
    pub fn set_color(&mut self, color: impl Into<KeyOrValue<Color>>) {
        self.color = color.into();
    }

    /// Method to set the scale of this icon
    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale;
    }
}

impl<D: Data> Widget<D> for Icon {
    fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &D, _data: &D, _env: &druid::Env) {
    }

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        _data: &D,
        _env: &druid::Env,
    ) -> Size {
        if self.flex {
            let max = bc.max();

            match max.width > max.height {
                true => {
                    self.scale = max.height / self.size.height;
                    Size::new(max.height, max.height)
                }
                false => {
                    self.scale = max.width / self.size.width;
                    Size::new(max.width, max.width)
                }
            }
        } else {
            self.size * self.scale
        }
    }

    fn event(
        &mut self,
        _ctx: &mut druid::EventCtx,
        _event: &druid::Event,
        _data: &mut D,
        _env: &druid::Env,
    ) {
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        _data: &D,
        _env: &druid::Env,
    ) {
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, _data: &D, env: &druid::Env) {
        let color = self.color.resolve(env);
        //Save context so that transform doesn't scale everything
        ctx.with_save(|paint| {
            paint.transform(Affine::scale(self.scale));
            paint.fill(&self.path, &color);
        });
    }
}
