use crate::rt::io::{AsyncRead, AsyncWrite};

pub mod layer;
pub mod service;

pub trait Stream: AsyncRead + AsyncWrite {}

impl<T> Stream for T where T: AsyncRead + AsyncWrite {}
