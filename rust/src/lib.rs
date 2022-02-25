use pyo3::prelude::*;
use pyo3::py_run;

pub mod rez;

/// Creates a rustrez namespace hierarchy
///
/// Native modules in Python do not allow "from package.module import function"
/// See -> https://github.com/PyO3/pyo3/issues/759#issuecomment-583769162
///
/// Here we've used a workaround involving the sys.modules object.
/// See -> https://github.com/PyO3/pyo3/issues/1517#issuecomment-808664021
///
/// # Examples:
///
/// ```python
/// import _rez
/// _rez.vendor.version.version.Version()
/// ```
///
/// or
///
/// ```python
/// from _rez.vendor.version.version import Version
/// Version()
/// ```
///
#[pymodule]
fn _rez(py: Python, module: &PyModule) -> PyResult<()> {
    // Setup _rez.vendor namespace
    let vendor_subpackage = PyModule::new(py, "_rez.vendor")?;
    py_run!(
        py,
        vendor_subpackage,
        "import sys; sys.modules['_rez.vendor'] = vendor_subpackage"
    );
    module.add_submodule(vendor_subpackage)?;

    // Setup _rez.vendor.version namespace
    let version_subpackage = PyModule::new(py, "_rez.vendor.version")?;
    py_run!(
        py,
        version_subpackage,
        "import sys; sys.modules['_rez.vendor.version'] = version_subpackage"
    );
    module.add_submodule(version_subpackage)?;

    // Setup _rez.vendor.version.version namespace
    let version_submodule = PyModule::new(py, "_rez.vendor.version.version")?;
    py_run!(
        py,
        version_submodule,
        "import sys; sys.modules['_rez.vendor.version.version'] = version_submodule"
    );
    module.add_submodule(version_submodule)?;
    Ok(())
}
