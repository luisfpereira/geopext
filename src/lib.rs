#![feature(array_chunks)]

use std::collections::HashMap;

use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1, PyReadonlyArray2};

use pyo3::{pymodule, types::PyModule, Bound, PyResult, Python};

/// A Python module implemented in Rust.
#[pymodule]
fn geopext(m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[pyfn(m)]
    fn mesh_laplacian<'py>(
        py: Python<'py>,
        vertices: PyReadonlyArray2<'py, f64>,
        faces: PyReadonlyArray1<'py, usize>,
    ) -> (HashMap<(usize, usize), f64>, Bound<'py, PyArray1<f64>>) {
        let vertices = vertices.as_slice().unwrap();
        let faces = faces.as_slice().unwrap();

        let (laplace_matrix, areas) = rust_fn::mesh_laplacian_wrapper(vertices, &faces);

        (laplace_matrix, areas.into_pyarray_bound(py))
    }

    Ok(())
}

mod rust_fn {
    use geop::corner_table_from_vertices_and_indices;
    use geop::operator::Laplacian;

    use std::collections::HashMap;

    pub fn mesh_laplacian_wrapper(
        vertices: &[f64],
        faces: &[usize],
    ) -> (HashMap<(usize, usize), f64>, Vec<f64>) {
        let mesh = corner_table_from_vertices_and_indices(&vertices, &faces);

        (mesh.laplace_matrix(), mesh.mass_matrix())
    }
}
