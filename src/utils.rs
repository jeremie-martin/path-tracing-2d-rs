#![feature(trait_alias)]

use ncollide2d;

pub type Vec2 = ncollide2d::math::Vector<f64>;
pub type Ray = ncollide2d::query::Ray<f64>;
pub type RayIntersection = ncollide2d::query::RayIntersection<f64>;
// pub type aabb = ncollide2d::bounding_volume::aabb4>;
pub type AABB = ncollide2d::bounding_volume::AABB<f64>;

pub type Point = ncollide2d::math::Point<f64>;
pub type Isometry = ncollide2d::math::Isometry<f64>;
