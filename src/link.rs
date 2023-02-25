use std::{
	cell::{BorrowMutError, RefCell},
	error::Error,
	fmt,
	rc::{Rc, Weak},
};

use crate::{
	cluster_objects::{
		kinematic_data_errors::{TryAddDataError, TryMergeTreeError},
		kinematic_tree::KinematicTree,
		kinematic_tree_data::KinematicTreeData,
		KinematicInterface,
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

// pub trait LinkTrait: Debug {
// 	/// Returns the parent of the `Link` wrapped in a optional.
// 	fn get_parent(&self) -> Option<LinkParent>;
// 	fn set_parent(&mut self, parent: LinkParent);

// 	/// Returns the name of the `Link`
// 	fn get_name(&self) -> String; // TODO: This might be temp because I want dynamic names.

// 	fn get_joints(&self) -> Vec<Rc<RefCell<Joint>>>; // TODO: Not final?
// 	fn try_attach_child(
// 		&mut self,
// 		tree: Box<dyn KinematicInterface>,
// 		joint_name: String,
// 		_joint_type: JointType,
// 	) -> Result<(), String>;

// 	// fn get_visual(&self) -> Vec<()>;
// 	// fn get_colliders(&self) -> Vec<()>;

// 	fn add_visual(&mut self, visual: Visual) -> Self;
// 	fn add_collider(&mut self, Collider: Collision) -> Self;
// }

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
	// }
	//
	// impl LinkTrait for Link {
	pub fn get_parent(&self) -> Option<LinkParent> {
		self.direct_parent.clone()
	}

	pub(crate) fn set_parent(&mut self, parent: LinkParent) {
		self.direct_parent = Some(parent);
		// TODO: Add yourself to registry.
		// Maybe that has already happend tho?
	}

	pub fn get_name(&self) -> String {
		self.name.clone()
	}

	pub fn get_joints(&self) -> Vec<Rc<RefCell<Joint>>> {
		self.child_joints
			.iter()
			.map(|joint| Rc::clone(joint))
			.collect()
	}

	///Maybe rename to try attach child
	pub fn try_attach_child(
		&mut self,
		tree: Box<dyn KinematicInterface>,
		joint_name: String,
		_joint_type: JointType,
	) -> Result<(), TryAttachChildError> {
		// TODO: NEEDS TO DO SOMETHING WITH JOINT TYPE
		let joint = Rc::new(RefCell::new(Joint {
			name: joint_name,
			tree: Weak::clone(&self.tree),
			parent_link: Weak::clone(
				self.tree
					.upgrade()
					.unwrap()
					.borrow()
					.links
					.borrow() // TODO: This might panic!
					.get(&self.get_name())
					.unwrap(),
			),
			child_link: tree.get_root_link(),
		}));

		self.child_joints.push(Rc::clone(&joint));

		// Maybe I can just go down the tree and add everything by hand for now? It sounds like a terrible Idea, let's do it!

		let parent_tree = self.tree.upgrade().unwrap();
		tree.get_root_link().borrow_mut().add_to_tree(&parent_tree);

		// parent_tree.try_merge(tree.get_kinematic_data())?;
		// Moved addign upwards

		let mut parent_tree = parent_tree.borrow_mut();
		parent_tree.try_add_link(tree.get_root_link())?;
		parent_tree.try_add_joint(joint)?;
		Ok(())
	}

	pub(crate) fn add_to_tree(&mut self, new_parent_tree: &Rc<RefCell<KinematicTreeData>>) {
		{
			let mut new_ptree = new_parent_tree.borrow_mut();
			self.child_joints
				.iter()
				.for_each(|joint| new_ptree.try_add_joint(Rc::clone(joint)).unwrap());
			// TODO: Add materials, and other stuff
		}
		self.child_joints.iter().for_each(|joint| {
			joint.borrow_mut().add_to_tree(new_parent_tree);
		});
		self.tree = Rc::downgrade(new_parent_tree);
	}

	pub fn add_visual(&mut self, visual: Visual) -> Self {
		todo!()
	}

	pub fn add_collider(&mut self, Collider: Collision) -> Self {
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

#[derive(Debug)]
pub enum TryAttachChildError {
	MergeTree(TryMergeTreeError),
	AddData(TryAddDataError),
}

impl fmt::Display for TryAttachChildError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			TryAttachChildError::MergeTree(err) => err.fmt(f),
			TryAttachChildError::AddData(err) => err.fmt(f),
		}
	}
}

impl Error for TryAttachChildError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			TryAttachChildError::MergeTree(err) => Some(err),
			TryAttachChildError::AddData(err) => Some(err),
		}
	}
}

impl From<TryMergeTreeError> for TryAttachChildError {
	fn from(value: TryMergeTreeError) -> Self {
		TryAttachChildError::MergeTree(value)
	}
}

impl From<TryAddDataError> for TryAttachChildError {
	fn from(value: TryAddDataError) -> Self {
		TryAttachChildError::AddData(value)
	}
}

#[cfg(test)]
mod tests {
	use super::Link;
	use crate::{cluster_objects::KinematicInterface, link::LinkParent};

	#[test]
	fn new() {
		let tree = Link::new("Link-on-Park".to_owned());

		let binding = tree.get_root_link();
		let root_link = binding.try_borrow().unwrap();
		assert_eq!(root_link.name, "Link-on-Park".to_owned());

		assert!(root_link.direct_parent.is_some());
		assert!({
			match root_link.direct_parent {
				Some(LinkParent::KinematicTree(_)) => true,
				_ => false,
			}
		});

		let newest_link = tree.get_newest_link().upgrade().unwrap();
		assert_eq!(
			newest_link.borrow().name,
			root_link.name
		);
		assert_eq!(
			newest_link.as_ptr(),
			binding.as_ptr()
		);

		assert_eq!(tree.get_links().try_borrow().unwrap().len(), 1);
		assert_eq!(tree.get_joints().try_borrow().unwrap().len(), 0);
	}
}
