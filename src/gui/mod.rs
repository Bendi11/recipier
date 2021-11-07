//! Module with function to build the GUI screens
pub mod state;

use druid::{Widget, WidgetExt, theme, widget::{Align, Axis, Flex, Label, List, Scroll, Tabs, TabsEdge, TabsTransition}};

use state::State;

/// Build the main screen widget
pub fn root_widget() -> impl Widget<State> {
    let tabs = Tabs::new()
        .with_edge(TabsEdge::Leading)
        .with_axis(Axis::Horizontal)
        .with_transition(TabsTransition::Slide(50_000))
        .with_tab("Recipes", recipes_widget());

    tabs
}

/// Build the recipes list widget
pub fn recipes_widget() -> impl Widget<State> {
    let mut layout = Flex::column();

    let title = Label::new("Recipes")
        .with_text_size(theme::TEXT_SIZE_LARGE)
        .with_font(theme::UI_FONT_BOLD);
    
    layout.add_child(Align::centered(title));

    
    layout
}