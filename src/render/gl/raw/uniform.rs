use crate::render::gl::raw::bool_to_enum;

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `v0` - For the scalar commands, specifies the new values to be used for the specified uniform
/// variable.
#[inline]
pub fn uniform_1f(location: i32, v0: f32) {
    unsafe {
        gl::Uniform1f(location, v0);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `v0, v1` - For the scalar commands, specifies the new values to be used for the specified
/// uniform variable.
#[inline]
pub fn uniform_2f(location: i32, v0: f32, v1: f32) {
    unsafe {
        gl::Uniform2f(location, v0, v1);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `v0, v1, v2` - For the scalar commands, specifies the new values to be used for the specified
/// uniform variable.
#[inline]
pub fn uniform_3f(location: i32, v0: f32, v1: f32, v2: f32) {
    unsafe {
        gl::Uniform3f(location, v0, v1, v2);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `v0, v1, v2, v3` - For the scalar commands, specifies the new values to be used for the
/// specified uniform variable.
#[inline]
pub fn uniform_4f(location: i32, v0: f32, v1: f32, v2: f32, v3: f32) {
    unsafe {
        gl::Uniform4f(location, v0, v1, v2, v3);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `v0` - For the scalar commands, specifies the new values to be used for the specified uniform
/// variable.
#[inline]
pub fn uniform_1i(location: i32, v0: i32) {
    unsafe {
        gl::Uniform1i(location, v0);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `v0, v1` - For the scalar commands, specifies the new values to be used for the specified
/// uniform variable.
#[inline]
pub fn uniform_2i(location: i32, v0: i32, v1: i32) {
    unsafe {
        gl::Uniform2i(location, v0, v1);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `v0, v1, v2` - For the scalar commands, specifies the new values to be used for the specified
/// uniform variable.
#[inline]
pub fn uniform_3i(location: i32, v0: i32, v1: i32, v2: i32) {
    unsafe {
        gl::Uniform3i(location, v0, v1, v2);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `v0, v1, v2, v3` - For the scalar commands, specifies the new values to be used for the
/// specified uniform variable.
#[inline]
pub fn uniform_4i(location: i32, v0: i32, v1: i32, v2: i32, v3: i32) {
    unsafe {
        gl::Uniform4i(location, v0, v1, v2, v3);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `v0` - For the scalar commands, specifies the new values to be used for the specified uniform
/// variable.
#[inline]
pub fn uniform_1ui(location: i32, v0: u32) {
    unsafe {
        gl::Uniform1ui(location, v0);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `v0, v1` - For the scalar commands, specifies the new values to be used for the specified
/// uniform variable.
#[inline]
pub fn uniform_2ui(location: i32, v0: u32, v1: u32) {
    unsafe {
        gl::Uniform2ui(location, v0, v1);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `v0, v1, v2` - For the scalar commands, specifies the new values to be used for the specified
/// uniform variable.
#[inline]
pub fn uniform_3ui(location: i32, v0: u32, v1: u32, v2: u32) {
    unsafe {
        gl::Uniform3ui(location, v0, v1, v2);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `v0, v1, v2, v3` - For the scalar commands, specifies the new values to be used for the
/// specified uniform variable.
#[inline]
pub fn uniform_4ui(location: i32, v0: u32, v1: u32, v2: u32, v3: u32) {
    unsafe {
        gl::Uniform4ui(location, v0, v1, v2, v3);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `count` - For the vector (glUniform*v) commands, specifies the number of elements that are to be
/// modified. This should be 1 if the targeted uniform variable is not an array, and 1 or more if it
/// is an array. For the matrix (glUniformMatrix*) commands, specifies the number of matrices that
/// are to be modified. This should be 1 if the targeted uniform variable is not an array of
/// matrices, and 1 or more if it is an array of matrices. `value` - For the vector and matrix
/// commands, specifies a pointer to an array of count values that will be used to update the
/// specified uniform variable.
#[inline]
pub fn uniform_1fv(location: i32, count: i32, value: *const f32) {
    unsafe {
        gl::Uniform1fv(location, count, value);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `count` - For the vector (glUniform*v) commands, specifies the number of elements that are to be
/// modified. This should be 1 if the targeted uniform variable is not an array, and 1 or more if it
/// is an array. For the matrix (glUniformMatrix*) commands, specifies the number of matrices that
/// are to be modified. This should be 1 if the targeted uniform variable is not an array of
/// matrices, and 1 or more if it is an array of matrices. `value` - For the vector and matrix
/// commands, specifies a pointer to an array of count values that will be used to update the
/// specified uniform variable.
#[inline]
pub fn uniform_2fv(location: i32, count: i32, value: *const f32) {
    unsafe {
        gl::Uniform2fv(location, count, value);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `count` - For the vector (glUniform*v) commands, specifies the number of elements that are to be
/// modified. This should be 1 if the targeted uniform variable is not an array, and 1 or more if it
/// is an array. For the matrix (glUniformMatrix*) commands, specifies the number of matrices that
/// are to be modified. This should be 1 if the targeted uniform variable is not an array of
/// matrices, and 1 or more if it is an array of matrices. `value` - For the vector and matrix
/// commands, specifies a pointer to an array of count values that will be used to update the
/// specified uniform variable.
#[inline]
pub fn uniform_3fv(location: i32, count: i32, value: *const f32) {
    unsafe {
        gl::Uniform3fv(location, count, value);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `count` - For the vector (glUniform*v) commands, specifies the number of elements that are to be
/// modified. This should be 1 if the targeted uniform variable is not an array, and 1 or more if it
/// is an array. For the matrix (glUniformMatrix*) commands, specifies the number of matrices that
/// are to be modified. This should be 1 if the targeted uniform variable is not an array of
/// matrices, and 1 or more if it is an array of matrices. `value` - For the vector and matrix
/// commands, specifies a pointer to an array of count values that will be used to update the
/// specified uniform variable.
#[inline]
pub fn uniform_4fv(location: i32, count: i32, value: *const f32) {
    unsafe {
        gl::Uniform4fv(location, count, value);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `count` - For the vector (glUniform*v) commands, specifies the number of elements that are to be
/// modified. This should be 1 if the targeted uniform variable is not an array, and 1 or more if it
/// is an array. For the matrix (glUniformMatrix*) commands, specifies the number of matrices that
/// are to be modified. This should be 1 if the targeted uniform variable is not an array of
/// matrices, and 1 or more if it is an array of matrices. `value` - For the vector and matrix
/// commands, specifies a pointer to an array of count values that will be used to update the
/// specified uniform variable.
#[inline]
pub fn uniform_1iv(location: i32, count: i32, value: *const i32) {
    unsafe {
        gl::Uniform1iv(location, count, value);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `count` - For the vector (glUniform*v) commands, specifies the number of elements that are to be
/// modified. This should be 1 if the targeted uniform variable is not an array, and 1 or more if it
/// is an array. For the matrix (glUniformMatrix*) commands, specifies the number of matrices that
/// are to be modified. This should be 1 if the targeted uniform variable is not an array of
/// matrices, and 1 or more if it is an array of matrices. `value` - For the vector and matrix
/// commands, specifies a pointer to an array of count values that will be used to update the
/// specified uniform variable.
#[inline]
pub fn uniform_2iv(location: i32, count: i32, value: *const i32) {
    unsafe {
        gl::Uniform2iv(location, count, value);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `count` - For the vector (glUniform*v) commands, specifies the number of elements that are to be
/// modified. This should be 1 if the targeted uniform variable is not an array, and 1 or more if it
/// is an array. For the matrix (glUniformMatrix*) commands, specifies the number of matrices that
/// are to be modified. This should be 1 if the targeted uniform variable is not an array of
/// matrices, and 1 or more if it is an array of matrices. `value` - For the vector and matrix
/// commands, specifies a pointer to an array of count values that will be used to update the
/// specified uniform variable.
#[inline]
pub fn uniform_3iv(location: i32, count: i32, value: *const i32) {
    unsafe {
        gl::Uniform3iv(location, count, value);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `count` - For the vector (glUniform*v) commands, specifies the number of elements that are to be
/// modified. This should be 1 if the targeted uniform variable is not an array, and 1 or more if it
/// is an array. For the matrix (glUniformMatrix*) commands, specifies the number of matrices that
/// are to be modified. This should be 1 if the targeted uniform variable is not an array of
/// matrices, and 1 or more if it is an array of matrices. `value` - For the vector and matrix
/// commands, specifies a pointer to an array of count values that will be used to update the
/// specified uniform variable.
#[inline]
pub fn uniform_4iv(location: i32, count: i32, value: *const i32) {
    unsafe {
        gl::Uniform4iv(location, count, value);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `count` - For the vector (glUniform*v) commands, specifies the number of elements that are to be
/// modified. This should be 1 if the targeted uniform variable is not an array, and 1 or more if it
/// is an array. For the matrix (glUniformMatrix*) commands, specifies the number of matrices that
/// are to be modified. This should be 1 if the targeted uniform variable is not an array of
/// matrices, and 1 or more if it is an array of matrices. `value` - For the vector and matrix
/// commands, specifies a pointer to an array of count values that will be used to update the
/// specified uniform variable.
#[inline]
pub fn uniform_1uiv(location: i32, count: i32, value: *const u32) {
    unsafe {
        gl::Uniform1uiv(location, count, value);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `count` - For the vector (glUniform*v) commands, specifies the number of elements that are to be
/// modified. This should be 1 if the targeted uniform variable is not an array, and 1 or more if it
/// is an array. For the matrix (glUniformMatrix*) commands, specifies the number of matrices that
/// are to be modified. This should be 1 if the targeted uniform variable is not an array of
/// matrices, and 1 or more if it is an array of matrices. `value` - For the vector and matrix
/// commands, specifies a pointer to an array of count values that will be used to update the
/// specified uniform variable.
#[inline]
pub fn uniform_2uiv(location: i32, count: i32, value: *const u32) {
    unsafe {
        gl::Uniform2uiv(location, count, value);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `count` - For the vector (glUniform*v) commands, specifies the number of elements that are to be
/// modified. This should be 1 if the targeted uniform variable is not an array, and 1 or more if it
/// is an array. For the matrix (glUniformMatrix*) commands, specifies the number of matrices that
/// are to be modified. This should be 1 if the targeted uniform variable is not an array of
/// matrices, and 1 or more if it is an array of matrices. `value` - For the vector and matrix
/// commands, specifies a pointer to an array of count values that will be used to update the
/// specified uniform variable.
#[inline]
pub fn uniform_3uiv(location: i32, count: i32, value: *const u32) {
    unsafe {
        gl::Uniform3uiv(location, count, value);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `count` - For the vector (glUniform*v) commands, specifies the number of elements that are to be
/// modified. This should be 1 if the targeted uniform variable is not an array, and 1 or more if it
/// is an array. For the matrix (glUniformMatrix*) commands, specifies the number of matrices that
/// are to be modified. This should be 1 if the targeted uniform variable is not an array of
/// matrices, and 1 or more if it is an array of matrices. `value` - For the vector and matrix
/// commands, specifies a pointer to an array of count values that will be used to update the
/// specified uniform variable.
#[inline]
pub fn uniform_4uiv(location: i32, count: i32, value: *const u32) {
    unsafe {
        gl::Uniform4uiv(location, count, value);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `count` - For the vector (glUniform*v) commands, specifies the number of elements that are to be
/// modified. This should be 1 if the targeted uniform variable is not an array, and 1 or more if it
/// is an array. For the matrix (glUniformMatrix*) commands, specifies the number of matrices that
/// are to be modified. This should be 1 if the targeted uniform variable is not an array of
/// matrices, and 1 or more if it is an array of matrices. `transpose` - For the matrix commands,
/// specifies whether to transpose the matrix as the values are loaded into the uniform variable.
/// `value` - For the vector and matrix commands, specifies a pointer to an array of count values
/// that will be used to update the specified uniform variable.
#[inline]
pub fn uniform_matrix_2fv(location: i32, count: i32, transpose: bool, value: *const f32) {
    unsafe {
        gl::UniformMatrix2fv(location, count, bool_to_enum(transpose), value);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `count` - For the vector (glUniform*v) commands, specifies the number of elements that are to be
/// modified. This should be 1 if the targeted uniform variable is not an array, and 1 or more if it
/// is an array. For the matrix (glUniformMatrix*) commands, specifies the number of matrices that
/// are to be modified. This should be 1 if the targeted uniform variable is not an array of
/// matrices, and 1 or more if it is an array of matrices. `transpose` - For the matrix commands,
/// specifies whether to transpose the matrix as the values are loaded into the uniform variable.
/// `value` - For the vector and matrix commands, specifies a pointer to an array of count values
/// that will be used to update the specified uniform variable.
#[inline]
pub fn uniform_matrix_3fv(location: i32, count: i32, transpose: bool, value: *const f32) {
    unsafe {
        gl::UniformMatrix3fv(location, count, bool_to_enum(transpose), value);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `count` - For the vector (glUniform*v) commands, specifies the number of elements that are to be
/// modified. This should be 1 if the targeted uniform variable is not an array, and 1 or more if it
/// is an array. For the matrix (glUniformMatrix*) commands, specifies the number of matrices that
/// are to be modified. This should be 1 if the targeted uniform variable is not an array of
/// matrices, and 1 or more if it is an array of matrices. `transpose` - For the matrix commands,
/// specifies whether to transpose the matrix as the values are loaded into the uniform variable.
/// `value` - For the vector and matrix commands, specifies a pointer to an array of count values
/// that will be used to update the specified uniform variable.
#[inline]
pub fn uniform_matrix_4fv(location: i32, count: i32, transpose: bool, value: *const f32) {
    unsafe {
        gl::UniformMatrix4fv(location, count, bool_to_enum(transpose), value);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `count` - For the vector (glUniform*v) commands, specifies the number of elements that are to be
/// modified. This should be 1 if the targeted uniform variable is not an array, and 1 or more if it
/// is an array. For the matrix (glUniformMatrix*) commands, specifies the number of matrices that
/// are to be modified. This should be 1 if the targeted uniform variable is not an array of
/// matrices, and 1 or more if it is an array of matrices. `transpose` - For the matrix commands,
/// specifies whether to transpose the matrix as the values are loaded into the uniform variable.
/// `value` - For the vector and matrix commands, specifies a pointer to an array of count values
/// that will be used to update the specified uniform variable.
#[inline]
pub fn uniform_matrix_2x3fv(location: i32, count: i32, transpose: bool, value: *const f32) {
    unsafe {
        gl::UniformMatrix2x3fv(location, count, bool_to_enum(transpose), value);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `count` - For the vector (glUniform*v) commands, specifies the number of elements that are to be
/// modified. This should be 1 if the targeted uniform variable is not an array, and 1 or more if it
/// is an array. For the matrix (glUniformMatrix*) commands, specifies the number of matrices that
/// are to be modified. This should be 1 if the targeted uniform variable is not an array of
/// matrices, and 1 or more if it is an array of matrices. `transpose` - For the matrix commands,
/// specifies whether to transpose the matrix as the values are loaded into the uniform variable.
/// `value` - For the vector and matrix commands, specifies a pointer to an array of count values
/// that will be used to update the specified uniform variable.
#[inline]
pub fn uniform_matrix_3x2fv(location: i32, count: i32, transpose: bool, value: *const f32) {
    unsafe {
        gl::UniformMatrix3x2fv(location, count, bool_to_enum(transpose), value);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `count` - For the vector (glUniform*v) commands, specifies the number of elements that are to be
/// modified. This should be 1 if the targeted uniform variable is not an array, and 1 or more if it
/// is an array. For the matrix (glUniformMatrix*) commands, specifies the number of matrices that
/// are to be modified. This should be 1 if the targeted uniform variable is not an array of
/// matrices, and 1 or more if it is an array of matrices. `transpose` - For the matrix commands,
/// specifies whether to transpose the matrix as the values are loaded into the uniform variable.
/// `value` - For the vector and matrix commands, specifies a pointer to an array of count values
/// that will be used to update the specified uniform variable.
#[inline]
pub fn uniform_matrix_2x4fv(location: i32, count: i32, transpose: bool, value: *const f32) {
    unsafe {
        gl::UniformMatrix2x4fv(location, count, bool_to_enum(transpose), value);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `count` - For the vector (glUniform*v) commands, specifies the number of elements that are to be
/// modified. This should be 1 if the targeted uniform variable is not an array, and 1 or more if it
/// is an array. For the matrix (glUniformMatrix*) commands, specifies the number of matrices that
/// are to be modified. This should be 1 if the targeted uniform variable is not an array of
/// matrices, and 1 or more if it is an array of matrices. `transpose` - For the matrix commands,
/// specifies whether to transpose the matrix as the values are loaded into the uniform variable.
/// `value` - For the vector and matrix commands, specifies a pointer to an array of count values
/// that will be used to update the specified uniform variable.
#[inline]
pub fn uniform_matrix_4x2fv(location: i32, count: i32, transpose: bool, value: *const f32) {
    unsafe {
        gl::UniformMatrix4x2fv(location, count, bool_to_enum(transpose), value);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `count` - For the vector (glUniform*v) commands, specifies the number of elements that are to be
/// modified. This should be 1 if the targeted uniform variable is not an array, and 1 or more if it
/// is an array. For the matrix (glUniformMatrix*) commands, specifies the number of matrices that
/// are to be modified. This should be 1 if the targeted uniform variable is not an array of
/// matrices, and 1 or more if it is an array of matrices. `transpose` - For the matrix commands,
/// specifies whether to transpose the matrix as the values are loaded into the uniform variable.
/// `value` - For the vector and matrix commands, specifies a pointer to an array of count values
/// that will be used to update the specified uniform variable.
#[inline]
pub fn uniform_matrix_3x4fv(location: i32, count: i32, transpose: bool, value: *const f32) {
    unsafe {
        gl::UniformMatrix3x4fv(location, count, bool_to_enum(transpose), value);
    }
}

/// [2.0] Specify the value of a uniform variable for the current program object.
///
/// # Arguments
///
/// `location` - Specifies the location of the uniform variable to be modified.
/// `count` - For the vector (glUniform*v) commands, specifies the number of elements that are to be
/// modified. This should be 1 if the targeted uniform variable is not an array, and 1 or more if it
/// is an array. For the matrix (glUniformMatrix*) commands, specifies the number of matrices that
/// are to be modified. This should be 1 if the targeted uniform variable is not an array of
/// matrices, and 1 or more if it is an array of matrices. `transpose` - For the matrix commands,
/// specifies whether to transpose the matrix as the values are loaded into the uniform variable.
/// `value` - For the vector and matrix commands, specifies a pointer to an array of count values
/// that will be used to update the specified uniform variable.
#[inline]
pub fn uniform_matrix_4x3fv(location: i32, count: i32, transpose: bool, value: *const f32) {
    unsafe {
        gl::UniformMatrix4x3fv(location, count, bool_to_enum(transpose), value);
    }
}
