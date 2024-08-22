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
        ds: &str,
    ) -> (HashMap<(usize, usize), f64>, Bound<'py, PyArray1<f64>>) {
        let vertices = vertices.as_slice().unwrap();
        let faces = faces.as_slice().unwrap();

        let ds = match ds {
            "corner_table" => rust_fn::DS::CornerTable,
            "half_edge" => rust_fn::DS::HalfEdge,
            _ => panic!("Unkown data structure."),
        };

        let (laplace_matrix, areas) = rust_fn::mesh_laplacian(vertices, faces, ds);

        (laplace_matrix, areas.into_pyarray_bound(py))
    }

    Ok(())
}

mod rust_fn {
    use baby_shark::mesh::corner_table::table::CornerTable;
    use geop::ds::FromSharedVertex;
    use geop::ds::HalfEdgeMesh;
    use geop::op::Laplacian;
    use std::collections::HashMap;

    pub enum DS {
        CornerTable,
        HalfEdge,
    }

    pub fn mesh_laplacian(
        vertices: &[f64],
        faces: &[usize],
        ds: DS,
    ) -> (HashMap<(usize, usize), f64>, Vec<f64>) {
        return match ds {
            DS::CornerTable => mesh_laplacian_corner_table(&vertices, &faces),
            DS::HalfEdge => mesh_laplacian_half_edge(&vertices, &faces),
        };
    }

    pub fn mesh_laplacian_corner_table(
        vertices: &[f64],
        faces: &[usize],
    ) -> (HashMap<(usize, usize), f64>, Vec<f64>) {
        let mesh = CornerTable::from_vertices_and_faces(&vertices, &faces);

        (mesh.laplace_matrix(), mesh.mass_matrix())
    }

    pub fn mesh_laplacian_half_edge(
        vertices: &[f64],
        faces: &[usize],
    ) -> (HashMap<(usize, usize), f64>, Vec<f64>) {
        let mesh = HalfEdgeMesh::from_vertices_and_faces(&vertices, &faces);

        (mesh.laplace_matrix(), mesh.mass_matrix())
    }
}
