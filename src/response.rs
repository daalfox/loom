use serde::Serialize;

use crate::request::{self, Init, Request};

#[derive(Debug, Serialize)]
pub struct Response<P> {
    pub src: String,
    pub dest: String,
    pub body: Body<P>,
}
impl<T, P: From<T>> From<Request<T>> for Response<P> {
    fn from(value: Request<T>) -> Self {
        Self {
            src: value.dest,
            dest: value.src,
            body: value.body.into(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Body<P> {
    pub in_reply_to: usize,
    #[serde(flatten)]
    pub payload: P,
}
impl<T, P: From<T>> From<request::Body<T>> for Body<P> {
    fn from(value: request::Body<T>) -> Self {
        Self {
            in_reply_to: value.id,
            payload: value.payload.into(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename = "init_ok")]
pub struct InitOk {}
impl From<Init> for InitOk {
    fn from(_value: Init) -> Self {
        Self {}
    }
}
