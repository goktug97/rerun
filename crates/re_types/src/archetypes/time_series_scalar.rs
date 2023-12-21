// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/archetypes/time_series_scalar.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: Log a double-precision scalar that will be visualized as a time-series plot.
///
/// The current simulation time will be used for the time/X-axis, hence scalars
/// cannot be timeless!
///
/// ## Example
///
/// ### Simple line plot
/// ```ignore
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_scalar").spawn()?;
///
///     for step in 0..64 {
///         rec.set_time_sequence("step", step);
///         rec.log(
///             "scalar",
///             &rerun::TimeSeriesScalar::new((step as f64 / 10.0).sin()),
///         )?;
///     }
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/1200w.png">
///   <img src="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq)]
pub struct TimeSeriesScalar {
    /// The scalar value to log.
    pub scalar: crate::components::Scalar,

    /// An optional radius for the point.
    ///
    /// Points within a single line do not have to share the same radius, the line
    /// will have differently sized segments as appropriate.
    ///
    /// If all points within a single entity path (i.e. a line) share the same
    /// radius, then this radius will be used as the line width too. Otherwise, the
    /// line will use the default width of `1.0`.
    pub radius: Option<crate::components::Radius>,

    /// Optional color for the scalar entry.
    ///
    /// If left unspecified, a pseudo-random color will be used instead. That
    /// same color will apply to all points residing in the same entity path
    /// that don't have a color specified.
    ///
    /// Points within a single line do not have to share the same color, the line
    /// will have differently colored segments as appropriate.
    /// If all points within a single entity path (i.e. a line) share the same
    /// color, then this color will be used as the line color in the plot legend.
    /// Otherwise, the line will appear gray in the legend.
    pub color: Option<crate::components::Color>,

    /// An optional label for the point.
    ///
    /// TODO(#1289): This won't show up on points at the moment, as our plots don't yet
    /// support displaying labels for individual points.
    /// If all points within a single entity path (i.e. a line) share the same label, then
    /// this label will be used as the label for the line itself. Otherwise, the
    /// line will be named after the entity path. The plot itself is named after
    /// the space it's in.
    pub label: Option<crate::components::Text>,

    /// Specifies whether a point in a scatter plot should form a continuous line.
    ///
    /// If set to true, this scalar will be drawn as a point, akin to a scatterplot.
    /// Otherwise, it will form a continuous line with its neighbors.
    /// Points within a single line do not have to all share the same scatteredness:
    /// the line will switch between a scattered and a continuous representation as
    /// required.
    pub scattered: Option<crate::components::ScalarScattering>,
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.Scalar".into()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.Color".into(),
            "rerun.components.Radius".into(),
            "rerun.components.TimeSeriesScalarIndicator".into(),
        ]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.InstanceKey".into(),
            "rerun.components.ScalarScattering".into(),
            "rerun.components.Text".into(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 7usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.Scalar".into(),
            "rerun.components.Color".into(),
            "rerun.components.Radius".into(),
            "rerun.components.TimeSeriesScalarIndicator".into(),
            "rerun.components.InstanceKey".into(),
            "rerun.components.ScalarScattering".into(),
            "rerun.components.Text".into(),
        ]
    });

impl TimeSeriesScalar {
    pub const NUM_COMPONENTS: usize = 7usize;
}

/// Indicator component for the [`TimeSeriesScalar`] [`::re_types_core::Archetype`]
pub type TimeSeriesScalarIndicator = ::re_types_core::GenericIndicatorComponent<TimeSeriesScalar>;

impl ::re_types_core::Archetype for TimeSeriesScalar {
    type Indicator = TimeSeriesScalarIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.TimeSeriesScalar".into()
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: TimeSeriesScalarIndicator = TimeSeriesScalarIndicator::DEFAULT;
        MaybeOwnedComponentBatch::Ref(&INDICATOR)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let scalar = {
            let array = arrays_by_name
                .get("rerun.components.Scalar")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.TimeSeriesScalar#scalar")?;
            <crate::components::Scalar>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.TimeSeriesScalar#scalar")?
                .into_iter()
                .next()
                .flatten()
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.TimeSeriesScalar#scalar")?
        };
        let radius = if let Some(array) = arrays_by_name.get("rerun.components.Radius") {
            <crate::components::Radius>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.TimeSeriesScalar#radius")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let color = if let Some(array) = arrays_by_name.get("rerun.components.Color") {
            <crate::components::Color>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.TimeSeriesScalar#color")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let label = if let Some(array) = arrays_by_name.get("rerun.components.Text") {
            <crate::components::Text>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.TimeSeriesScalar#label")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let scattered = if let Some(array) = arrays_by_name.get("rerun.components.ScalarScattering")
        {
            <crate::components::ScalarScattering>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.TimeSeriesScalar#scattered")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        Ok(Self {
            scalar,
            radius,
            color,
            label,
            scattered,
        })
    }
}

impl ::re_types_core::AsComponents for TimeSeriesScalar {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            Some((&self.scalar as &dyn ComponentBatch).into()),
            self.radius
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.color
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.label
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.scattered
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    #[inline]
    fn num_instances(&self) -> usize {
        1
    }
}

impl TimeSeriesScalar {
    pub fn new(scalar: impl Into<crate::components::Scalar>) -> Self {
        Self {
            scalar: scalar.into(),
            radius: None,
            color: None,
            label: None,
            scattered: None,
        }
    }

    #[inline]
    pub fn with_radius(mut self, radius: impl Into<crate::components::Radius>) -> Self {
        self.radius = Some(radius.into());
        self
    }

    #[inline]
    pub fn with_color(mut self, color: impl Into<crate::components::Color>) -> Self {
        self.color = Some(color.into());
        self
    }

    #[inline]
    pub fn with_label(mut self, label: impl Into<crate::components::Text>) -> Self {
        self.label = Some(label.into());
        self
    }

    #[inline]
    pub fn with_scattered(
        mut self,
        scattered: impl Into<crate::components::ScalarScattering>,
    ) -> Self {
        self.scattered = Some(scattered.into());
        self
    }
}
