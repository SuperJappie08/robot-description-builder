use std::{
	cell::RefCell,
	rc::{Rc, Weak},
};

use crate::link::Link;

#[derive(Debug)]
pub struct Joint {
	pub name: String,
	parent_link: Weak<RefCell<Link>>,
	pub child_link: Rc<RefCell<Link>>, //temp pub TODO: THIS PROBABLY ISN'T THE NICEST WAY TO DO THIS.
}

/// TODO: Might add data of specif joint type to Struct Spaces.
#[derive(Debug)]
pub enum JointType {
	Fixed, // — this is not really a joint because it cannot move. All degrees of freedom are locked. This type of joint does not require the <axis>, <calibration>, <dynamics>, <limits> or <safety_controller>.
	Revolute, // — a hinge joint that rotates along the axis and has a limited range specified by the upper and lower limits.
	Continuous, // — a continuous hinge joint that rotates around the axis and has no upper and lower limits.
	Prismatic, // — a sliding joint that slides along the axis, and has a limited range specified by the upper and lower limits.
	Floating,  // — this joint allows motion for all 6 degrees of freedom.
	Planar,    // — this joint allows motion in a plane perpendicular to the axis.
}
