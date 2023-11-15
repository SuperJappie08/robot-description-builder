mod to_urdf;

use pyo3::{exceptions::PyTypeError, intern, prelude::*, types::PyDict};

use robot_description_builder::to_rdf::XMLMode;

use to_urdf::to_urdf_string;

pub(super) fn init_module(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
	module.add_function(wrap_pyfunction!(to_urdf_string, module)?)?;

	Ok(())
}

/// Takes `'indent'` argument from a `&PyDict` to extract a `XMLMode`
fn dict2xmlmode(py: Python<'_>, kwds: &PyDict) -> PyResult<XMLMode> {
	if let Some(indent) = kwds.get_item(intern!(py, "indent"))? {
		if let Ok((c, count)) = indent.extract::<(char, usize)>() {
			// Char and count so indentation
			kwds.del_item(intern!(py, "indent"))?;
			return Ok(XMLMode::Indent(c, count));
		} else if indent.is_none() {
			// Specified as None so no indentation
			kwds.del_item(intern!(py, "indent"))?;
			return Ok(XMLMode::NoIndent);
		} else {
			// Unexpected type
			return Err(PyTypeError::new_err(format!(
				"Could not convert '{}' into '{}'",
				indent.repr()?,
				if py.version_info() >= (3, 9) {
					"tuple[char,+int]|None"
				} else {
					"Optional[Tuple[char,+int]]"
				}
			)));
		}
	}

	// Nothing specified so default
	Ok(XMLMode::default())
}
