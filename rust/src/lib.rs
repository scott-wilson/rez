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
/// import rustrez
/// rustrez.vendor.version.version.Version()
/// ```
///
/// or
///
/// ```python
/// from rustrez.vendor.version.version import Version
/// Version()
/// ```
///
#[pymodule]
fn rustrez(py: Python, module: &PyModule) -> PyResult<()> {
    // Setup rustrez.vendor namespace
    let vendor_subpackage = PyModule::new(py, "rustrez.vendor")?;
    py_run!(
        py,
        vendor_subpackage,
        "import sys; sys.modules['rustrez.vendor'] = vendor_subpackage"
    );
    module.add_submodule(vendor_subpackage)?;

    // Setup rustrez.vendor.version namespace
    let version_subpackage = PyModule::new(py, "rustrez.vendor.version")?;
    py_run!(
        py,
        version_subpackage,
        "import sys; sys.modules['rustrez.vendor.version'] = version_subpackage"
    );
    module.add_submodule(version_subpackage)?;

    // Setup rustrez.vendor.version.version namespace
    let version_submodule = PyModule::new(py, "rustrez.vendor.version.version")?;
    py_run!(
        py,
        version_submodule,
        "import sys; sys.modules['rustrez.vendor.version.version'] = version_submodule"
    );
    version_submodule.add_class::<rez::version::version::Version>()?;
    module.add_submodule(version_submodule)?;
    Ok(())
}
