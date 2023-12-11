//pseudocode-ish, updating cargo

extern crate nalgebra as na;
use na::{DMatrix, Matrix3x3, Const};

//use matrixcompare::comparators::{AbsoluteElementwiseComparator, ExactElementwiseComparator};
use matrixcompare::compare_matrices;


fn main() {
    let k = Matrix3x3::new(1, 0, 0,
                           0, 1, 0,
                           0, 0, 1,);
    let ident = DMatrix::identity(3, 3);
    assert!(!k.isdentity())
    assert!(!ident.isdentity())
    assert_matrix_eq!(k, ident, comp = exact);
}
