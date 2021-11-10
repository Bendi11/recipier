//! Widgets that contain one bezier path to draw with optional colors

use druid::{kurbo::BezPath, Affine, Color, Data, KeyOrValue, RenderContext, Size, Widget};

/// Logo bowl icon SVG data
pub const BOWL_ICON: IconData = IconData {
    path: include_str!("../../../assets/icon-path.txt"),
    size: Size::new(16., 16.),
};

/// Structure with all data needed to render an icon: size and bezier path
#[derive(Clone, Debug)]
pub struct IconData {
    /// The path to draw
    pub path: &'static str,
    /// The size of the svg
    pub size: Size,
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
}

impl Icon {
    /// Create a new icon from the specified icon data
    pub fn svg(data: &IconData) -> Self {
        Self {
            path: BezPath::from_svg(data.path).unwrap(),
            scale: 1.,
            size: data.size,
            color: Color::BLACK.into(),
        }
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
}

impl<D: Data> Widget<D> for Icon {
    fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &D, _data: &D, _env: &druid::Env) {
    }

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        _bc: &druid::BoxConstraints,
        _data: &D,
        _env: &druid::Env,
    ) -> Size {
        self.size * self.scale
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