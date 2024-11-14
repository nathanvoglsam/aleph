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

use std::cell::UnsafeCell;
use std::mem::{needs_drop, ManuallyDrop, MaybeUninit};
use std::num::NonZeroUsize;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use thiserror::Error;

use crate::{BufferHandle, MeshHandle, TextureHandle};

#[derive(Clone)]
#[repr(transparent)]
pub struct StreamingRequest<T: Send, E: Send = ()> {
    data: Arc<RequestData<T, E>>,
}

impl<T: Send, E: Send> StreamingRequest<T, E> {
    pub fn get_id(&self) -> RequestId {
        // Just use the address of the Arc, which is guaranteed to uniquely identify our request
        // while that request object is live.
        let id = self.data.as_ref() as *const RequestData<T, E> as usize;

        // NOTE: do we need some kind of nonce or token in case we get the same address for a
        //       request multiple times? Ideally the system can just not care, but we can add an
        //       anti-collision token and put it in 'RequestData' if it avoids other problems.

        // Safety: We create the id from an address (which was a reference) which means it's UB if
        //         'id' = 0, so we can safely assume that it is in fact != 0.
        unsafe { RequestId(NonZeroUsize::new_unchecked(id)) }
    }

    pub fn new() -> Self {
        Self {
            data: Arc::new(RequestData {
                state: Default::default(),
                payload: UnsafeCell::new(MaybeUninit::uninit()),
            }),
        }
    }

    /// Transition to the 'complete' state with the given payload.
    pub fn mark_complete(&self, payload: T) -> Result<(), TransitionResult> {
        let current = RequestState::Opened as u64;
        let new = RequestState::Finalizing as u64;
        let result =
            self.data
                .state
                .compare_exchange(current, new, Ordering::Acquire, Ordering::Relaxed);

        // Check to enter the critical section. Only a single thread can pass this check so once
        // we've passed this goalpost we have exclusive access to the payload slot.
        match result {
            Ok(_) => {}
            Err(_) => return Err(TransitionResult::InvalidSourceState),
        }

        // Safety: We've entered an atomically guarded critical section. It's impossible for more
        //         than a single thread to enter this code for any one request object. So we can
        //         safely write our request object
        unsafe {
            let slot = self.data.payload.get().as_mut().unwrap_unchecked();
            let data = Payload {
                success: ManuallyDrop::new(payload),
            };
            slot.write(data);
        }

        // Finally, we can mark the request as complete after we've written the payload.
        self.data
            .state
            .store(RequestState::Complete as u64, Ordering::Relaxed);

        Ok(())
    }

    /// Function used for transitioning to the 'failed' state. This will fail if the request is in
    /// a state that it is not valid to transition to 'failed' from.
    pub fn mark_failed(&self, reason: E) -> Result<(), TransitionResult> {
        let current = RequestState::Opened as u64;
        let new = RequestState::FinalizingFail as u64;
        let result =
            self.data
                .state
                .compare_exchange(current, new, Ordering::Acquire, Ordering::Relaxed);

        // Check to enter the critical section. Only a single thread can pass this check so once
        // we've passed this goalpost we have exclusive access to the payload slot.
        match result {
            Ok(_) => {}
            Err(_) => return Err(TransitionResult::InvalidSourceState),
        }

        // Safety: We've entered an atomically guarded critical section. It's impossible for more
        //         than a single thread to enter this code for any one request object. So we can
        //         safely write our request object
        unsafe {
            let slot = self.data.payload.get().as_mut().unwrap_unchecked();
            let data = Payload {
                fail: ManuallyDrop::new(reason),
            };
            slot.write(data);
        }

        // Finally, we can mark the request as complete after we've written the payload.
        self.data
            .state
            .store(RequestState::Failed as u64, Ordering::Relaxed);

        Ok(())
    }

    /// Function used for transitioning to the 'cancelled' state. This will fail if the request is
    /// in a state that it is not valid to transition to 'cancelled' from.
    pub fn mark_cancelled(&self) -> Result<(), TransitionResult> {
        let current = RequestState::Opened as u64;
        let new = RequestState::Cancelled as u64;
        let result =
            self.data
                .state
                .compare_exchange(current, new, Ordering::Relaxed, Ordering::Relaxed);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(TransitionResult::InvalidSourceState),
        }
    }

    /// Polls the request to see if it has completed, failed or been cancelled.
    pub fn poll_state(&self) -> RequestState {
        self.data.get_state()
    }

    /// Polls the request to see if it has completed, failed or been cancelled.
    ///
    /// This will return [`Ok`] exactly once, taking the `T` from the payload slot and leaving the
    /// request in the terminal 'Consumed' state.
    pub fn poll_complete(&self) -> Result<T, PollCompleteError> {
        let current = RequestState::Complete as u64;
        let new = RequestState::Consumed as u64;
        let result =
            self.data
                .state
                .compare_exchange(current, new, Ordering::Release, Ordering::Relaxed);

        match result {
            Ok(_) => {
                // Safety: It is a bug for the request to be in the complete state and for payload
                //         to not contain a valid T. We have entered a critical section by winning
                //         the race to transition from Complete to Consumed so we have ownership
                //         of the T in payload.
                unsafe {
                    let slot = self.data.payload.get().read();
                    let slot = slot.assume_init();
                    let payload = ManuallyDrop::into_inner(slot.success);
                    Ok(payload)
                }
            }
            Err(state) => {
                // Safety: It's a bug for state to not be a valid RequestState.
                let state = unsafe { RequestState::from_u64(state).unwrap_unchecked() };
                Err(PollCompleteError::from_state(state))
            }
        }
    }

    /// Polls the request to see if it has completed, failed or been cancelled.
    ///
    /// This will return [`Ok`] exactly once, taking the `T` from the payload slot and leaving the
    /// request in the terminal 'Consumed' state.
    pub fn poll_fail(&self) -> Result<E, PollFailError> {
        let current = RequestState::Failed as u64;
        let new = RequestState::ConsumedFail as u64;
        let result =
            self.data
                .state
                .compare_exchange(current, new, Ordering::Release, Ordering::Relaxed);

        match result {
            Ok(_) => {
                // Safety: It is a bug for the request to be in the complete state and for payload
                //         to not contain a valid T. We have entered a critical section by winning
                //         the race to transition from Complete to Consumed so we have ownership
                //         of the T in payload.
                unsafe {
                    let slot = self.data.payload.get().read();
                    let slot = slot.assume_init();
                    let payload = ManuallyDrop::into_inner(slot.fail);
                    Ok(payload)
                }
            }
            Err(state) => {
                // Safety: It's a bug for state to not be a valid RequestState.
                let state = unsafe { RequestState::from_u64(state).unwrap_unchecked() };
                Err(PollFailError::from_state(state))
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Error)]
pub enum TransitionResult {
    #[error("The request was in a state where it is invalid to transition to new state")]
    InvalidSourceState,
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

    /// The state the request will transition to when it has been completed, but the payload has
    /// not yet been written. This state is needed so that only a single writer can take ownership
    /// of the request for the purposes of writing the payload.
    Finalizing = 1,

    /// The state the request will transition to when completed. Only once the request is in this
    /// state is it correct to read the payload results.
    Complete = 2,

    /// The state the request will transition to after a reader has consumed the payload from the
    /// request. This is terminal state.
    Consumed = 3,

    /// Like [`RequestState::Finalizing`], but for the failure path.
    FinalizingFail = 4,

    /// The state the request will transition to when the request fails. The failure message will
    /// be written to the payload slot. This is a terminal state.
    Failed = 5,

    /// Like [`RequestState::Consumed`], but for the failure path.
    ConsumedFail = 6,

    /// The state the request will transition to when the request is cancelled. This is logically
    /// similar to a failure. The data was not successfully loaded, but the cause was the request
    /// being cancelled by the streaming subsystem. This is a terminal state.
    Cancelled = 7,
}

impl RequestState {
    /// Shorthand for checking if `self` is [`RequestState::Opened`]
    pub const fn is_open(&self) -> bool {
        matches!(*self, RequestState::Opened)
    }

    /// Shorthand for checking if `self` is [`RequestState::Finalizing`]
    pub const fn is_finalizing(&self) -> bool {
        matches!(*self, RequestState::Finalizing)
    }

    /// Shorthand for checking if `self` is [`RequestState::Complete`]
    pub const fn is_complete(&self) -> bool {
        matches!(*self, RequestState::Complete)
    }

    /// Shorthand for checking if `self` is [`RequestState::Consumed`]
    pub const fn is_consumed(&self) -> bool {
        matches!(*self, RequestState::Consumed)
    }

    /// Shorthand for checking if `self` is [`RequestState::FinalizingFail`]
    pub const fn is_finalizing_fail(&self) -> bool {
        matches!(*self, RequestState::FinalizingFail)
    }

    /// Shorthand for checking if `self` is [`RequestState::Failed`]
    pub const fn is_failed(&self) -> bool {
        matches!(*self, RequestState::Failed)
    }

    /// Shorthand for checking if `self` is [`RequestState::ConsumedFail`]
    pub const fn is_consumed_fail(&self) -> bool {
        matches!(*self, RequestState::ConsumedFail)
    }

    /// Shorthand for checking if `self` is [`RequestState::Cancelled`]
    pub const fn is_cancelled(&self) -> bool {
        matches!(*self, RequestState::Cancelled)
    }

    pub const fn from_u64(v: u64) -> Option<Self> {
        match v {
            0 => Some(Self::Opened),
            1 => Some(Self::Finalizing),
            2 => Some(Self::Complete),
            3 => Some(Self::Consumed),
            4 => Some(Self::FinalizingFail),
            5 => Some(Self::Failed),
            6 => Some(Self::ConsumedFail),
            7 => Some(Self::Cancelled),
            _ => None,
        }
    }

    pub const fn into_u64(self) -> u64 {
        self as u64
    }

    /// Checks whether the target state is valid to transition to from the state `self` represents.
    pub const fn can_transition_to(self, target: Self) -> bool {
        match self {
            Self::Opened => matches!(
                target,
                Self::Finalizing | Self::FinalizingFail | Self::Cancelled
            ),
            Self::Finalizing => matches!(target, Self::Complete),
            Self::Complete => matches!(target, Self::Consumed),
            Self::Consumed => false,
            Self::FinalizingFail => matches!(target, Self::Failed),
            Self::Failed => matches!(target, Self::ConsumedFail),
            Self::ConsumedFail => false,
            Self::Cancelled => false,
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Error)]
pub enum PollCompleteError {
    #[error("The request is not complete")]
    Waiting,

    #[error("The request failed. Poll the request again for the error object!")]
    Failed,

    #[error("The request was cancelled")]
    Cancelled,
}

impl PollCompleteError {
    pub const fn from_state(v: RequestState) -> Self {
        match v {
            RequestState::Opened => Self::Waiting,
            RequestState::Finalizing => Self::Waiting,
            RequestState::Complete => Self::Waiting, // Nonsense but means the function won't fail
            RequestState::Consumed => Self::Waiting, // Nonsense but means the function won't fail
            RequestState::FinalizingFail => Self::Waiting,
            RequestState::Failed => Self::Failed,
            RequestState::ConsumedFail => Self::Failed,
            RequestState::Cancelled => Self::Cancelled,
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Error)]
pub enum PollFailError {
    #[error("The request is not complete")]
    Waiting,

    #[error("The request hasn't failed. Poll for success instead!")]
    Complete,

    #[error("The request was cancelled")]
    Cancelled,
}

impl PollFailError {
    pub const fn from_state(v: RequestState) -> Self {
        match v {
            RequestState::Opened => Self::Waiting,
            RequestState::Finalizing => Self::Waiting,
            RequestState::Complete => Self::Complete,
            RequestState::Consumed => Self::Complete,
            RequestState::FinalizingFail => Self::Waiting,
            RequestState::Failed => Self::Waiting, // Junk so function won't fail
            RequestState::ConsumedFail => Self::Waiting, // Junk so function won't fail
            RequestState::Cancelled => Self::Cancelled,
        }
    }
}

/// Type that represents an opaque 'id' for a request.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[repr(transparent)]
pub struct RequestId(NonZeroUsize);

pub type TextureStreamingRequest = StreamingRequest<TextureHandle>;
pub type BufferStreamingRequest = StreamingRequest<BufferHandle>;
pub type MeshStreamingRequest = StreamingRequest<MeshHandle>;

/// The data layout of a streaming request handle.
struct RequestData<T: Send, E: Send> {
    /// The current state of the request.
    state: AtomicU64,

    /// A general 'payload' field that is used as a back-channel so the streaming system can give
    /// a result back to the request creator. This is only sane to read in a terminal state.
    payload: UnsafeCell<MaybeUninit<Payload<T, E>>>,
}

// Safety: Request data will never _share_ the stored T. It is _owned_ while inside the request and
//         once stored inside the payload the T is either dropped while owned, or _moved_ out of the
//         the slot and sent to another thread.
unsafe impl<T: Send, E: Send> Sync for RequestData<T, E> {}

impl<T: Send, E: Send> RequestData<T, E> {
    pub(crate) fn get_state(&self) -> RequestState {
        let state = self.state.load(Ordering::Relaxed);
        // Safety: It is illegal to write a non-valid enum variant into 'state'
        unsafe { RequestState::from_u64(state).unwrap_unchecked() }
    }
}

impl<T: Send, E: Send> Drop for RequestData<T, E> {
    fn drop(&mut self) {
        // If we don't need to drop any of the stored types just bail immediately
        if !needs_drop::<T>() && !needs_drop::<E>() {
            return;
        }

        let state = self.get_state();

        match state {
            RequestState::Complete => {
                // Safety: It is a bug if we are ever in the complete state and payload is
                //         uninitialized.
                unsafe {
                    // If the request was complete but nobody consumed the output we need to drop the
                    // payload ourselves.
                    let payload = self.payload.get_mut().assume_init_mut();
                    ManuallyDrop::drop(&mut payload.success);
                }
            }
            RequestState::Failed => {
                // Safety: It is a bug if we are ever in the complete state and payload is
                //         uninitialized.
                unsafe {
                    // If the request was complete but nobody consumed the output we need to drop the
                    // payload ourselves.
                    let payload = self.payload.get_mut().assume_init_mut();
                    ManuallyDrop::drop(&mut payload.fail);
                }
            }
            _ => {}
        }
    }
}

union Payload<T: Send, E: Send> {
    success: ManuallyDrop<T>,
    fail: ManuallyDrop<E>,
}

#[cfg(test)]
mod tests {
    use crate::{PollCompleteError, PollFailError, StreamingRequest};

    fn make_request() -> StreamingRequest<u64, f32> {
        StreamingRequest::new()
    }

    #[test]
    pub fn request_to_completion() {
        let request = make_request();

        assert_eq!(
            request.poll_complete().unwrap_err(),
            PollCompleteError::Waiting
        );

        // Complete the request, will panic if we fail
        request.mark_complete(56).unwrap();

        // Unwrap the request
        assert_eq!(request.poll_fail().unwrap_err(), PollFailError::Complete);

        // Unwrap the request with the payload
        assert_eq!(request.poll_complete().unwrap(), 56);
    }

    #[test]
    pub fn request_to_failure() {
        let request = make_request();

        // Fail the request, will panic if we fail this call
        request.mark_failed(1234.5).unwrap();

        // Unwrap the request
        assert_eq!(
            request.poll_complete().unwrap_err(),
            PollCompleteError::Failed
        );

        assert_eq!(request.poll_fail(), Ok(1234.5));
    }

    #[test]
    pub fn request_to_cancelled() {
        let request = make_request();

        // Cancel the request, will panic if we fail
        request.mark_cancelled().unwrap();

        // Unwrap the request
        assert_eq!(
            request.poll_complete().unwrap_err(),
            PollCompleteError::Cancelled
        );
        // Unwrap the request
        assert_eq!(request.poll_fail().unwrap_err(), PollFailError::Cancelled);
    }

    #[test]
    pub fn request_bad_transition_success() {
        let request = make_request();

        // Complete the request
        request.mark_complete(56).unwrap();

        // Complete it again, should fail as the state machine won't allow the transition
        assert!(request.mark_complete(21).is_err());
    }

    #[test]
    pub fn request_bad_transition_cancel_complete() {
        let request = make_request();

        // Cancel the request
        request.mark_cancelled().unwrap();

        // Try to complete it after cancelling, should fail as the state machine won't allow the
        // transition
        assert!(request.mark_complete(21).is_err());
    }

    #[test]
    pub fn request_bad_transition_complete_cancel() {
        let request = make_request();

        // Cancel the request
        request.mark_complete(21).unwrap();

        // Try to complete it after cancelling, should fail as the state machine won't allow the
        // transition
        assert!(request.mark_cancelled().is_err());
    }
}
