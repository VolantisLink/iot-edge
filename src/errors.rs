use std::error::Error;
use std::convert::From;
use std::fmt;
use rumqttc::{Request, SendError};


#[derive(Debug)]
pub enum IotEdgeError {
    Generic(&'static str),
   
    MqttSendError,
    MqttPubAckError,
}

impl fmt::Display for IotEdgeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for IotEdgeError {}

impl <'a>From<&'static str> for IotEdgeError {
    fn from(s: &'static str) -> Self {
        IotEdgeError::Generic(s)
    }
}

impl From<SendError<Request>> for IotEdgeError {
    fn from(_: SendError<Request>) -> Self {
        IotEdgeError::MqttSendError
    }
}

