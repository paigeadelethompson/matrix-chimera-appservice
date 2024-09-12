// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartSpeakerSearchTaskOutput {
    /// <p>The details of the speaker search task.</p>
    pub speaker_search_task: ::std::option::Option<crate::types::SpeakerSearchTask>,
    _request_id: Option<String>,
}
impl StartSpeakerSearchTaskOutput {
    /// <p>The details of the speaker search task.</p>
    pub fn speaker_search_task(&self) -> ::std::option::Option<&crate::types::SpeakerSearchTask> {
        self.speaker_search_task.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for StartSpeakerSearchTaskOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl StartSpeakerSearchTaskOutput {
    /// Creates a new builder-style object to manufacture [`StartSpeakerSearchTaskOutput`](crate::operation::start_speaker_search_task::StartSpeakerSearchTaskOutput).
    pub fn builder() -> crate::operation::start_speaker_search_task::builders::StartSpeakerSearchTaskOutputBuilder {
        crate::operation::start_speaker_search_task::builders::StartSpeakerSearchTaskOutputBuilder::default()
    }
}

/// A builder for [`StartSpeakerSearchTaskOutput`](crate::operation::start_speaker_search_task::StartSpeakerSearchTaskOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct StartSpeakerSearchTaskOutputBuilder {
    pub(crate) speaker_search_task: ::std::option::Option<crate::types::SpeakerSearchTask>,
    _request_id: Option<String>,
}
impl StartSpeakerSearchTaskOutputBuilder {
    /// <p>The details of the speaker search task.</p>
    pub fn speaker_search_task(mut self, input: crate::types::SpeakerSearchTask) -> Self {
        self.speaker_search_task = ::std::option::Option::Some(input);
        self
    }
    /// <p>The details of the speaker search task.</p>
    pub fn set_speaker_search_task(mut self, input: ::std::option::Option<crate::types::SpeakerSearchTask>) -> Self {
        self.speaker_search_task = input;
        self
    }
    /// <p>The details of the speaker search task.</p>
    pub fn get_speaker_search_task(&self) -> &::std::option::Option<crate::types::SpeakerSearchTask> {
        &self.speaker_search_task
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`StartSpeakerSearchTaskOutput`](crate::operation::start_speaker_search_task::StartSpeakerSearchTaskOutput).
    pub fn build(self) -> crate::operation::start_speaker_search_task::StartSpeakerSearchTaskOutput {
        crate::operation::start_speaker_search_task::StartSpeakerSearchTaskOutput {
            speaker_search_task: self.speaker_search_task,
            _request_id: self._request_id,
        }
    }
}
