#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct DynamicsData {
	pub damping: Option<f32>,
	pub friction: Option<f32>,
}

impl DynamicsData {
	/// A function to check if any of the fields are set.
	///
	/// It doesn't check if the some fields have the default value, since it can be format depended.
	///
	/// ## Example
	/// ```--rust
	/// # use rdf_builder_rs::joint::joint_data::DynamicsData;
	/// assert!(DynamicsData {
	///     damping: Some(1.),
	///     friction: Some(2.)
	/// }
	/// .contains_some());
	///
	/// assert!(DynamicsData {
	///     damping: Some(1.),
	///     ..Default::default()
	/// }
	/// .contains_some());
	///
	/// assert!(DynamicsData {
	///     friction: Some(2.),
	///     ..Default::default()
	/// }
	/// .contains_some());
	///
	/// assert!(!DynamicsData::default().contains_some())
	/// ```
	pub fn contains_some(&self) -> bool {
		self.damping.is_some() || self.friction.is_some()
	}
}

#[cfg(test)]
mod tests {
	use crate::joint::joint_data::DynamicsData;

	#[test]
	fn contains_some() {
		assert!(DynamicsData {
			damping: Some(1.),
			friction: Some(2.)
		}
		.contains_some());

		assert!(DynamicsData {
			damping: Some(1.),
			..Default::default()
		}
		.contains_some());

		assert!(DynamicsData {
			friction: Some(2.),
			..Default::default()
		}
		.contains_some());

		assert!(!DynamicsData::default().contains_some())
	}
}
