pub mod platform_h;
pub mod cl_gl_h;

pub use self::platform_h::{ ClGlUint, ClGlint, ClGlEnum };
pub use self::cl_gl_h::{ clCreateFromGLBuffer, clCreateFromGLRenderbuffer,
    clCreateFromGLTexture2D, clCreateFromGLTexture3D, clEnqueueAcquireGLObjects,
    clEnqueueReleaseGLObjects };