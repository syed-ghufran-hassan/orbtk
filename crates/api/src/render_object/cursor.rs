use crate::{
    proc_macros::IntoRenderObject,
    render_object::*,
    utils::{Brush, Point, Rectangle, Thickness},
};

/// The CursorRenderObject is used to render the `Cursor` widget.
///
/// [`Cursor`]: ../../widgets/struct.Cursor.html
#[derive(Debug, IntoRenderObject)]
pub struct CursorRenderObject;

impl RenderObject for CursorRenderObject {
    fn render_self(&self, ctx: &mut Context, global_position: &Point) {
        let (
            bounds,
            background,
            border_width,
            border_brush,
            background_opacity,
            selection,
            selection_width,
            selection_x,
        ) = {
            let widget = ctx.widget();
            (
                *widget.get::<Rectangle>("bounds"),
                widget.get::<Brush>("background").clone(),
                *widget.get::<Thickness>("border_width"),
                widget.clone_or_default::<Brush>("border_brush"),
                *widget.get::<f32>("background_opacity"),
                *widget.get::<TextSelection>("selection"),
                *widget.get::<f64>("selection_width"),
                *widget.get::<f64>("selection_x"),
            )
        };

        let x_position_indicator = {
            if selection.start() <= selection.end() {
                0.
            } else {
                bounds.width() - border_width.right()
            }
        };

        let border_width = border_width.right();

        // background
        ctx.render_context_2_d().set_alpha(background_opacity);
        ctx.render_context_2_d().set_fill_style(background);
        ctx.render_context_2_d().fill_rect(
            global_position.x() + bounds.x() + selection_x + border_width,
            global_position.y() + bounds.y(),
            bounds.width(),
            bounds.height(),
        );
        ctx.render_context_2_d().set_alpha(1.);

        // border
        ctx.render_context_2_d().set_fill_style(border_brush);
        ctx.render_context_2_d().fill_rect(
            global_position.x() + bounds.x() + selection_x + x_position_indicator,
            global_position.y() + bounds.y(),
            border_width,
            bounds.height(),
        );
    }
}