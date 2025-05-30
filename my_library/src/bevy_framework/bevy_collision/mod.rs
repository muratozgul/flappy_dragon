mod aabb;
mod rect2d;
mod static_quadtree;
pub use aabb::AxisAlignedBoundingBox;
pub use rect2d::Rect2D;
pub use static_quadtree::*;
use bevy::{prelude::*, utils::HashMap};
use std::marker::PhantomData;

#[derive(Event)]
pub struct OnCollision<A, B>
where
  A: Component,
  B: Component,
{
  pub entity_a: Entity,
  pub entity_b: Entity,
  marker: PhantomData<(A, B)>,
}

pub fn check_collisions<A, B>(
  quad_tree: Res<StaticQuadTree>,
  query_a: Query<(Entity, &Transform, &AxisAlignedBoundingBox), With<A>>,
  query_b: Query<(Entity, &Transform, &AxisAlignedBoundingBox), With<B>>,
  mut sender: EventWriter<OnCollision<A, B>>,
) where
  A: Component,
  B: Component,
{
  let mut spatial_index: HashMap<usize, Vec<(Entity, Rect2D)>> =
    HashMap::new();

  query_b.iter().for_each(|(entity, transform, bbox)| {
    let bbox = bbox.as_rect(transform.translation.truncate());
    let in_node = quad_tree.smallest_node(&bbox);
    if let Some(contents) = spatial_index.get_mut(&in_node) {
      contents.push((entity, bbox));
    } else {
      spatial_index.insert(in_node, vec![(entity, bbox)]);
    }
  });

  query_a.iter().for_each(|(entity_a, transform_a, bbox_a)| {
    let bbox_a = bbox_a.as_rect(transform_a.translation.truncate());
    for node in quad_tree.intersecting_nodes(&bbox_a) {
      if let Some(contents) = spatial_index.get(&node) {
        for (entity_b, bbox_b) in contents {
          if entity_a != *entity_b && bbox_a.intersect(bbox_b) {
            sender.send(OnCollision {
              entity_a,
              entity_b: *entity_b,
              marker: PhantomData,
            });
          }
        }
      }
    }
  });
}