// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StopSpeakerSearchTaskOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for StopSpeakerSearchTaskOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl StopSpeakerSearchTaskOutput {
    /// Creates a new builder-style object to manufacture [`StopSpeakerSearchTaskOutput`](crate::operation::stop_speaker_search_task::StopSpeakerSearchTaskOutput).
    pub fn builder() -> crate::operation::stop_speaker_search_task::builders::StopSpeakerSearchTaskOutputBuilder {
        crate::operation::stop_speaker_search_task::builders::StopSpeakerSearchTaskOutputBuilder::default()
    }
}

/// A builder for [`StopSpeakerSearchTaskOutput`](crate::operation::stop_speaker_search_task::StopSpeakerSearchTaskOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct StopSpeakerSearchTaskOutputBuilder {
    _request_id: Option<String>,
}
impl StopSpeakerSearchTaskOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`StopSpeakerSearchTaskOutput`](crate::operation::stop_speaker_search_task::StopSpeakerSearchTaskOutput).
    pub fn build(self) -> crate::operation::stop_speaker_search_task::StopSpeakerSearchTaskOutput {
        crate::operation::stop_speaker_search_task::StopSpeakerSearchTaskOutput {
            _request_id: self._request_id,
        }
    }
}