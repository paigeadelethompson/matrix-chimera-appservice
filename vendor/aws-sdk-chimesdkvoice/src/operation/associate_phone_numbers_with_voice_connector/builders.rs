// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::associate_phone_numbers_with_voice_connector::_associate_phone_numbers_with_voice_connector_output::AssociatePhoneNumbersWithVoiceConnectorOutputBuilder;

pub use crate::operation::associate_phone_numbers_with_voice_connector::_associate_phone_numbers_with_voice_connector_input::AssociatePhoneNumbersWithVoiceConnectorInputBuilder;

impl crate::operation::associate_phone_numbers_with_voice_connector::builders::AssociatePhoneNumbersWithVoiceConnectorInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::associate_phone_numbers_with_voice_connector::AssociatePhoneNumbersWithVoiceConnectorOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::associate_phone_numbers_with_voice_connector::AssociatePhoneNumbersWithVoiceConnectorError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.associate_phone_numbers_with_voice_connector();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AssociatePhoneNumbersWithVoiceConnector`.
///
/// <p>Associates phone numbers with the specified Amazon Chime SDK Voice Connector.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AssociatePhoneNumbersWithVoiceConnectorFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::associate_phone_numbers_with_voice_connector::builders::AssociatePhoneNumbersWithVoiceConnectorInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::associate_phone_numbers_with_voice_connector::AssociatePhoneNumbersWithVoiceConnectorOutput,
        crate::operation::associate_phone_numbers_with_voice_connector::AssociatePhoneNumbersWithVoiceConnectorError,
    > for AssociatePhoneNumbersWithVoiceConnectorFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::associate_phone_numbers_with_voice_connector::AssociatePhoneNumbersWithVoiceConnectorOutput,
            crate::operation::associate_phone_numbers_with_voice_connector::AssociatePhoneNumbersWithVoiceConnectorError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AssociatePhoneNumbersWithVoiceConnectorFluentBuilder {
    /// Creates a new `AssociatePhoneNumbersWithVoiceConnectorFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AssociatePhoneNumbersWithVoiceConnector as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::associate_phone_numbers_with_voice_connector::builders::AssociatePhoneNumbersWithVoiceConnectorInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::associate_phone_numbers_with_voice_connector::AssociatePhoneNumbersWithVoiceConnectorOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::associate_phone_numbers_with_voice_connector::AssociatePhoneNumbersWithVoiceConnectorError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::associate_phone_numbers_with_voice_connector::AssociatePhoneNumbersWithVoiceConnector::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::associate_phone_numbers_with_voice_connector::AssociatePhoneNumbersWithVoiceConnector::orchestrate(&runtime_plugins, input)
            .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::associate_phone_numbers_with_voice_connector::AssociatePhoneNumbersWithVoiceConnectorOutput,
        crate::operation::associate_phone_numbers_with_voice_connector::AssociatePhoneNumbersWithVoiceConnectorError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The Voice Connector ID.</p>
    pub fn voice_connector_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.voice_connector_id(input.into());
        self
    }
    /// <p>The Voice Connector ID.</p>
    pub fn set_voice_connector_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_voice_connector_id(input);
        self
    }
    /// <p>The Voice Connector ID.</p>
    pub fn get_voice_connector_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_voice_connector_id()
    }
    ///
    /// Appends an item to `E164PhoneNumbers`.
    ///
    /// To override the contents of this collection use [`set_e164_phone_numbers`](Self::set_e164_phone_numbers).
    ///
    /// <p>List of phone numbers, in E.164 format.</p>
    pub fn e164_phone_numbers(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.e164_phone_numbers(input.into());
        self
    }
    /// <p>List of phone numbers, in E.164 format.</p>
    pub fn set_e164_phone_numbers(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_e164_phone_numbers(input);
        self
    }
    /// <p>List of phone numbers, in E.164 format.</p>
    pub fn get_e164_phone_numbers(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_e164_phone_numbers()
    }
    /// <p>If true, associates the provided phone numbers with the provided Amazon Chime SDK Voice Connector and removes any previously existing associations. If false, does not associate any phone numbers that have previously existing associations.</p>
    pub fn force_associate(mut self, input: bool) -> Self {
        self.inner = self.inner.force_associate(input);
        self
    }
    /// <p>If true, associates the provided phone numbers with the provided Amazon Chime SDK Voice Connector and removes any previously existing associations. If false, does not associate any phone numbers that have previously existing associations.</p>
    pub fn set_force_associate(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_force_associate(input);
        self
    }
    /// <p>If true, associates the provided phone numbers with the provided Amazon Chime SDK Voice Connector and removes any previously existing associations. If false, does not associate any phone numbers that have previously existing associations.</p>
    pub fn get_force_associate(&self) -> &::std::option::Option<bool> {
        self.inner.get_force_associate()
    }
}