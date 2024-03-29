use bevy::math::Affine3A;
use bevy::text::{TextLayoutInfo, Text2dBounds};
use bevy::prelude::*;

use bevy::sprite::Anchor as BevyAnchor;
use crate::dimension::DimensionMut;
use crate::util::ScalingFactor;
use crate::{RotatedRect, BuildTransform, Transform2D, Opacity, IgnoreAlpha, BuildMeshTransform, Anchor, DimensionData, Dimension, Coloring};


/// Copy [`Anchor`](BevyAnchor) component's value to the [`Transform2D`] component
pub fn copy_anchor(mut query: Query<(&mut BevyAnchor, &Transform2D)>) {
    query.iter_mut().for_each(|(mut a, anc)| *a = anc.anchor.into())
}

/// Copy evaluated `TextLayoutInfo` value to our `Dimension::Copied` value
pub fn copy_dimension_text(mut query: Query<(&TextLayoutInfo, DimensionMut)>) {
    query.iter_mut().for_each(|(text, mut dim)| {
        dim.update_size(|| text.logical_size)
    })
}

/// Copy our [`Anchor`] value to the [`Sprite`] component
pub fn copy_anchor_sprite(
    mut query: Query<(&mut Sprite, &Transform2D)>
) {
    query.iter_mut().for_each(|(mut sp, anc)| {
        sp.anchor = anc.anchor.into();
    })
}

/// Synchronize size between [`Sprite`] and [`Dimension`].
pub fn copy_dimension_sprite(
    scaling_factor: ScalingFactor,
    mut query: Query<(&mut Sprite, &Handle<Image>, DimensionMut)>,
    assets: Res<Assets<Image>>
) {
    let scaling_factor = scaling_factor.get();
    query.iter_mut().for_each(|(sp, im, mut dimension)| {
        dimension.update_size(|| {
            let pixels = match sp.rect {
                Some(rect) => rect.max - rect.min,
                None => assets.get(im).map(|x|x.size().as_vec2()).unwrap_or(Vec2::ZERO),
            };
            pixels * scaling_factor
        });
    })
}

/// copy size between `TextureAtlasSprite` and `Dimension`
pub fn copy_dimension_atlas(
    scaling_factor: ScalingFactor,
    mut query: Query<(&TextureAtlas, DimensionMut)>,
    assets: Res<Assets<TextureAtlasLayout>>
) {
    let scaling_factor = scaling_factor.get();
    query.iter_mut().for_each(|(atlas, mut dimension)| {
        dimension.update_size(|| {
            let pixels = (|| -> Option<_> {
                let rect = assets.get(&atlas.layout)?.textures.get(atlas.index)?;
                Some(rect.max - rect.min)
            })().unwrap_or(Vec2::ZERO);
            pixels / scaling_factor
        });
    })
}

/// Synchonize size from `Dimension` to `Sprite`
pub fn sync_dimension_sprite(
    mut query: Query<(&mut Sprite, &Dimension, &DimensionData)>
) {
    //let scaling_factor = scaling_factor.get();
    query.iter_mut().for_each(|(mut sp, dimension, data)| {
        if !dimension.is_copied() && sp.custom_size != Some(data.size) {
            sp.custom_size = Some(data.size)
        }
    })
}

/// Copy owned dimension as text bounds.
pub fn sync_dimension_text_bounds(mut query: Query<(&mut Text2dBounds, &Dimension, &DimensionData), Without<OptOutTextBoundsSync>>) {
    query.iter_mut().for_each(|(mut sp, dimension, data)| {
        if !dimension.is_copied() && sp.as_ref().size != data.size {
            sp.size = data.size
        }
    })
}



/// Opts out of synchronizing text bounds.
#[derive(Debug, Component)]
pub struct OptOutTextBoundsSync;

/// Opts out of synchronizing font size.
#[derive(Debug, Component)]
pub struct OptOutFontSizeSync;


/// Copy em as text size.
pub fn set_occluded(mut query: Query<&mut Opacity>) {
    query.iter_mut().for_each(|mut op| { op.occluded = true })
}

/// Copy em as text size.
pub fn sync_em_text(mut query: Query<(&mut Text, &DimensionData), Without<OptOutFontSizeSync>>) {
    query.iter_mut().for_each(|(mut sp, dimension)| {
        if sp.as_ref().sections.iter().any(|x| x.style.font_size != dimension.em) {
            sp.sections.iter_mut().for_each(|x| x.style.font_size = dimension.em)
        }
    })
}

#[allow(clippy::collapsible_else_if)]
/// Copy opacity as sprite alpha.
pub fn sync_opacity_vis(mut query: Query<(&Opacity, &mut Visibility), Without<IgnoreAlpha>>) {
    query.iter_mut().for_each(|(opacity, mut vis)| {
        if opacity.computed_opacity <= 0.0 {
            if vis.as_ref() != Visibility::Hidden {
                *vis = Visibility::Hidden
            }
        } else {
            if vis.as_ref() != Visibility::Inherited {
                *vis = Visibility::Inherited
            }
        }
    })
}

/// Copy opacity as text alpha.
pub fn sync_opacity_text(mut query: Query<(&Coloring, &Opacity, &mut Text), Without<IgnoreAlpha>>) {
    query.iter_mut().for_each(|(color, opacity, mut text)| {
        let color = color.color.with_a(color.color.a() * opacity.get());
        if text.sections.iter().any(|x| x.style.color != color) {
            text.sections.iter_mut().for_each(|x| {x.style.color = color} )
        }
    })
}

/// Copy opacity as sprite alpha.
pub fn sync_opacity_sprite(mut query: Query<(&Coloring, &Opacity, &mut Sprite), Without<IgnoreAlpha>>) {
    query.iter_mut().for_each(|(color, opacity, mut sprite)| {
        let color = color.color.with_a(color.color.a() * opacity.get());
        if sprite.color != color {
            sprite.color = color;
        }
    })
}

pub fn build_mesh_2d_global_transform(
    mut query: Query<(&RotatedRect, &DimensionData, &mut GlobalTransform), With<BuildMeshTransform>>
) {
    query.iter_mut().for_each(|(rect, dim, mut transform)|
        *transform = Affine3A::from_scale_rotation_translation(
            (rect.scale * dim.size).extend(1.0),
            Quat::from_rotation_z(rect.rotation),
            rect.anchor(Anchor::CENTER).extend(rect.z)
        ).into()
    );
}

/// Generate [`GlobalTransform`] with  [`BuildTransform`].
pub fn build_global_transform(
    mut query: Query<(&BuildTransform, &Transform2D, &RotatedRect, &mut GlobalTransform)>,
) {
    query.iter_mut().for_each(|(build, transform, rect, mut global)| {
        *global = Affine3A::from_scale_rotation_translation(
            rect.scale.extend(1.0),
            Quat::from_rotation_z(rect.rotation),
            rect.anchor(build.0.or(transform.anchor)).extend(rect.z)
        ).into()
    });
}
