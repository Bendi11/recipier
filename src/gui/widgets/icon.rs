//! Widgets that contain one bezier path to draw with optional colors

use druid::{Color, Data, KeyOrValue, RenderContext, Size, Widget, kurbo::BezPath};

pub static BOWL_ICON: IconData = IconData {
    path: include_str!("../../../assets/icon-path.txt"),
    size: Size::new(16., 16.),
    rendermethod: IconRenderMethod::Outline(2.)
};

/// Structure with all data needed to render an icon: size and bezier path
#[derive(Clone, Debug)]
pub struct IconData {
    /// The path to draw
    pub path: &'static str,
    /// The size of the svg 
    pub size: Size,
    /// How to render the bezier
    pub rendermethod: IconRenderMethod,
}

/// How an SVG image is drawn onscreen
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum IconRenderMethod {
    /// Fill the bezier path with color
    Fill,
    /// Outline with width
    Outline(f64),
}

/// Icon that holds a bezier path, color, and size to render
#[derive(Clone)]
pub struct Icon {
    /// The data to render
    path: BezPath,
    /// The size of the svg 
    size: Size,
    /// How to render the bezier
    rendermethod: IconRenderMethod,
    /// What color to render the data in
    color: KeyOrValue<Color>
}

impl Icon {
    /// Create a new icon from the specified icon data
    pub fn svg(data: &IconData) -> Self {
        Self {
            path: BezPath::from_svg(data.path).unwrap(),
            size: data.size,
            rendermethod: data.rendermethod,
            color: Color::BLACK.into()
        }
    }
    /// Builder method to set the rendering color of this icon
    pub fn with_color(mut self, color: impl Into<KeyOrValue<Color>>) -> Self {
        self.color = color.into();
        self
    }

    /// Builder method to set the size
    pub fn with_size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }
}

impl<D: Data> Widget<D> for Icon {
    fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &D, _data: &D, _env: &druid::Env) {
        
    }

    fn layout(&mut self, _ctx: &mut druid::LayoutCtx, _bc: &druid::BoxConstraints, _data: &D, _env: &druid::Env) -> Size {
        self.size
    }

    fn event(&mut self, _ctx: &mut druid::EventCtx, _event: &druid::Event, _data: &mut D, _env: &druid::Env) {
        
    }

    fn lifecycle(&mut self, _ctx: &mut druid::LifeCycleCtx, _event: &druid::LifeCycle, _data: &D, _env: &druid::Env) {
        
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, _data: &D, env: &druid::Env) {
        let color = self.color.resolve(env);
        match self.rendermethod {
            IconRenderMethod::Fill => ctx.fill(&self.path, &color),
            IconRenderMethod::Outline(width) => ctx.stroke(&self.path, &color, width)
        }
    }
}