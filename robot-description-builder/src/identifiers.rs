//! TODO: GLOBAL DOCS
//!
//! TODO: FINISH DOCS OF THE MODULE
//!
//! # GroupID Delimiters
//! # TODO: ADD FORMATTING AND ESCAPED CHARACTER EXPLANATION

use std::fmt;

/// The delimiter used at the start of a [`GroupID`].
pub const DELIMITER_OPEN_GROUPID: &str = r"[[";
/// The delimiter used at the end of a [`GroupID`].
pub const DELIMITER_CLOSE_GROUPID: &str = r"]]";

/// The escaped delimiter, which gets converted to [`DELIMITER_OPEN_GROUPID`] when applied.
pub const DELIMITER_ESCAPED_OPEN_GROUPID: &str = r"[\[";
/// The escaped delimiter, which gets converted to [`DELIMITER_CLOSE_GROUPID`] when applied.
pub const DELIMITER_ESCAPED_CLOSE_GROUPID: &str = r"]\]";

/// Enum to store the various types of errors that can cause invalidation of a [`GroupID`].
///
/// # Important
/// When a validity check fails the error gets returned immediately,
/// meaning that if it fails for multiple reasons only the first one is provided.
/// This is the order the [`GroupID`] validity checks get performed:
/// 1. Check for [`DELIMITER_OPEN_GROUPID`] ([`ContainsOpen`](`GroupIDErrorKind::ContainsOpen`))
/// 2. Check for [`DELIMITER_CLOSE_GROUPID`] ([`ContainsClose`](`GroupIDErrorKind::ContainsClose`))
/// 3. Check if non-empty ([`Empty`](`GroupIDErrorKind::Empty`))
///
/// # Example
///
/// ```
/// # use robot_description_builder::identifiers::{GroupIDError, GroupID, GroupIDErrorKind};
/// if let Err(e) = GroupID::is_valid_group_id(&"[[ThisIsInvalid]]") {
///     println!("Invalid GroupID: {:?}", e.kind());
/// #   assert_eq!(e.kind(), &GroupIDErrorKind::ContainsOpen);
/// }
/// # else { unreachable!() }
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GroupIDErrorKind {
	/// `GroupID` being checked contains an unescaped opening `GroupID` delimiter.
	///
	/// This variant will be constructed when the [`GroupID`] being checked contains [`DELIMITER_OPEN_GROUPID`].
	ContainsOpen,
	/// `GroupID` being checked contains an unescaped closing `GroupID` delimiter.
	///
	/// This variant will be constructed when the [`GroupID`] being checked contains [`DELIMITER_CLOSE_GROUPID`].
	ContainsClose,
	/// `GroupID` being checked is empty.
	///
	/// This variant will be constructed when checking the [`GroupID`] validity of an empty string.
	Empty,
}

/// An error which can be returned when checking for a [`GroupID`]'s validity.
///
/// This error is used as an error type for functions which check for [`GroupID`] validity such as [`GroupID::is_valid_group_id`]/
///
/// # TODO: Potential causes ?
///
/// # Example
///
/// ```
/// # use robot_description_builder::identifiers::{GroupIDError, GroupID};
/// if let Err(e) = GroupID::is_valid_group_id(&"[[no]]") {
///     println!("Invalid GroupID: {e}");
/// }
/// # else { unreachable!() }
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GroupIDError {
	/// The invalid [`GroupID`]
	invalid_group_id: String,
	/// The reason why the [`GroupID`] is invalid
	pub(super) kind: GroupIDErrorKind,
}

impl GroupIDError {
	/// Creates a [`GroupIDError`] of kind [`GroupIDErrorKind::ContainsOpen`]
	pub(super) fn new_open(invalid_group_id: &str) -> Self {
		Self {
			invalid_group_id: invalid_group_id.to_string(),
			kind: GroupIDErrorKind::ContainsOpen,
		}
	}

	/// Creates a [`GroupIDError`] of kind [`GroupIDErrorKind::ContainsClose`]
	pub(super) fn new_close(invalid_group_id: &str) -> Self {
		Self {
			invalid_group_id: invalid_group_id.to_string(),
			kind: GroupIDErrorKind::ContainsClose,
		}
	}

	/// Creates a [`GroupIDError`] of kind [`GroupIDErrorKind::Empty`]
	pub(super) fn new_empty() -> Self {
		Self {
			invalid_group_id: String::new(),
			kind: GroupIDErrorKind::Empty,
		}
	}

	/// Returns a reference to a cloned [`String`] of the [`GroupID`], which caused the error.
	pub fn group_id(&self) -> &String {
		&self.invalid_group_id
	}

	/// Outputs the detailed cause of invalidation of the [`GroupID`].
	pub fn kind(&self) -> &GroupIDErrorKind {
		&self.kind
	}
}

impl fmt::Display for GroupIDError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self.kind {
			GroupIDErrorKind::ContainsOpen => write!(
				f,
				"invalid opening delimter (\"{}\") found in GroupID (\"{}\")",
				DELIMITER_OPEN_GROUPID, self.invalid_group_id
			),
			GroupIDErrorKind::ContainsClose => write!(
				f,
				"invalid closing delimiter (\"{}\") found in GroupID (\"{}\")",
				DELIMITER_CLOSE_GROUPID, self.invalid_group_id
			),
			GroupIDErrorKind::Empty => write!(f, "cannot change GroupID to empty string"),
		}
	}
}

impl std::error::Error for GroupIDError {}

/// Checks if the supplied `&str` is a valid [`GroupID`].
///
/// Returns a result type containing the valid `GroupID` as an `&str`, or an error with the invalidation reason.  
fn check_group_id_validity(new_group_id: &str) -> Result<&str, GroupIDError> {
	// !(new_group_id.contains(DELIMTER_OPEN_GROUPID)
	// 	|| new_group_id.contains(DELIMTER_CLOSE_GROUPID)
	// 	|| new_group_id.is_empty());

	// TODO: Maybe also check for '"'

	if new_group_id.contains(DELIMITER_OPEN_GROUPID) {
		Err(GroupIDError::new_open(new_group_id))
	} else if new_group_id.contains(DELIMITER_CLOSE_GROUPID) {
		Err(GroupIDError::new_close(new_group_id))
	} else if new_group_id.is_empty() {
		Err(GroupIDError::new_empty())
	} else {
		Ok(new_group_id)
	}
}

/// Replaces the [`GroupID`] delimiters in the supplied `&str`
///
/// `replace_group_id_delimiters` creates a new `String`, and copies the data from the provided string slice into it.
/// While doing so, it attempts to find matches of a the non-escaped and escapded `GroupID` delimiters.
/// If it finds any, it replaces them with the replacements specified below.
///
/// The following replacements get made:
///  - [`DELIMITER_OPEN_GROUPID`] with `""`
///  - [`DELIMITER_CLOSE_GROUPID`] with `""`
///  - [`DELIMITER_ESCAPED_OPEN_GROUPID`] with [`DELIMITER_OPEN_GROUPID`]
///  - [`DELIMITER_ESCAPED_CLOSE_GROUPID`] with [`DELIMITER_CLOSE_GROUPID`]
fn replace_group_id_delimiters(input: &str) -> String {
	input
		.replace(DELIMITER_OPEN_GROUPID, "")
		.replace(DELIMITER_CLOSE_GROUPID, "")
		.replace(DELIMITER_ESCAPED_OPEN_GROUPID, DELIMITER_OPEN_GROUPID)
		.replace(DELIMITER_ESCAPED_CLOSE_GROUPID, DELIMITER_CLOSE_GROUPID)
}

/// Format and validation trait for `GroupID`s
///
/// This trait is used to expand [`String`] and string slices for validity checks and `GroupID` escaped formatting applied
///
/// For more information on `GroupID` changing and escaping, see [the module-level documentation](`crate::identifiers`).
///
/// # TODO: Examples
/// TODO: Maybe skip examples for this one
pub trait GroupID {
	/// Checks if the current `GroupID` is Valid
	///
	/// If the current [`GroupID`] is valid, the [`GroupID`] get's returned as `Ok(&str)`.
	///
	/// Otherwise an error of type [`GroupIDError`] is returned describing why the current [`GroupID`] is invalid.  
	fn is_valid_group_id(&self) -> Result<&str, GroupIDError>;

	/// Return an cloned `String` of the current `GroupID` with the delimiters replaced.
	///
	/// TODO: UPGRADE OR REFERENCE ANOTHER DOCUMENTATION TO PREVENT DISCRAPENCIES BETWEEN DOCS
	///
	/// Returns a cloned [`String`] with the following replacements:
	///  - [`DELIMITER_OPEN_GROUPID`] with `""`
	///  - [`DELIMITER_CLOSE_GROUPID`] with `""`
	///  - [`DELIMITER_ESCAPED_OPEN_GROUPID`] with [`DELIMITER_OPEN_GROUPID`]
	///  - [`DELIMITER_ESCAPED_CLOSE_GROUPID`] with [`DELIMITER_CLOSE_GROUPID`]
	fn display(&self) -> String;

	/// Maybe wrong place. TODO: Consider moving to `GroupIDChanger`
	///
	/// TODO:
	/// - Move?
	/// - Document
	/// - Test
	fn get_group_id(&self) -> Option<&str>;
}

impl GroupID for String {
	fn is_valid_group_id(&self) -> Result<&str, GroupIDError> {
		check_group_id_validity(self)
	}

	fn display(&self) -> String {
		replace_group_id_delimiters(self)
	}

	/// Maybe wrong place. TODO: Consider moving to `GroupIDChanger`
	fn get_group_id(&self) -> Option<&str> {
		self.split_once(DELIMITER_OPEN_GROUPID)
			.and_then(|(_, near_group_id)| {
				near_group_id
					.rsplit_once(DELIMITER_CLOSE_GROUPID)
					.map(|(group_id, _)| group_id)
			})
	}
}

impl GroupID for &str {
	fn is_valid_group_id(&self) -> Result<&str, GroupIDError> {
		check_group_id_validity(self)
	}

	fn display(&self) -> String {
		replace_group_id_delimiters(self)
	}

	/// Maybe wrong place. TODO: Consider moving to `GroupIDChanger`
	fn get_group_id(&self) -> Option<&str> {
		self.split_once(DELIMITER_OPEN_GROUPID)
			.and_then(|(_, near_group_id)| {
				near_group_id
					.rsplit_once(DELIMITER_CLOSE_GROUPID)
					.map(|(group_id, _)| group_id)
			})
	}
}

/// Used for `GroupID` modifications on buildertrees.
///
/// Implementing this trait allows for the modification of identification string (often `name` field) of its implementor and his children.
///
/// The following operations can be done:
///  - Replacing the [`GroupID`] section of the identification string.
///  - Appling the [`GroupID` delimiter transformations](crate::identifiers#groupid-delimiters)
///
/// This should be achieved by recursively calling the desired method on the children of the implementor.
///
/// # Examples
///
/// Impemtation of `GroupIDChanger` for on an example struct tree:
///
/// ```
/// use robot_description_builder::identifiers::{GroupIDChanger,GroupIDErrorKind};
///
/// #[derive(Debug, PartialEq, Eq, Clone)]
/// struct ChildStruct {
///     name: String
/// }
///
/// impl GroupIDChanger for ChildStruct {
///     unsafe fn change_group_id_unchecked(&mut self, new_group_id: &str) {
///         self.name.change_group_id_unchecked(new_group_id);
///     }
///
///     fn apply_group_id(&mut self) {
///         self.name.apply_group_id();
///     }
/// }
///
/// #[derive(Debug, PartialEq, Eq, Clone)]
/// struct ParentStruct {
///     name: String,
///     child: Option<ChildStruct>
/// }
///
/// impl GroupIDChanger for ParentStruct {
///     unsafe fn change_group_id_unchecked(&mut self, new_group_id: &str) {
///         self.name.change_group_id_unchecked(new_group_id);
///         if let Some(child) = self.child.as_mut() {
///             child.change_group_id_unchecked(new_group_id);
///         }
///     }
///
///     fn apply_group_id(&mut self) {
///         self.name.apply_group_id();
///         if let Some(child) = self.child.as_mut() {
///             child.apply_group_id();
///         }
///     }
/// }
///
/// let example_tree = ParentStruct{
///         name: "tree_[[0]]".into(),
///         child: Some(ChildStruct{name:"tree_child_[[0]][\\[".into()})
///     };
///
/// // Appling a GroupID
/// let mut applied_tree = example_tree.clone();
/// applied_tree.apply_group_id();
/// assert_eq!(
///     applied_tree,
///     ParentStruct{
///         name: "tree_0".into(),
///         child: Some(ChildStruct{name:"tree_child_0[[".into()})
///     }
/// );
///
/// // Changing the GroupID
/// let mut changed_tree = example_tree.clone();
/// assert!(changed_tree.change_group_id("1").is_ok());
/// assert_eq!(
///     changed_tree,
///     ParentStruct{
///         name: "tree_[[1]]".into(),
///         child: Some(ChildStruct{name:"tree_child_[[1]][\\[".into()})
///     }
/// );
///
/// // Invalid GroupID
/// let mut failed_tree = example_tree.clone();
/// assert_eq!(changed_tree.change_group_id("").unwrap_err().kind(), &GroupIDErrorKind::Empty);
/// // The tree remains unchanged
/// assert_eq!(failed_tree, example_tree);
/// ```
pub trait GroupIDChanger {
	/// Replaces the `GroupID` of the builder tree with `new_group_id`.
	///
	/// If `new_group_id` is a valid [`GroupID`] then the `GroupID` of the whole buildertree is replaced.
	/// Otherwise, this method fails returning an error explaing the invalidation.
	///
	/// For performance reasons the check only get's performed here,
	/// when this succeeds [`change_group_id_unchecked`][GroupIDChanger::change_group_id_unchecked] is used to perform the actual updating.
	fn change_group_id(&mut self, new_group_id: impl GroupID) -> Result<(), GroupIDError> {
		unsafe {
			Self::change_group_id_unchecked(self, new_group_id.is_valid_group_id()?);
		};
		Ok(())
	}

	/// Unchecked replacement of the `GroupID` of the builder tree with `new_group_id`.
	///
	/// Changes the [`GroupID`] of the identification string of the current builder tree without checking if the `new_group_id` is valid.
	/// This should be achieved by calling this method on all its implementors childeren and its identification string often called `name`.
	///
	/// # Safety
	///
	/// This function should be called with a valid [`GroupID`].
	/// It is recommended to use [`change_group_id`](GroupIDChanger::change_group_id) instead.
	unsafe fn change_group_id_unchecked(&mut self, new_group_id: &str);

	/// Applies `GroupID` delimiter replacements.
	///
	/// Replaces the [`GroupID`] delimiters in the current builder tree.
	///
	/// TODO: REFERENCE MODULE DOC ABOUT GroupID Delimiters and replacements
	///
	/// -----
	/// TODO: UPGRADE
	///
	/// Replaces:
	///  - [`DELIMITER_OPEN_GROUPID`] with `""`
	///  - [`DELIMITER_CLOSE_GROUPID`] with `""`
	///  - [`DELIMITER_ESCAPED_OPEN_GROUPID`] with [`DELIMITER_OPEN_GROUPID`]
	///  - [`DELIMITER_ESCAPED_CLOSE_GROUPID`] with [`DELIMITER_CLOSE_GROUPID`]
	fn apply_group_id(&mut self);
}

impl GroupIDChanger for String {
	unsafe fn change_group_id_unchecked(&mut self, new_group_id: &str) {
		if self.matches(DELIMITER_OPEN_GROUPID).count() == 1
			&& self.matches(DELIMITER_CLOSE_GROUPID).count() == 1
		{
			if let Some((pre, _, post)) =
				self.split_once(DELIMITER_OPEN_GROUPID)
					.and_then(|(pre, remainder)| {
						remainder
							.split_once(DELIMITER_CLOSE_GROUPID)
							.map(|(group_id, post)| (pre, group_id, post))
					}) {
				let new = format!(
					"{}{}{}{}{}",
					pre, DELIMITER_OPEN_GROUPID, new_group_id, DELIMITER_CLOSE_GROUPID, post
				);

				#[cfg(any(feature = "logging", test))]
				log::info!(
					target: "GroupIDChanger",
					"The identification string \"{}\" was replaced by \"{}\"",
					self, new
				);

				*self = new;
			}
		} else {
			#[cfg(any(feature = "logging", test))]
			log::info!(
				target: "GroupIDChanger",
				"The changing of the GroupID of \"{}\" was skipped due to not having exactly 1 opening and 1 closing delimiter",
				self
			);
		}
	}

	fn apply_group_id(&mut self) {
		// Maybe checking is uncessesary
		let open_count = self.matches(DELIMITER_OPEN_GROUPID).count();
		let close_count = self.matches(DELIMITER_CLOSE_GROUPID).count();

		if (open_count == 1 && close_count == 1) || (open_count == 0 && close_count == 0) {
			let new = Self::display(self);

			#[cfg(any(feature = "logging", test))]
			log::info!(
				target: "GroupIDChanger",
				"Applied GroupID delimiter transformations to \"{}\", changed to \"{}\"",
				self, new
			);

			*self = new;
		} else {
			#[cfg(any(feature = "logging", test))]
			log::info!(
				target: "GroupIDChanger",
				"The GroupID delimiters transformations where not applied to \"{}\", because {}",
				self,
				match (open_count, close_count) {
					(0, 0) | (1, 1) => unreachable!(),
					(1, 0) => format!("of an unclosed GroupID field. (missing \"{DELIMITER_CLOSE_GROUPID}\")"),
					(0, 1) => format!("of an unopened GroupID field. (missing \"{DELIMITER_OPEN_GROUPID}\")"),
					(0 | 1, _) => format!("of excess closing delimeters (\"{DELIMITER_CLOSE_GROUPID}\"), expected {open_count} closing tags based on amount of opening tags, got {close_count} closing tags"),
					(_, 0 | 1) => format!("of excess opening delimeters (\"{DELIMITER_OPEN_GROUPID}\"), expected {close_count} opening tags based on amount of closing tags, got {open_count} opening tags"),
					(_, _) => format!("of unexpected amount of opening and closing tags, got (Open, close) = ({open_count}, {close_count}), expected (0, 0) or (1, 1)")
				}
			);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{
		check_group_id_validity, replace_group_id_delimiters, GroupIDError, GroupIDErrorKind,
		DELIMITER_ESCAPED_CLOSE_GROUPID, DELIMITER_ESCAPED_OPEN_GROUPID,
	};
	use test_log::test;

	#[test]
	fn test_check_group_id_validity() {
		assert_eq!(
			check_group_id_validity("[[---"),
			Err(GroupIDError {
				invalid_group_id: "[[---".to_string(),
				kind: GroupIDErrorKind::ContainsOpen
			})
		);

		assert_eq!(
			check_group_id_validity("smiley? :]]"),
			Err(GroupIDError {
				invalid_group_id: "smiley? :]]".to_string(),
				kind: GroupIDErrorKind::ContainsClose
			})
		);

		assert_eq!(
			check_group_id_validity(""),
			Err(GroupIDError {
				invalid_group_id: String::new(),
				kind: GroupIDErrorKind::Empty
			})
		);

		assert_eq!(check_group_id_validity("L02"), Ok("L02"));
		assert_eq!(check_group_id_validity("left_arm"), Ok("left_arm"));
		assert_eq!(
			check_group_id_validity(&String::from("Left[4]")),
			Ok("Left[4]")
		);
		assert_eq!(
			check_group_id_validity(&format!(
				"Right{}99999999999999{}_final_count_down",
				DELIMITER_ESCAPED_OPEN_GROUPID, DELIMITER_ESCAPED_CLOSE_GROUPID
			)),
			Ok(r#"Right[\[99999999999999]\]_final_count_down"#)
		);
	}

	#[test]
	fn test_replace_group_id_delimiters() {
		assert_eq!(replace_group_id_delimiters("nothing"), "nothing");

		// Delimiters
		assert_eq!(
			replace_group_id_delimiters("[[Hopefully Not Hidden]]"),
			"Hopefully Not Hidden"
		);
		assert_eq!(replace_group_id_delimiters("colo[[[u]]]r"), "colo[u]r");
		assert_eq!(
			replace_group_id_delimiters("Before[[[[Anything]]]]After"),
			"BeforeAnythingAfter"
		);

		// Escaped
		assert_eq!(
			replace_group_id_delimiters("Obsidian Internal Link [\\[Anything]\\]"),
			"Obsidian Internal Link [[Anything]]"
		);
		assert_eq!(
			replace_group_id_delimiters("Front[\\[:[\\[Center]\\]:]\\]Back"),
			"Front[[:[[Center]]:]]Back"
		);

		// Mixed
		assert_eq!(
			replace_group_id_delimiters("multi_groupid_Leg_[\\[L04]\\]_Claw_[[L01]]"),
			"multi_groupid_Leg_[[L04]]_Claw_L01"
		);
	}

	mod group_id {
		use super::{test, DELIMITER_ESCAPED_CLOSE_GROUPID, DELIMITER_ESCAPED_OPEN_GROUPID};
		use crate::identifiers::{GroupID, GroupIDError, GroupIDErrorKind};

		#[test]
		/// GroupID::is_valid_group_id()
		fn is_valid_group_id() {
			assert_eq!(
				"[[---".is_valid_group_id(),
				Err(GroupIDError {
					invalid_group_id: "[[---".to_string(),
					kind: GroupIDErrorKind::ContainsOpen
				})
			);

			assert_eq!(
				"smiley? :]]".is_valid_group_id(),
				Err(GroupIDError {
					invalid_group_id: "smiley? :]]".to_string(),
					kind: GroupIDErrorKind::ContainsClose
				})
			);

			assert_eq!(
				"".is_valid_group_id(),
				Err(GroupIDError {
					invalid_group_id: String::new(),
					kind: GroupIDErrorKind::Empty
				})
			);

			assert_eq!("L02".is_valid_group_id(), Ok("L02"));
			assert_eq!("left_arm".is_valid_group_id(), Ok("left_arm"));
			assert_eq!("Left[4]".is_valid_group_id(), Ok("Left[4]"));
			assert_eq!(
				format!(
					"Right{}99999999999999{}_final_count_down",
					DELIMITER_ESCAPED_OPEN_GROUPID, DELIMITER_ESCAPED_CLOSE_GROUPID
				)
				.is_valid_group_id(),
				Ok(r#"Right[\[99999999999999]\]_final_count_down"#)
			);
		}

		#[test]
		fn display() {
			assert_eq!("nothing".display(), "nothing");

			// Delimiters
			assert_eq!("[[Hopefully Not Hidden]]".display(), "Hopefully Not Hidden");
			assert_eq!("colo[[[u]]]r".display(), "colo[u]r");
			assert_eq!("colo[[[u]]]r".to_string().display(), "colo[u]r");
			assert_eq!(
				"Before[[[[Anything]]]]After".display(),
				"BeforeAnythingAfter"
			);

			// Escaped
			assert_eq!(
				"Obsidian Internal Link [\\[Anything]\\]".display(),
				"Obsidian Internal Link [[Anything]]"
			);
			assert_eq!(
				"Front[\\[:[\\[Center]\\]:]\\]Back".display(),
				"Front[[:[[Center]]:]]Back"
			);

			// Mixed
			assert_eq!(
				"multi_groupid_Leg_[\\[L04]\\]_Claw_[[L01]]".display(),
				"multi_groupid_Leg_[[L04]]_Claw_L01"
			);
			// Mixed
			assert_eq!(
				"multi_groupid_Leg_[\\[L04]\\]_Claw_[[L01]]"
					.to_string()
					.display(),
				"multi_groupid_Leg_[[L04]]_Claw_L01"
			);
		}
	}

	mod group_id_changer {
		use super::test;
		use crate::identifiers::{GroupIDChanger, GroupIDError, GroupIDErrorKind};

		fn test_change_group_id_unchecked(s: impl Into<String>, new_group_id: &str, result: &str) {
			let mut s: String = s.into();
			unsafe {
				s.change_group_id_unchecked(new_group_id);
			}
			assert_eq!(s, result)
		}

		#[test]
		fn change_group_id_unchecked() {
			test_change_group_id_unchecked("nothing", "R02", "nothing");

			// Delimiters
			test_change_group_id_unchecked("[[Hopefully Not Hidden]]", "R02", "[[R02]]");
			test_change_group_id_unchecked("colo[[[u]]]r", "u", "colo[[u]]]r");
			test_change_group_id_unchecked(
				// TODO: Is this final behavior?
				"Before[[[[Anything]]]]After",
				"Sunrise",
				"Before[[[[Anything]]]]After", // "BeforeSunriseAfter",
			);

			// Escaped
			test_change_group_id_unchecked(
				"Obsidian Internal Link [\\[Anything]\\]",
				".....",
				"Obsidian Internal Link [\\[Anything]\\]",
			);
			test_change_group_id_unchecked(
				"Front[\\[:[\\[Center]\\]:]\\]Back",
				".....",
				"Front[\\[:[\\[Center]\\]:]\\]Back",
			);

			// Mixed
			test_change_group_id_unchecked(
				"multi_groupid_Leg_[\\[L04]\\]_Claw_[[L01]]",
				"R09",
				"multi_groupid_Leg_[\\[L04]\\]_Claw_[[R09]]",
			);
			test_change_group_id_unchecked(
				"Front[\\[:[[Center]]:]\\]Back",
				"Middle",
				"Front[\\[:[[Middle]]:]\\]Back",
			);

			// UNCHECKED BEHAVIOR
			test_change_group_id_unchecked(
				"multi_groupid_Leg_[\\[L04]\\]_Claw_[[L01]]",
				"[[R08]]",
				"multi_groupid_Leg_[\\[L04]\\]_Claw_[[[[R08]]]]",
			);
			test_change_group_id_unchecked(
				"Front[\\[:[[Center]]:]\\]Back",
				"",
				"Front[\\[:[[]]:]\\]Back",
			);
		}

		fn test_change_group_id(
			s: impl Into<String>,
			new_group_id: &str,
			func_result: Result<(), GroupIDError>,
			new_identifier: &str,
		) {
			let mut s: String = s.into();
			assert_eq!(s.change_group_id(new_group_id), func_result);
			assert_eq!(s, new_identifier);
		}

		#[test]
		fn change_group_id() {
			test_change_group_id("nothing", "R02", Ok(()), "nothing");

			// Delimiters
			test_change_group_id("[[Hopefully Not Hidden]]", "R02", Ok(()), "[[R02]]");
			test_change_group_id("colo[[[u]]]r", "u", Ok(()), "colo[[u]]]r");
			test_change_group_id(
				// TODO: Is this final behavior?
				"Before[[[[Anything]]]]After",
				"Sunrise",
				Ok(()),
				"Before[[[[Anything]]]]After", // "BeforeSunriseAfter",
			);

			// Escaped
			test_change_group_id(
				"Obsidian Internal Link [\\[Anything]\\]",
				".....",
				Ok(()),
				"Obsidian Internal Link [\\[Anything]\\]",
			);
			test_change_group_id(
				"Front[\\[:[\\[Center]\\]:]\\]Back",
				".....",
				Ok(()),
				"Front[\\[:[\\[Center]\\]:]\\]Back",
			);

			// Mixed
			test_change_group_id(
				"multi_groupid_Leg_[\\[L04]\\]_Claw_[[L01]]",
				"R09",
				Ok(()),
				"multi_groupid_Leg_[\\[L04]\\]_Claw_[[R09]]",
			);
			test_change_group_id(
				"Front[\\[:[[Center]]:]\\]Back",
				"Middle",
				Ok(()),
				"Front[\\[:[[Middle]]:]\\]Back",
			);

			// UNCHECKED BEHAVIOR
			test_change_group_id(
				"multi_groupid_Leg_[\\[L04]\\]_Claw_[[L01]]",
				"[[R08]]",
				Err(GroupIDError {
					invalid_group_id: "[[R08]]".into(),
					kind: GroupIDErrorKind::ContainsOpen,
				}),
				"multi_groupid_Leg_[\\[L04]\\]_Claw_[[L01]]",
			);
			test_change_group_id(
				"Front[\\[:[[Center]]:]\\]Back",
				"",
				Err(GroupIDError {
					invalid_group_id: String::new(),
					kind: GroupIDErrorKind::Empty,
				}),
				"Front[\\[:[[Center]]:]\\]Back",
			);
		}

		fn test_apply_group_id(s: impl Into<String>, result: &str) {
			let mut s: String = s.into();
			s.apply_group_id();
			assert_eq!(s, result);
		}

		#[test]
		fn apply_group_id() {
			test_apply_group_id("nothing", "nothing");

			// Delimiters
			test_apply_group_id("[[Hopefully Not Hidden]]", "Hopefully Not Hidden");
			test_apply_group_id("colo[[[u]]]r", "colo[u]r");
			test_apply_group_id(
				// TODO: Is this final behavior?
				"Before[[[[Anything]]]]After",
				"Before[[[[Anything]]]]After",
			);

			// Escaped
			test_apply_group_id(
				"Obsidian Internal Link [\\[Anything]\\]",
				"Obsidian Internal Link [[Anything]]",
			);
			test_apply_group_id(
				"Front[\\[:[\\[Center]\\]:]\\]Back",
				"Front[[:[[Center]]:]]Back",
			);

			// Mixed
			test_apply_group_id(
				"multi_groupid_Leg_[\\[L04]\\]_Claw_[[L01]]",
				"multi_groupid_Leg_[[L04]]_Claw_L01",
			);
			test_apply_group_id("Front[\\[:[[Center]]:]\\]Back", "Front[[:Center:]]Back");

			test_apply_group_id(
				"multi_groupid_Leg_[\\[L04]\\]]_Claw_[[L01]]",
				"multi_groupid_Leg_[\\[L04]\\]]_Claw_[[L01]]",
			);
		}
	}
}
