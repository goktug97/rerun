// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

/// The `AnnotationContext` provides additional information on how to display entities.
///
/// Entities can use `ClassId`s and `KeypointId`s to provide annotations, and
/// the labels and colors will be looked up in the appropriate
/// `AnnotationContext`. We use the *first* annotation context we find in the
/// path-hierarchy when searching up through the ancestors of a given entity
/// path.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AnnotationContext(pub Vec<crate::datatypes::ClassDescriptionMapElem>);

impl<I: Into<crate::datatypes::ClassDescriptionMapElem>, T: IntoIterator<Item = I>> From<T>
    for AnnotationContext
{
    fn from(v: T) -> Self {
        Self(v.into_iter().map(|v| v.into()).collect())
    }
}

impl<'a> From<AnnotationContext> for ::std::borrow::Cow<'a, AnnotationContext> {
    #[inline]
    fn from(value: AnnotationContext) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a AnnotationContext> for ::std::borrow::Cow<'a, AnnotationContext> {
    #[inline]
    fn from(value: &'a AnnotationContext) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl crate::Loggable for AnnotationContext {
    type Name = crate::ComponentName;
    type Item<'a> = Option<Self>;
    type Iter<'a> = <Vec<Self::Item<'a>> as IntoIterator>::IntoIter;

    #[inline]
    fn name() -> Self::Name {
        "rerun.annotation_context".into()
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn to_arrow_datatype() -> arrow2::datatypes::DataType {
        use ::arrow2::datatypes::*;
        DataType::List(Box::new(Field {
            name: "item".to_owned(),
            data_type: <crate::datatypes::ClassDescriptionMapElem>::to_arrow_datatype(),
            is_nullable: false,
            metadata: [].into(),
        }))
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
        extension_wrapper: Option<&str>,
    ) -> crate::SerializationResult<Box<dyn ::arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        use crate::{Loggable as _, ResultExt as _};
        use ::arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data0): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| {
                        let Self(data0) = datum.into_owned();
                        data0
                    });
                    (datum.is_some(), datum)
                })
                .unzip();
            let data0_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            {
                use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                let data0_inner_data: Vec<_> = data0
                    .iter()
                    .flatten()
                    .flatten()
                    .cloned()
                    .map(Some)
                    .collect();
                let data0_inner_bitmap: Option<::arrow2::bitmap::Bitmap> = None;
                let offsets = ::arrow2::offset::Offsets::<i32>::try_from_lengths(
                    data0
                        .iter()
                        .map(|opt| opt.as_ref().map(|datum| datum.len()).unwrap_or_default()),
                )
                .unwrap()
                .into();
                ListArray::new(
                        {
                            _ = extension_wrapper;
                            DataType::Extension(
                                    "rerun.components.AnnotationContext".to_owned(),
                                    Box::new(
                                        DataType::List(
                                            Box::new(Field {
                                                name: "item".to_owned(),
                                                data_type: <crate::datatypes::ClassDescriptionMapElem>::to_arrow_datatype(),
                                                is_nullable: false,
                                                metadata: [].into(),
                                            }),
                                        ),
                                    ),
                                    None,
                                )
                                .to_logical_type()
                                .clone()
                        },
                        offsets,
                        {
                            _ = data0_inner_bitmap;
                            _ = extension_wrapper;
                            crate::datatypes::ClassDescriptionMapElem::try_to_arrow_opt(
                                data0_inner_data,
                                Some("rerun.components.AnnotationContext"),
                            )?
                        },
                        data0_bitmap,
                    )
                    .boxed()
            }
        })
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_from_arrow_opt(
        data: &dyn ::arrow2::array::Array,
    ) -> crate::DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        use crate::{Loggable as _, ResultExt as _};
        use ::arrow2::{array::*, buffer::*, datatypes::*};
        Ok({
            let data = data
                .as_any()
                .downcast_ref::<::arrow2::array::ListArray<i32>>()
                .ok_or_else(|| {
                    crate::DeserializationError::datatype_mismatch(
                        DataType::List(Box::new(Field {
                            name: "item".to_owned(),
                            data_type:
                                <crate::datatypes::ClassDescriptionMapElem>::to_arrow_datatype(),
                            is_nullable: false,
                            metadata: [].into(),
                        })),
                        data.data_type().clone(),
                    )
                })
                .with_context("rerun.components.AnnotationContext#class_map")?;
            if data.is_empty() {
                Vec::new()
            } else {
                let data_inner = {
                    let data_inner = &**data.values();
                    crate::datatypes::ClassDescriptionMapElem::try_from_arrow_opt(data_inner)
                        .with_context("rerun.components.AnnotationContext#class_map")?
                        .into_iter()
                        .collect::<Vec<_>>()
                };
                let offsets = data.offsets();
                arrow2::bitmap::utils::ZipValidity::new_with_validity(
                    offsets.iter().zip(offsets.lengths()),
                    data.validity(),
                )
                .map(|elem| {
                    elem.map(|(start, len)| {
                        let start = *start as usize;
                        let end = start + len;
                        if end as usize > data_inner.len() {
                            return Err(crate::DeserializationError::offset_slice_oob(
                                (start, end),
                                data_inner.len(),
                            ));
                        }

                        #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                        let data =
                            unsafe { data_inner.get_unchecked(start as usize..end as usize) };
                        let data = data
                            .iter()
                            .cloned()
                            .map(Option::unwrap_or_default)
                            .collect();
                        Ok(data)
                    })
                    .transpose()
                })
                .collect::<crate::DeserializationResult<Vec<Option<_>>>>()?
            }
            .into_iter()
        }
        .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
        .map(|res| res.map(|v| Some(Self(v))))
        .collect::<crate::DeserializationResult<Vec<Option<_>>>>()
        .with_context("rerun.components.AnnotationContext#class_map")
        .with_context("rerun.components.AnnotationContext")?)
    }

    #[inline]
    fn try_iter_from_arrow(
        data: &dyn ::arrow2::array::Array,
    ) -> crate::DeserializationResult<Self::Iter<'_>>
    where
        Self: Sized,
    {
        Ok(Self::try_from_arrow_opt(data)?.into_iter())
    }

    #[inline]
    fn convert_item_to_opt_self(item: Self::Item<'_>) -> Option<Self> {
        item
    }
}

impl crate::Component for AnnotationContext {}
