use druid::{Data, Env, EventCtx, Widget, widget::{Controller, ControllerHost}};

pub mod icon;
pub mod none;
pub mod separator;

/// A controller that handles hover events using the provided callback
pub struct HoverController<D, W> {
    /// The callback function to run on hover
    cb: Box<dyn Fn(&mut EventCtx, &mut D, &mut W, &Env) -> ()>,
    /// The function that undos the effects of the hover callback
    undo: Box<dyn Fn(&mut EventCtx, &mut D, &mut W, &Env) -> ()>,
    /// If the widget is hovered, used to debounce input
    hovered: bool,
}

impl<D: Data, W: Widget<D>> Controller<D, W> for HoverController<D, W> {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &druid::Event, data: &mut D, env: &Env) {
        if let druid::Event::MouseMove(_) = event {
            if ctx.is_hot() && !self.hovered {
                self.hovered = true;
                (self.cb)(ctx, data, child, env);
            } else if !ctx.is_hot() && self.hovered {
                self.hovered = false;
                (self.undo)(ctx, data, child, env);
            }
        }
        
        child.event(ctx, event, data, env)
    }
}

/// Extension methods for widgets with common functionality
pub trait RecipierWidget<D: Data>: Widget<D> + Sized {
    /// Set a callback function to execute on hover
    fn on_hover(self, 
        cb: impl Fn(&mut EventCtx, &mut D, &mut Self, &Env) -> () + 'static,
        undo: impl Fn(&mut EventCtx, &mut D, &mut Self, &Env) -> () + 'static,
    ) -> ControllerHost<Self, HoverController<D, Self>> {
        ControllerHost::new(self, HoverController {
            cb: Box::new(cb),
            undo: Box::new(undo),
            hovered: false
        })
    }
}

impl<D: Data, W: Widget<D>> RecipierWidget<D> for W {}