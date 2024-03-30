use std::ops::RangeInclusive;

use egui::{Align2, Color32, epaint, pos2, Pos2, Rect, Shape, Stroke, TextStyle, Ui, WidgetText};

use crate::{LineStyle, PlotBounds, PlotPoint, PlotTransform};
use crate::items::{DEFAULT_FILL_ALPHA, PlotItem};
use crate::items::values::PlotGeometry;

#[derive(Clone, Debug, PartialEq)]
pub struct HRay {
    pub(super) point: PlotPoint,
    pub(crate) stroke: Stroke,
    pub(super) name: String,
    pub(super) highlight: bool,
    pub(super) style: LineStyle,
}

impl HRay {
    pub fn new(point: impl Into<PlotPoint>) -> Self {
        Self {
            point: point.into(),
            stroke: Stroke::new(1.0, Color32::TRANSPARENT),
            name: String::default(),
            highlight: false,
            style: LineStyle::Solid,
        }
    }

    #[inline]
    pub fn highlight(mut self, highlight: bool) -> Self {
        self.highlight = highlight;
        self
    }

    #[inline]
    pub fn stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.stroke = stroke.into();
        self
    }

    #[inline]
    pub fn width(mut self, width: impl Into<f32>) -> Self {
        self.stroke.width = width.into();
        self
    }

    #[inline]
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.stroke.color = color.into();
        self
    }

    #[inline]
    pub fn style(mut self, style: LineStyle) -> Self {
        self.style = style;
        self
    }

    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn name(mut self, name: impl ToString) -> Self {
        self.name = name.to_string();
        self
    }
}

impl PlotItem for HRay {
    fn shapes(&self, ui: &mut Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        let HRay {
            point,
            stroke,
            highlight,
            style,
            ..
        } = self;

        // Round to minimize aliasing:
        let points = vec![
            ui.painter().round_pos_to_pixels(
                transform.position_from_point(&PlotPoint::new(point.x, point.y)),
            ),
            ui.painter().round_pos_to_pixels(
                transform.position_from_point(&PlotPoint::new(transform.bounds().max[0], point.y)),
            ),
        ];
        style.style_line(points, *stroke, *highlight, shapes);
    }

    fn initialize(&mut self, _x_range: RangeInclusive<f64>) {}

    fn name(&self) -> &str {
        &self.name
    }

    fn color(&self) -> Color32 {
        self.stroke.color
    }

    fn highlight(&mut self) {
        self.highlight = true;
    }

    fn highlighted(&self) -> bool {
        self.highlight
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::None
    }

    fn bounds(&self) -> PlotBounds {
        let mut bounds = PlotBounds::NOTHING;
        bounds.min[0] = self.point.x;
        bounds.min[1] = self.point.y;
        bounds.max[1] = self.point.y;
        bounds
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LinkedYHRay {
    pub(super) y: f64,
    pub(super) offset: f32,
    pub(crate) stroke: Stroke,
    pub(super) name: String,
    pub(super) highlight: bool,
    pub(super) style: LineStyle,
}

impl LinkedYHRay {
    pub fn new(y: impl Into<f64>, offset: impl Into<f32>) -> Self {
        Self {
            y: y.into(),
            offset: offset.into(),
            stroke: Stroke::new(1.0, Color32::TRANSPARENT),
            name: String::default(),
            highlight: false,
            style: LineStyle::Solid,
        }
    }

    #[inline]
    pub fn highlight(mut self, highlight: bool) -> Self {
        self.highlight = highlight;
        self
    }

    #[inline]
    pub fn stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.stroke = stroke.into();
        self
    }

    #[inline]
    pub fn width(mut self, width: impl Into<f32>) -> Self {
        self.stroke.width = width.into();
        self
    }

    #[inline]
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.stroke.color = color.into();
        self
    }

    #[inline]
    pub fn style(mut self, style: LineStyle) -> Self {
        self.style = style;
        self
    }

    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn name(mut self, name: impl ToString) -> Self {
        self.name = name.to_string();
        self
    }
}

impl PlotItem for LinkedYHRay {
    fn shapes(&self, ui: &mut Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        let LinkedYHRay {
            y,
            offset,
            stroke,
            highlight,
            style,
            ..
        } = self;

        // Round to minimize aliasing:
        let points = vec![
            ui.painter().round_pos_to_pixels(
                pos2(
                    transform.position_from_point_x(transform.bounds().max[0]) - *offset,
                    transform.position_from_point_y(self.y),
                )
            ),
            ui.painter().round_pos_to_pixels(
                transform.position_from_point(&PlotPoint::new(transform.bounds().max[0], *y)),
            ),
        ];
        style.style_line(points, *stroke, *highlight, shapes);
    }

    fn initialize(&mut self, _x_range: RangeInclusive<f64>) {}

    fn name(&self) -> &str {
        &self.name
    }

    fn color(&self) -> Color32 {
        self.stroke.color
    }

    fn highlight(&mut self) {
        self.highlight = true;
    }

    fn highlighted(&self) -> bool {
        self.highlight
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::None
    }

    fn bounds(&self) -> PlotBounds {
        let mut bounds = PlotBounds::NOTHING;
        bounds.min[1] = self.y;
        bounds.max[1] = self.y;
        bounds
    }
}

#[derive(Clone)]
pub struct LinkedYText {
    pub(crate) text: WidgetText,
    pub(super) y: f64,
    pub(super) offset: f32,
    pub(super) name: String,
    pub(super) highlight: bool,
    pub(super) color: Color32,
    pub(super) anchor: Align2,
}

impl LinkedYText {
    pub fn new(y: impl Into<f64>, offset: impl Into<f32>, text: impl Into<WidgetText>) -> Self {
        Self {
            text: text.into(),
            y: y.into(),
            offset: offset.into(),
            name: Default::default(),
            highlight: false,
            color: Color32::TRANSPARENT,
            anchor: Align2::CENTER_CENTER,
        }
    }

    #[inline]
    pub fn highlight(mut self, highlight: bool) -> Self {
        self.highlight = highlight;
        self
    }

    #[inline]
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.color = color.into();
        self
    }

    #[inline]
    pub fn anchor(mut self, anchor: Align2) -> Self {
        self.anchor = anchor;
        self
    }

    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn name(mut self, name: impl ToString) -> Self {
        self.name = name.to_string();
        self
    }
}

impl PlotItem for LinkedYText {
    fn shapes(&self, ui: &mut Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        let color = if self.color == Color32::TRANSPARENT {
            ui.style().visuals.text_color()
        } else {
            self.color
        };

        let galley =
            self.text
                .clone()
                .into_galley(ui, Some(false), f32::INFINITY, TextStyle::Small);

        let pos = pos2(
            transform.position_from_point_x(transform.bounds().max[0]) - self.offset,
            transform.position_from_point_y(self.y),
        );
        let rect = self
            .anchor
            .anchor_rect(Rect::from_min_size(pos, galley.size()));

        shapes.push(epaint::TextShape::new(rect.min, galley, color).into());

        if self.highlight {
            shapes.push(Shape::rect_stroke(
                rect.expand(2.0),
                1.0,
                Stroke::new(0.5, color),
            ));
        }
    }

    fn initialize(&mut self, _x_range: RangeInclusive<f64>) {}

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn color(&self) -> Color32 {
        self.color
    }

    fn highlight(&mut self) {
        self.highlight = true;
    }

    fn highlighted(&self) -> bool {
        self.highlight
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::None
    }

    fn bounds(&self) -> PlotBounds {
        let mut bounds = PlotBounds::NOTHING;
        bounds.min[1] = self.y;
        bounds.max[1] = self.y;
        bounds
    }
}

pub struct LinkedYPolygon {
    pub(crate) series: Vec<Pos2>,
    pub(super) y: f64,
    pub(crate) stroke: Stroke,
    pub(super) name: String,
    pub(super) highlight: bool,
    pub(super) fill_color: Option<Color32>,
    pub(super) style: LineStyle,
}

impl LinkedYPolygon {
    pub fn new(series: Vec<Pos2>, y: impl Into<f64>) -> Self {
        Self {
            series,
            y: y.into(),
            stroke: Stroke::new(1.0, Color32::TRANSPARENT),
            name: Default::default(),
            highlight: false,
            fill_color: None,
            style: LineStyle::Solid,
        }
    }

    #[inline]
    pub fn highlight(mut self, highlight: bool) -> Self {
        self.highlight = highlight;
        self
    }

    #[inline]
    pub fn stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.stroke = stroke.into();
        self
    }

    #[inline]
    pub fn width(mut self, width: impl Into<f32>) -> Self {
        self.stroke.width = width.into();
        self
    }

    #[inline]
    pub fn fill_color(mut self, color: impl Into<Color32>) -> Self {
        self.fill_color = Some(color.into());
        self
    }

    #[inline]
    pub fn style(mut self, style: LineStyle) -> Self {
        self.style = style;
        self
    }

    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn name(mut self, name: impl ToString) -> Self {
        self.name = name.to_string();
        self
    }
}

impl PlotItem for LinkedYPolygon {
    fn shapes(&self, _ui: &mut Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        let Self {
            series,
            y,
            stroke,
            highlight,
            fill_color,
            style,
            ..
        } = self;

        let x = transform.position_from_point_x(transform.bounds().max[0]);
        let y = transform.position_from_point_y(*y);

        let mut values_tf: Vec<_> = series
            .iter()
            .map(|v| pos2(x + v.x, y + v.y))
            .collect();

        let fill_color = fill_color.unwrap_or(stroke.color.linear_multiply(DEFAULT_FILL_ALPHA));

        let shape = Shape::convex_polygon(values_tf.clone(), fill_color, Stroke::NONE);
        shapes.push(shape);
        values_tf.push(*values_tf.first().unwrap());
        style.style_line(values_tf, *stroke, *highlight, shapes);
    }

    fn initialize(&mut self, _x_range: RangeInclusive<f64>) {}

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn color(&self) -> Color32 {
        self.stroke.color
    }

    fn highlight(&mut self) {
        self.highlight = true;
    }

    fn highlighted(&self) -> bool {
        self.highlight
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::None
    }

    fn bounds(&self) -> PlotBounds {
        let mut bounds = PlotBounds::NOTHING;
        bounds.min[1] = self.y;
        bounds.max[1] = self.y;
        bounds
    }
}
