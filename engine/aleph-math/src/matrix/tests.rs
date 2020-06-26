//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::traits::{Column, Inverse, InverseAssign};
use crate::types::{Mat4x4, Vec4};

#[test]
pub fn matrix_creation_1() {
    let m1 = Mat4x4::new([
        0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
    ]);

    let col0 = m1.get_column(0);

    let col1 = m1.get_column(1);

    let col2 = m1.get_column(2);

    let col3 = m1.get_column(3);

    assert_eq!(&col0, &Vec4::from([0.0, 1.0, 2.0, 3.0]));
    assert_eq!(&col1, &Vec4::from([4.0, 5.0, 6.0, 7.0]));
    assert_eq!(&col2, &Vec4::from([8.0, 9.0, 10.0, 11.0]));
    assert_eq!(&col3, &Vec4::from([12.0, 13.0, 14.0, 15.0]));

    assert_eq!(6.0, m1[(2, 1)]);
    assert_eq!(9.0, m1[(1, 2)]);

    assert_eq!(0.0, m1[(0, 0)]);
    assert_eq!(15.0, m1[(3, 3)]);

    assert_eq!(1.0, m1[(1, 0)]);
    assert_eq!(13.0, m1[(1, 3)]);

    assert_eq!(7.0, m1[(3, 1)]);
    assert_eq!(14.0, m1[(2, 3)]);
}

#[test]
pub fn matrix_creation_2() {
    let m1 = Mat4x4::new([
        0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 21.0,
    ]);

    let col0 = m1.get_column(0);

    let col1 = m1.get_column(1);

    let col2 = m1.get_column(2);

    let col3 = m1.get_column(3);

    assert_eq!(&col0, &Vec4::from([0.0, 1.0, 2.0, 3.0]));
    assert_eq!(&col1, &Vec4::from([4.0, 5.0, 6.0, 7.0]));
    assert_eq!(&col2, &Vec4::from([8.0, 9.0, 10.0, 11.0]));
    assert_eq!(&col3, &Vec4::from([12.0, 13.0, 14.0, 21.0]));

    assert_eq!(6.0, m1[(2, 1)]);
    assert_eq!(9.0, m1[(1, 2)]);

    assert_eq!(0.0, m1[(0, 0)]);
    assert_eq!(21.0, m1[(3, 3)]);

    assert_eq!(1.0, m1[(1, 0)]);
    assert_eq!(13.0, m1[(1, 3)]);

    assert_eq!(7.0, m1[(3, 1)]);
    assert_eq!(14.0, m1[(2, 3)]);
}

#[test]
pub fn matrix_add_1() {
    let m1 = Mat4x4::new([
        0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
    ]);
    let m2 = Mat4x4::new([
        0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
    ]);

    {
        let col0 = m1.get_column(0);
        let col1 = m1.get_column(1);
        let col2 = m1.get_column(2);
        let col3 = m1.get_column(3);

        assert_eq!(&col0, &Vec4::from([0.0, 1.0, 2.0, 3.0]));
        assert_eq!(&col1, &Vec4::from([4.0, 5.0, 6.0, 7.0]));
        assert_eq!(&col2, &Vec4::from([8.0, 9.0, 10.0, 11.0]));
        assert_eq!(&col3, &Vec4::from([12.0, 13.0, 14.0, 15.0]));
    }

    {
        let col0 = m2.get_column(0);
        let col1 = m2.get_column(1);
        let col2 = m2.get_column(2);
        let col3 = m2.get_column(3);

        assert_eq!(&col0, &Vec4::from([0.0, 1.0, 2.0, 3.0]));
        assert_eq!(&col1, &Vec4::from([4.0, 5.0, 6.0, 7.0]));
        assert_eq!(&col2, &Vec4::from([8.0, 9.0, 10.0, 11.0]));
        assert_eq!(&col3, &Vec4::from([12.0, 13.0, 14.0, 15.0]));
    }

    assert_eq!(6.0, m1[(2, 1)]);
    assert_eq!(9.0, m1[(1, 2)]);

    assert_eq!(0.0, m1[(0, 0)]);
    assert_eq!(15.0, m1[(3, 3)]);

    assert_eq!(1.0, m1[(1, 0)]);
    assert_eq!(13.0, m1[(1, 3)]);

    assert_eq!(7.0, m1[(3, 1)]);
    assert_eq!(14.0, m1[(2, 3)]);

    assert_eq!(6.0, m2[(2, 1)]);
    assert_eq!(9.0, m2[(1, 2)]);

    assert_eq!(0.0, m2[(0, 0)]);
    assert_eq!(15.0, m2[(3, 3)]);

    assert_eq!(1.0, m2[(1, 0)]);
    assert_eq!(13.0, m2[(1, 3)]);

    assert_eq!(7.0, m2[(3, 1)]);
    assert_eq!(14.0, m2[(2, 3)]);

    let m = m1 + m2;

    assert_eq!(6.0 * 2.0, m[(2, 1)]);
    assert_eq!(9.0 * 2.0, m[(1, 2)]);

    assert_eq!(0.0 * 2.0, m[(0, 0)]);
    assert_eq!(15.0 * 2.0, m[(3, 3)]);

    assert_eq!(1.0 * 2.0, m[(1, 0)]);
    assert_eq!(13.0 * 2.0, m[(1, 3)]);

    assert_eq!(7.0 * 2.0, m[(3, 1)]);
    assert_eq!(14.0 * 2.0, m[(2, 3)]);
}

#[test]
pub fn matrix_add_2() {
    let m1 = Mat4x4::new([
        0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 21.0,
    ]);
    let m2 = Mat4x4::new([
        0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 21.0,
    ]);

    {
        let col0 = m1.get_column(0);
        let col1 = m1.get_column(1);
        let col2 = m1.get_column(2);
        let col3 = m1.get_column(3);

        assert_eq!(&col0, &Vec4::from([0.0, 1.0, 2.0, 3.0]));
        assert_eq!(&col1, &Vec4::from([4.0, 5.0, 6.0, 7.0]));
        assert_eq!(&col2, &Vec4::from([8.0, 9.0, 10.0, 11.0]));
        assert_eq!(&col3, &Vec4::from([12.0, 13.0, 14.0, 21.0]));
    }

    {
        let col0 = m2.get_column(0);
        let col1 = m2.get_column(1);
        let col2 = m2.get_column(2);
        let col3 = m2.get_column(3);

        assert_eq!(&col0, &Vec4::from([0.0, 1.0, 2.0, 3.0]));
        assert_eq!(&col1, &Vec4::from([4.0, 5.0, 6.0, 7.0]));
        assert_eq!(&col2, &Vec4::from([8.0, 9.0, 10.0, 11.0]));
        assert_eq!(&col3, &Vec4::from([12.0, 13.0, 14.0, 21.0]));
    }

    assert_eq!(6.0, m1[(2, 1)]);
    assert_eq!(9.0, m1[(1, 2)]);

    assert_eq!(0.0, m1[(0, 0)]);
    assert_eq!(21.0, m1[(3, 3)]);

    assert_eq!(1.0, m1[(1, 0)]);
    assert_eq!(13.0, m1[(1, 3)]);

    assert_eq!(7.0, m1[(3, 1)]);
    assert_eq!(14.0, m1[(2, 3)]);

    assert_eq!(6.0, m2[(2, 1)]);
    assert_eq!(9.0, m2[(1, 2)]);

    assert_eq!(0.0, m2[(0, 0)]);
    assert_eq!(21.0, m2[(3, 3)]);

    assert_eq!(1.0, m2[(1, 0)]);
    assert_eq!(13.0, m2[(1, 3)]);

    assert_eq!(7.0, m2[(3, 1)]);
    assert_eq!(14.0, m2[(2, 3)]);

    let m = m1 + m2;

    assert_eq!(6.0 * 2.0, m[(2, 1)]);
    assert_eq!(9.0 * 2.0, m[(1, 2)]);

    assert_eq!(0.0 * 2.0, m[(0, 0)]);
    assert_eq!(21.0 * 2.0, m[(3, 3)]);

    assert_eq!(1.0 * 2.0, m[(1, 0)]);
    assert_eq!(13.0 * 2.0, m[(1, 3)]);

    assert_eq!(7.0 * 2.0, m[(3, 1)]);
    assert_eq!(14.0 * 2.0, m[(2, 3)]);
}

#[test]
pub fn matrix_mul_1() {
    let m1 = Mat4x4::translation([2f32, 0f32, 1f32].into());

    let vec = Vec4::new(2f32, 0f32, 1f32, 1f32);
    let vec = m1 * vec;

    assert_eq!(&Vec4::new(4f32, 0f32, 2f32, 1f32), &vec);
}

#[test]
pub fn matrix_mul_2() {
    let m1 = Mat4x4::translation([5f32, 0f32, 1f32].into());

    let vec = Vec4::new(2f32, 0f32, 1f32, 1f32);
    let vec = m1 * vec;

    assert_eq!(&Vec4::new(7f32, 0f32, 2f32, 1f32), &vec);
}

#[test]
pub fn matrix_inverse_same_1() {
    let m1 = Mat4x4::translation([5f32, 0f32, 1f32].into());
    let mut m2 = Mat4x4::translation([5f32, 0f32, 1f32].into());

    let m1 = m1.inverse();
    m2.inverse_assign();

    assert_eq!(&m1, &m2);
}

#[test]
pub fn matrix_inverse_same_2() {
    // INFO: Taking the inverse of a projection matrix is invalid, i'm just ensuring that the same
    //       underlying math is being done in inverse and inverse_assign with a reasonably complex
    //       matrix
    let m1 = Mat4x4::perspective(5f32, 90f32.to_radians(), 0.1f32, 100.0f32);
    let mut m2 = Mat4x4::perspective(5f32, 90f32.to_radians(), 0.1f32, 100.0f32);

    let m1 = m1.inverse();
    m2.inverse_assign();

    assert_eq!(&m1, &m2);
}
