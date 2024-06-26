//
//
// This file is a part of Aleph
//
// https://github.com/nathanvoglsam/aleph
//
// MIT License
//
// Copyright (c) 2020 Aleph Engine
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

use crate::render::{BufferHandle, MeshHandle, TextureHandle};
use std::marker::PhantomData;
use std::num::NonZeroUsize;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use thiserror::Error;

pub type TextureStreamingRequest = StreamingRequest<TextureHandle>;
pub type BufferStreamingRequest = StreamingRequest<BufferHandle>;
pub type MeshStreamingRequest = StreamingRequest<MeshHandle>;

/// Type that represents an opaque 'id' for a request.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[repr(transparent)]
pub struct RequestId(NonZeroUsize);

#[derive(Clone)]
#[repr(transparent)]
pub struct StreamingRequest<T> {
    data: Arc<RequestData>,
    _phantom: PhantomData<T>,
}

impl<T> StreamingRequest<T> {
    pub fn get_id(&self) -> RequestId {
        // Just use the address of the Arc, which is guaranteed to uniquely identify our request
        // while that request object is live.
        let id = self.data.as_ref() as *const RequestData as usize;

        // NOTE: do we need some kind of nonce or token in case we get the same address for a
        //       request multiple times? Ideally the system can just not care, but we can add an
        //       anti-collision token and put it in 'RequestData' if it avoids other problems.

        // Safety: We create the id from an address (which was a reference) which means it's UB if
        //         'id' = 0, so we can safely assume that it is in fact != 0.
        unsafe {
            RequestId(NonZeroUsize::new_unchecked(id))
        }
    }

    /// Try to take ownership of this request and move it into the 'waiting' state. This will only
    /// succeed a single time and failure should be taken as a signal that someone else is handling
    /// the request.
    pub(crate) fn try_take_ownership(&self) -> Option<()> {
        let result = self.data.state.compare_exchange(
            RequestState::Opened as _,
            RequestState::Waiting as _,
            Ordering::Relaxed,
            Ordering::Relaxed,
        );
        match result {
            Ok(_) => Some(()),
            Err(_) => None,
        }
    }

    /// Internal function used for transitioning to the 'failed' state.
    ///
    /// # Panics
    ///
    /// Will panic if the source state is not [RequestState::Waiting]
    pub(crate) fn mark_failed(&self) {
        self.transition_state(RequestState::Waiting, RequestState::Failed);
    }

    /// Internal function used for transitioning to the 'cancelled' state.
    ///
    /// # Panics
    ///
    /// Will panic if the source state is not [RequestState::Waiting]
    pub(crate) fn mark_cancelled(&self) {
        self.transition_state(RequestState::Waiting, RequestState::Cancelled);
    }

    pub(crate) fn transition_state(&self, expected: RequestState, dst: RequestState) {
        debug_assert!(expected.can_transition_to(dst));

        let result = self.data.state.compare_exchange(
            expected as _,
            dst as _,
            Ordering::Release,
            Ordering::Relaxed,
        );

        match result {
            Ok(_) => {}
            Err(v) => {
                // Safety: The state field will never contain a value that isn't a valid request
                //         state making this safe.
                let src = unsafe { RequestState::from_u64(v).unwrap_unchecked() };
                panic!(
                    "Tried to transition a request from '{:?}' to '{:?}'",
                    src, dst,
                );
            }
        }
    }
}

impl<T: IntoPayload> StreamingRequest<T> {
    /// Polls the request to see if it has completed, failed or been cancelled.
    ///
    /// Will yield the
    pub fn poll_complete(&self) -> Result<T, RequestError> {
        let state = self.data.get_state();
        if state == RequestState::Complete {
            let payload = self.data.get_payload();

            // Safety: It is unsafe for a caller to write anything other than a valid payload to the
            //         payload slot when in the Complete state. It is a bug if this _isn't_ safe.
            let payload = unsafe { T::from_payload(payload) };

            Ok(payload)
        } else {
            Err(RequestError::from_state(state))
        }
    }

    /// Internal function used for transitioning to the 'complete' state with the given payload.
    ///
    /// # Warning
    ///
    /// Two threads could race when calling this function between writing the payload and updating
    /// the state. It's theoretically possible for two threads to write payload A and B before
    /// either thread transitions the state. This would effectively destroy one of the payloads.
    /// However, we only allow [TextureHandle] objects to be stored into the payload slot so
    /// observers will only ever read a valid handle, just might not be the one they expected.
    ///
    /// It is expected that any calls to these functions are centrally managed and that any request
    /// manager takes ownership of the management of the request by only accepting requests in the
    /// 'open' state. This can be done atomically with a compare-exchange. It's not UB to call this
    /// function on multiple threads simultaneously, but it's probably going to give logic errors.
    pub(crate) fn mark_complete(&self, payload: T) {
        // Writes the payload and updates the state. The payload must be written first so that as
        // soon as an observer sees the state change the payload will be valid to read.
        let payload = payload.into_payload();
        self.data.payload.store(payload, Ordering::Relaxed);
        self.transition_state(RequestState::Waiting, RequestState::Complete);
    }
}

/// Trait that must be implemented on a type to allow it to be sent as a request payload.
///
/// The type must meet a few criteria:
///
/// - 8-bytes or less. The type must be able to fit inside a `u64` so it can be written atomically.
/// - Plain-old-data. The type will be mem-copied around and so will not uphold ownership semantics.
///
/// In simple terms, most small [Copy] types should be fine like [u64], but more complex types are
/// also supported as long as they are plain-old-data too.
pub unsafe trait IntoPayload {
    /// Rehydrates a `Self` from the given value, pulled from the request payload.
    ///
    /// # Safety
    ///
    /// The implementation is allowed to make some assumptions about the input, mainly:
    /// - Only `u64` values created from a call to [IntoPayload::into_payload] may be passed into
    ///   [IntoPayload::from_payload].
    ///
    /// It is the caller of [IntoPayload::from_payload] to ensure this requirement is upheld. What
    /// this means is that a payload should be assumed to be uninitialized memory until initialized
    /// with the output from a call to [IntoPayload::into_payload].
    ///
    /// Any number of `Self` instances can be created from that single [IntoPayload::into_payload]
    /// call output though, so callers of 'from_payload' should not expect single-ownership
    /// semantics to be upheld. `Self` should be plain-old-data.
    unsafe fn from_payload(v: u64) -> Self;

    /// Dehydrate `self` into a [u64] representation that can be passed through a request payload
    /// channel.
    ///
    /// A single [IntoPayload::into_payload] call can be 'rehydrated' into more than 1 `Self`
    /// instance, so do not expect ownership semantics to be upheld.
    fn into_payload(self) -> u64;
}

unsafe impl IntoPayload for u64 {
    unsafe fn from_payload(v: u64) -> Self {
        v
    }

    fn into_payload(self) -> u64 {
        self
    }
}

/// The data layout of a streaming request handle.
pub struct RequestData {
    /// The current state of the request.
    state: AtomicU64,

    /// A general 'payload' field that is used as a back-channel so the streaming system can give
    /// a result back to the request creator. This is only sane to read in a terminal state.
    payload: AtomicU64,
}

impl RequestData {
    pub(crate) fn get_state(&self) -> RequestState {
        let state = self.state.load(Ordering::Relaxed);
        // Safety: It is illegal to write a non-valid enum variant into 'state'
        unsafe { RequestState::from_u64(state).unwrap_unchecked() }
    }

    pub(crate) fn get_payload(&self) -> u64 {
        self.payload.load(Ordering::Relaxed)
    }
}

/// Enumeration of all supported values of the [RequestData::state] field.
///
/// # Info
///
/// This could be smaller than a `u64` but because our payload is `u64` sized the [RequestData]
/// struct will get padded out to 16-bytes anyway. May as well fill them so we don't get undefined
/// padding bytes.
#[repr(u64)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum RequestState {
    /// The initial state of a request, before it has been submitted to the queue for processing.
    Opened = 0,

    /// The state the request will transition to when submitted to the queue for processing.
    Waiting = 1,

    /// The state the request will transition to when completed. Only once the request is in this
    /// state is it correct to read the payload results. This is a terminal state.
    Complete = 2,

    /// The state the request will transition to when the request fails. The failure message will
    /// be written to the payload slot. This is a terminal state.
    Failed = 3,

    /// The state the request will transition to when the request is cancelled. This is logically
    /// similar to a failure. The data was not successfully loaded, but the cause was the request
    /// being cancelled by the streaming subsystem. This is a terminal state.
    Cancelled = 4,
}

impl RequestState {
    pub const fn from_u64(v: u64) -> Option<Self> {
        match v {
            0 => Some(Self::Opened),
            1 => Some(Self::Waiting),
            2 => Some(Self::Complete),
            3 => Some(Self::Failed),
            4 => Some(Self::Cancelled),
            _ => None,
        }
    }

    pub const fn into_u64(self) -> u64 {
        self as u64
    }

    /// Checks whether the target state is valid to transition to from the state `self` represents.
    pub const fn can_transition_to(self, target: Self) -> bool {
        match self {
            Self::Opened => matches!(target, Self::Waiting | Self::Failed | Self::Cancelled),
            Self::Waiting => matches!(target, Self::Failed | Self::Cancelled | Self::Complete),
            Self::Complete => false,
            Self::Failed => false,
            Self::Cancelled => false,
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Error)]
pub enum RequestError {
    #[error("The request is not complete")]
    Waiting,

    #[error("The request failed for an unknown reason")]
    Failed,

    #[error("The request was cancelled")]
    Cancelled,
}

impl RequestError {
    pub const fn from_state(v: RequestState) -> Self {
        match v {
            RequestState::Opened => Self::Waiting,
            RequestState::Waiting => Self::Waiting,
            RequestState::Complete => Self::Waiting, // Nonsense but means the function won't fail
            RequestState::Failed => Self::Failed,
            RequestState::Cancelled => Self::Cancelled,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::render::streaming_request::RequestError;
    use crate::render::{RequestData, StreamingRequest};
    use std::sync::Arc;

    fn make_request() -> StreamingRequest<u64> {
        StreamingRequest {
            data: Arc::new(RequestData {
                state: Default::default(),
                payload: Default::default(),
            }),
            _phantom: Default::default(),
        }
    }

    #[test]
    pub fn request_to_completion() {
        let request = make_request();

        assert_eq!(request.poll_complete().unwrap_err(), RequestError::Waiting);

        // First move the request into the waiting state, assert we succeed
        request.try_take_ownership().unwrap();

        assert_eq!(request.poll_complete().unwrap_err(), RequestError::Waiting);

        // Complete the request, will panic if we fail
        request.mark_complete(56);

        // Unwrap the request with the payload
        assert_eq!(request.poll_complete().unwrap(), 56);
    }

    #[test]
    pub fn request_to_failure() {
        let request = make_request();

        // First move the request into the waiting state, assert we succeed
        request.try_take_ownership().unwrap();

        // Fail the request, will panic if we fail this call
        request.mark_failed();

        // Unwrap the request
        assert_eq!(request.poll_complete().unwrap_err(), RequestError::Failed);
    }

    #[test]
    pub fn request_to_cancelled() {
        let request = make_request();

        // First move the request into the waiting state, assert we succeed
        request.try_take_ownership().unwrap();

        // Cancel the request, will panic if we fail
        request.mark_cancelled();

        // Unwrap the request
        assert_eq!(
            request.poll_complete().unwrap_err(),
            RequestError::Cancelled
        );
    }

    #[test]
    pub fn request_double_waiting() {
        let request = make_request();

        // Move the request to the 'waiting' state
        request.try_take_ownership().unwrap();

        // Try again, should always fail afterwards
        assert!(request.try_take_ownership().is_none());
        assert!(request.try_take_ownership().is_none());
    }

    #[test]
    #[should_panic]
    pub fn request_bad_transition_success() {
        let request = make_request();

        // Move the request to the 'waiting' state
        request.try_take_ownership().unwrap();

        // Complete the request
        request.mark_complete(56);

        // Complete it again, should fail as the state machine won't allow the transition
        request.mark_complete(21);
    }

    #[test]
    #[should_panic]
    pub fn request_bad_transition_cancel_complete() {
        let request = make_request();

        // Move the request to the 'waiting' state
        request.try_take_ownership().unwrap();

        // Cancel the request
        request.mark_cancelled();

        // Try to complete it after cancelling, should fail as the state machine won't allow the
        // transition
        request.mark_complete(21);
    }
}
