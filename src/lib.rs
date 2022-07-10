#[cfg(not(feature = "internal-bindgen-on-build"))]
#[allow(non_camel_case_types)]
pub mod sys {
    include!("stb_image.rs");
}

pub use bindings::*;

#[cfg(feature = "internal-bindgen-on-build")]
pub mod bindings {}

#[cfg(not(feature = "internal-bindgen-on-build"))]
pub mod bindings {
    use thiserror::Error;

    use super::sys;

    pub struct Image {
        ptr: core::ptr::NonNull<u8>,
        len: usize,
        width: usize,
        height: usize,
        channels: usize,
    }

    #[derive(Debug, Error)]
    pub enum ImgCreationError {
        #[error("image is invalid or corrupted")]
        InvalidOrCorrupted,
    }

    impl Image {
        pub const fn width(&self) -> usize {
            self.width
        }

        pub const fn height(&self) -> usize {
            self.height
        }

        pub const fn channels(&self) -> usize {
            self.channels
        }

        pub fn bytes(&self) -> &[u8] {
            unsafe { core::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
        }

        pub fn from_bytes(
            bytes: &[u8],
            desired_channels: usize,
        ) -> Result<Image, ImgCreationError> {
            let mut width: cty::c_int = 0;
            let mut height: cty::c_int = 0;
            let mut channels: cty::c_int = 0;

            if let Some(ptr) = core::ptr::NonNull::new(unsafe {
                sys::stbi_load_from_memory(
                    bytes.as_ptr(),
                    bytes.len() as _,
                    &mut width as *mut _,
                    &mut height as *mut _,
                    &mut channels as *mut _,
                    desired_channels as _,
                )
            }) {
                Ok(Image {
                    ptr,
                    len: width as usize * height as usize * desired_channels,
                    width: width as usize,
                    height: height as usize,
                    channels: desired_channels,
                })
            } else {
                Err(ImgCreationError::InvalidOrCorrupted)
            }
        }
    }

    impl Drop for Image {
        fn drop(&mut self) {
            unsafe {
                sys::stbi_image_free(self.ptr.as_ptr() as *mut _);
            }
        }
    }
}
