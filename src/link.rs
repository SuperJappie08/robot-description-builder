use std::{
	cell::RefCell,
	fmt::Debug,
	rc::{Rc, Weak},
};

use crate::{
	cluster_objects::{
		kinematic_tree::KinematicTree, kinematic_tree_data::KinematicTreeData, KinematicInterface,
	},
	joint::{Joint, JointType},
	Robot,
};

#[derive(Debug)]
pub enum LinkParent {
	Robot(Weak<RefCell<Robot>>),
	Joint(Weak<RefCell<Joint>>),
	KinematicTree(Weak<RefCell<KinematicTreeData>>),
}

impl Clone for LinkParent {
	fn clone(&self) -> Self {
		match self {
			Self::Robot(robot) => Self::Robot(Weak::clone(robot)),
			Self::Joint(joint) => Self::Joint(Weak::clone(joint)),
			Self::KinematicTree(tree) => todo!(),
		}
	}
}

impl From<Weak<RefCell<Robot>>> for LinkParent {
	fn from(value: Weak<RefCell<Robot>>) -> Self {
		Self::Robot(value)
	}
}

impl From<Weak<RefCell<KinematicTreeData>>> for LinkParent {
	fn from(value: Weak<RefCell<KinematicTreeData>>) -> Self {
		Self::KinematicTree(value)
	}
}

impl PartialEq for LinkParent {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Robot(l0), Self::Robot(r0)) => l0.upgrade() == r0.upgrade(),
			(Self::Joint(l0), Self::Joint(r0)) => l0.upgrade() == r0.upgrade(),
			(Self::KinematicTree(l0), Self::KinematicTree(r0)) => l0.upgrade() == r0.upgrade(),
			_ => false,
		}
	}
}

#[derive(Debug)]
pub struct Visual;

#[derive(Debug)]
pub struct Collision;

pub trait LinkTrait: Debug {
	/// Returns the parent of the `Link` wrapped in a optional.
	fn get_parent(&self) -> Option<LinkParent>;
	fn set_parent(&mut self, parent: LinkParent);

	/// Returns the name of the `Link`
	fn get_name(&self) -> String; // TODO: This might be temp because I want dynamic names.

	fn get_joints(&self) -> Vec<Rc<RefCell<Joint>>>; // TODO: Not final?
	fn try_attach_child(
		&mut self,
		tree: Box<dyn KinematicInterface>,
		joint_name: String,
		_joint_type: JointType,
	) -> Result<(), String>;

	// fn get_visual(&self) -> Vec<()>;
	// fn get_colliders(&self) -> Vec<()>;

	fn add_visual(&mut self, visual: Visual) -> Self;
	fn add_collider(&mut self, Collider: Collision) -> Self;
}

#[derive(Debug)]
pub struct Link {
	pub name: String,
	pub(crate) tree: Weak<RefCell<KinematicTreeData>>,
	direct_parent: Option<LinkParent>,
	child_joints: Vec<Rc<RefCell<Joint>>>,
}

impl Link {
	pub fn new(name: String) -> KinematicTree {
		let link = Link {
			name,
			tree: Weak::new(),
			direct_parent: None,
			child_joints: Vec::new(),
		};

		let tree = KinematicTreeData::new_link(link);

		KinematicTree::new(tree)
	}
}

impl LinkTrait for Link {
	fn get_parent(&self) -> Option<LinkParent> {
		self.direct_parent.clone()
	}

	fn set_parent(&mut self, parent: LinkParent) {
		self.direct_parent = Some(parent);
		// TODO: Add yourself to registry.
	}

	fn get_name(&self) -> String {
		self.name.clone()
	}

	fn get_joints(&self) -> Vec<Rc<RefCell<Joint>>> {
		self.child_joints
			.iter()
			.map(|joint| Rc::clone(joint))
			.collect()
	}

	///Maybe rename to try attach child
	fn try_attach_child(
		&mut self,
		tree: Box<dyn KinematicInterface>,
		joint_name: String,
		_joint_type: JointType,
	) -> Result<(), String> {
		// TODO: NEEDS TO DO SOMETHING WITH JOINT TYPE
		let joint = Rc::new(RefCell::new(Joint {
			name: joint_name,
			parent_link: Weak::clone(
				self.tree
					.upgrade()
					.unwrap()
					.borrow()
					.links
					.get(&self.get_name())
					.unwrap(),
			),
			child_link: tree.get_root_link(),
		}));

		self.child_joints.push(joint);

		let mut parent_tree = self.tree.upgrade().unwrap().borrow_mut();
		parent_tree.try_merge(tree.get_kinematic_data())?;
		parent_tree.try_add_joint(joint)?;
		Ok(())
	}

	fn add_visual(&mut self, visual: Visual) -> Self {
		todo!()
	}

	fn add_collider(&mut self, Collider: Collision) -> Self {
		todo!()
	}
}

impl PartialEq for Link {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
			&& self.direct_parent == other.direct_parent
			&& self.child_joints == other.child_joints
			&& self.tree.upgrade() == other.tree.upgrade()
	}
}
