use crate::{
    error::{ArrayError, LibpdError, SizeError},
    C_STRING_FAILURE,
};

use std::ffi::CString;

/// Gets the size of an array by name in the pd patch which is running.
///
/// # Example
/// ```no_run
/// use libpd_rs::array::array_size;
///
/// let size = array_size("my_array").unwrap();
/// ```
pub fn array_size<T: AsRef<str>>(name: T) -> Result<i32, LibpdError> {
    unsafe {
        let name = CString::new(name.as_ref()).expect(C_STRING_FAILURE);
        // Returns size or negative error code if non-existent
        let result = libpd_sys::libpd_arraysize(name.as_ptr());
        if result >= 0 {
            return Ok(result);
        }
        Err(LibpdError::SizeError(SizeError::CouldNotDetermine))
    }
}

/// Resizes an array found by name in the pd patch which is running.
///
/// Sizes <= 0 or > size limit are clipped to 1
///
/// # Example
/// ```no_run
/// use libpd_rs::array::{array_size, resize_array};
///
/// resize_array("my_array", 1024).unwrap();
/// let size = array_size("my_array").unwrap();
/// assert_eq!(size, 1024);
///
/// resize_array("my_array", 0).unwrap();
/// let size = array_size("my_array").unwrap();
/// assert_eq!(size, 1);
/// ```
pub fn resize_array<T: AsRef<str>>(name: T, size: i64) -> Result<(), LibpdError> {
    unsafe {
        let name = CString::new(name.as_ref()).expect(C_STRING_FAILURE);
        // returns 0 on success or negative error code if non-existent
        match libpd_sys::libpd_resize_array(name.as_ptr(), size) {
            0 => Ok(()),
            _ => Err(LibpdError::SizeError(SizeError::CouldNotDetermine)),
        }
    }
}

/// Reads a named array from pd to a mutable slice of `f32`.
///
/// Reads values as much as `read_amount` from the array which is specified with the `source_name` argument
/// and writes them to `destination` starting at `destination_offset`.
///
/// # Example
/// ```no_run
/// use libpd_rs::array::read_float_array_from;
///
/// let mut destination = [0.0_f32; 64];
/// read_float_array_from("my_array", 32, &mut destination, 32).unwrap();
/// ```
/// # Errors
/// This function performs no bounds checking on the destination.
///
/// If `destination_offset` + `read_amount` is greater than the size of the `destination` or
/// the array which we're trying to read from is not existent it will return an error.
pub fn read_float_array_from<T: AsRef<str>>(
    source_name: T,
    read_amount: i32,
    destination: &mut [f32],
    destination_offset: i32,
) -> Result<(), LibpdError> {
    unsafe {
        let name = CString::new(source_name.as_ref()).expect(C_STRING_FAILURE);
        // Returns 0 on success or a negative error code if the array is non-existent
        // or offset + n exceeds range of array
        match libpd_sys::libpd_read_array(
            destination.as_mut_ptr(),
            name.as_ptr(),
            destination_offset,
            read_amount,
        ) {
            0 => Ok(()),
            -2 => Err(LibpdError::ArrayError(ArrayError::OutOfBounds)),
            _ => Err(LibpdError::ArrayError(ArrayError::NonExistent)),
        }
    }
}

/// Writes a slice of `f32` to a pd named array.
///
/// Reads values as much as `read_amount` from the array which is given as the `source` argument
/// and writes them to a named array in pd which is specified with `destination_name` argument starting at `destination_offset`.
///
/// # Example
/// ```no_run
/// use libpd_rs::array::write_float_array_to;
///
/// let mut source = [1.0_f32; 64];
/// write_float_array_to("my_array", 32, &source, 32).unwrap();
/// ```
/// # Errors
/// This function performs no bounds checking on the destination.
///
/// If `destination_offset` + `read_amount` is greater than the size of the `destination` or
/// the array which we're trying to read from is not existent it will return an error.
pub fn write_float_array_to<T: AsRef<str>>(
    destination_name: T,
    destination_offset: i32,
    source: &[f32],
    read_amount: i32,
) -> Result<(), LibpdError> {
    unsafe {
        let name = CString::new(destination_name.as_ref()).expect(C_STRING_FAILURE);
        // Returns 0 on success or a negative error code if the array is non-existent
        // or offset + n exceeds range of array
        match libpd_sys::libpd_write_array(
            name.as_ptr(),
            destination_offset,
            source.as_ptr(),
            read_amount,
        ) {
            0 => Ok(()),
            -2 => Err(LibpdError::ArrayError(ArrayError::OutOfBounds)),
            _ => Err(LibpdError::ArrayError(ArrayError::NonExistent)),
        }
    }
}

/// Reads a named array from pd to a mutable slice of `f64`.
///
/// Reads values as much as `read_amount` from the array which is specified with the `source_name` argument
/// and writes them to `destination` starting at `destination_offset`.
///
/// # Example
/// ```no_run
/// use libpd_rs::array::read_double_array_from;
///
/// let mut destination = [0.0_f64; 64];
/// read_double_array_from("my_array", 32, &mut destination, 32).unwrap();
/// ```
/// # Errors
/// This function performs no bounds checking on the destination.
///
/// If `destination_offset` + `read_amount` is greater than the size of the `destination` or
/// the array which we're trying to read from is not existent it will return an error.
pub fn read_double_array_from<T: AsRef<str>>(
    source_name: T,
    read_amount: i32,
    destination: &mut [f64],
    destination_offset: i32,
) -> Result<(), LibpdError> {
    unsafe {
        let name = CString::new(source_name.as_ref()).expect(C_STRING_FAILURE);
        // Returns 0 on success or a negative error code if the array is non-existent
        // or offset + n exceeds range of array
        match libpd_sys::libpd_read_array_double(
            destination.as_mut_ptr(),
            name.as_ptr(),
            destination_offset,
            read_amount,
        ) {
            0 => Ok(()),
            -2 => Err(LibpdError::ArrayError(ArrayError::OutOfBounds)),
            _ => Err(LibpdError::ArrayError(ArrayError::NonExistent)),
        }
    }
}

/// Writes a slice of `f64` to a pd named array.
///
/// Reads values as much as `read_amount` from the array which is given as the `source` argument
/// and writes them to a named array in pd which is specified with `destination_name` argument starting at `destination_offset`.
///
/// # Example
/// ```no_run
/// use libpd_rs::array::write_double_array_to;
///
/// let source = [1.0_f64; 64];
/// write_double_array_to("my_array", 32, &source, 32).unwrap();
/// ```
/// # Errors
/// This function performs no bounds checking on the destination.
///
/// If `destination_offset` + `read_amount` is greater than the size of the `destination` or
/// the array which we're trying to read from is not existent it will return an error.
pub fn write_double_array_to<T: AsRef<str>>(
    destination_name: T,
    destination_offset: i32,
    source: &[f64],
    read_amount: i32,
) -> Result<(), LibpdError> {
    unsafe {
        let name = CString::new(destination_name.as_ref()).expect(C_STRING_FAILURE);
        // Returns 0 on success or a negative error code if the array is non-existent
        // or offset + n exceeds range of array
        match libpd_sys::libpd_write_array_double(
            name.as_ptr(),
            destination_offset,
            source.as_ptr(),
            read_amount,
        ) {
            0 => Ok(()),
            -2 => Err(LibpdError::ArrayError(ArrayError::OutOfBounds)),
            _ => Err(LibpdError::ArrayError(ArrayError::NonExistent)),
        }
    }
}
