mod cluster_objects;
mod joint;
mod link;
use joint::Joint;
use link::{Link, LinkTrait};

use std::{
	cell::RefCell,
	collections::HashMap,
	rc::{Rc, Weak},
};

// TODO: Maybe nest a seperate struct, in order to reuse some logic. it also would allow internal referencing without the use of an external Rc RefCell.
#[derive(Debug)]
pub struct Robot {
	name: String,
	pub root_link: Rc<RefCell<Link>>,
	//TODO: In this implementation the Keys, are not linked to the objects and could be changed.
	material_index: HashMap<String, Rc<RefCell<Material>>>,
	links: HashMap<String, Weak<RefCell<Link>>>,
	joints: HashMap<String, Weak<RefCell<Joint>>>,
	transmissions: HashMap<String, Rc<RefCell<Transmission>>>,
	// is_rigid: bool // ? For gazebo
}

impl Robot {
	//TODO: There is a posiblility for this to fail, do something with that
	pub fn new(name: String, root_link: Link) -> Rc<RefCell<Self>> {
		let mut root_link = Rc::new(RefCell::new(root_link));
		let mut material_index = HashMap::new();
		let mut links = HashMap::new();
		let mut joints = HashMap::new();
		let mut transmissions = HashMap::new();

		// Can unwrap here due to owning the only reference
		links.insert(
			root_link.borrow().get_name(),
			Rc::downgrade(&root_link.clone()),
		);

		let mut extra_links = Vec::new();

		for joint in root_link.borrow().get_joints() {
			if joints.contains_key(&joint.borrow().name) {
				panic!("Joint name not unique: {:?}", joint)
			}
			joints.insert(joint.borrow().name.clone(), Rc::downgrade(&joint));

			extra_links.push(Rc::clone(&joint.borrow().child_link));
			// let child_link = Rc::clone(&joint.borrow().child_link);

			// if links.contains_key(&child_link.borrow().name) {
			// 	panic!("link name not unique: {:?}", child_link)
			// }

			// links.insert(
			// 	child_link.borrow().name.clone(),
			// 	Rc::downgrade(&child_link)
			// );
		}

		//TODO: Add materials and possible joints to the robot.
		let robot = Rc::new(RefCell::new(Self {
			name,
			root_link,
			material_index,
			links,
			joints,
			transmissions,
		}));

		{
			robot
				.borrow()
				.root_link
				.try_borrow_mut()
				.unwrap()
				.set_parent(Rc::downgrade(&robot).into());
		}

		//TODO: Do something with extra links
		println!("NOT YET ADDED LINKS AND JOINTS: {:#?}", extra_links);

		robot
	}
}

impl PartialEq for Robot {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
			&& self.root_link == other.root_link
			&& self.material_index == other.material_index
			&& self.transmissions == other.transmissions
	}
}

//tmp
#[derive(Debug, PartialEq, Eq)]
pub struct Material {
	pub name: String,
}

//tmp
#[derive(Debug, PartialEq, Eq)]
struct Transmission;

pub fn add(left: usize, right: usize) -> usize {
	left + right
}

#[cfg(test)]
mod tests {
	use super::link::Link;
	use super::*;

	#[test]
	fn new() {
		let link = Link::new("test_link_1".to_owned());

		todo!("This test is very broken. 😢");
		// let robot = Robot::new("robot".to_owned(), link);
		// println!("{:?}", robot);
		// todo!()
	}

	#[test]
	fn it_works() {
		let result = add(2, 2);
		assert_eq!(result, 4);
	}
}
