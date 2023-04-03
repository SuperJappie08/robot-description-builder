#[derive(Debug, PartialEq, Clone, Copy, Default)]
/// TODO: maybe change visibilty
pub struct CalibrationData {
	pub rising: Option<f32>,
	pub falling: Option<f32>,
}

impl CalibrationData {
	/// A function to check if any of the fields are set.
	///
	/// It doesn't check if the some fields have the default value, since it can be format depended.
	///
	/// ## Example
	/// ```--rust
	/// # use rdf_builder_rs::joint::joint_data::CalibrationData;
	/// assert!(CalibrationData {
	///     rising: Some(1.),
	///     falling: Some(2.)
	/// }
	/// .contains_some());
	///
	/// assert!(CalibrationData {
	///     rising: Some(1.),
	///     ..Default::default()
	/// }
	/// .contains_some());
	///
	/// assert!(CalibrationData {
	///     falling: Some(2.),
	///     ..Default::default()
	/// }
	/// .contains_some());
	///
	/// assert!(!CalibrationData::default().contains_some())
	/// ```
	pub fn contains_some(&self) -> bool {
		self.rising.is_some() || self.falling.is_some()
	}
}

#[cfg(test)]
mod tests {
	use crate::joint::joint_data::calibration_data::CalibrationData;

	#[test]
	fn contains_some() {
		assert!(CalibrationData {
			rising: Some(1.),
			falling: Some(2.)
		}
		.contains_some());

		assert!(CalibrationData {
			rising: Some(1.),
			..Default::default()
		}
		.contains_some());

		assert!(CalibrationData {
			falling: Some(2.),
			..Default::default()
		}
		.contains_some());

		assert!(!CalibrationData::default().contains_some())
	}
}
