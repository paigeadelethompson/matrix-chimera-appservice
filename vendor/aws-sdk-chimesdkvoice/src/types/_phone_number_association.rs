// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The phone number associations, such as an Amazon Chime SDK account ID, user ID, Voice Connector ID, or Voice Connector group ID.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PhoneNumberAssociation {
    /// <p>Contains the ID for the entity specified in Name.</p>
    pub value: ::std::option::Option<::std::string::String>,
    /// <p>Defines the association with an Amazon Chime SDK account ID, user ID, Voice Connector ID, or Voice Connector group ID.</p>
    pub name: ::std::option::Option<crate::types::PhoneNumberAssociationName>,
    /// <p>The timestamp of the phone number association, in ISO 8601 format.</p>
    pub associated_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl PhoneNumberAssociation {
    /// <p>Contains the ID for the entity specified in Name.</p>
    pub fn value(&self) -> ::std::option::Option<&str> {
        self.value.as_deref()
    }
    /// <p>Defines the association with an Amazon Chime SDK account ID, user ID, Voice Connector ID, or Voice Connector group ID.</p>
    pub fn name(&self) -> ::std::option::Option<&crate::types::PhoneNumberAssociationName> {
        self.name.as_ref()
    }
    /// <p>The timestamp of the phone number association, in ISO 8601 format.</p>
    pub fn associated_timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.associated_timestamp.as_ref()
    }
}
impl PhoneNumberAssociation {
    /// Creates a new builder-style object to manufacture [`PhoneNumberAssociation`](crate::types::PhoneNumberAssociation).
    pub fn builder() -> crate::types::builders::PhoneNumberAssociationBuilder {
        crate::types::builders::PhoneNumberAssociationBuilder::default()
    }
}

/// A builder for [`PhoneNumberAssociation`](crate::types::PhoneNumberAssociation).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct PhoneNumberAssociationBuilder {
    pub(crate) value: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<crate::types::PhoneNumberAssociationName>,
    pub(crate) associated_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl PhoneNumberAssociationBuilder {
    /// <p>Contains the ID for the entity specified in Name.</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Contains the ID for the entity specified in Name.</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// <p>Contains the ID for the entity specified in Name.</p>
    pub fn get_value(&self) -> &::std::option::Option<::std::string::String> {
        &self.value
    }
    /// <p>Defines the association with an Amazon Chime SDK account ID, user ID, Voice Connector ID, or Voice Connector group ID.</p>
    pub fn name(mut self, input: crate::types::PhoneNumberAssociationName) -> Self {
        self.name = ::std::option::Option::Some(input);
        self
    }
    /// <p>Defines the association with an Amazon Chime SDK account ID, user ID, Voice Connector ID, or Voice Connector group ID.</p>
    pub fn set_name(mut self, input: ::std::option::Option<crate::types::PhoneNumberAssociationName>) -> Self {
        self.name = input;
        self
    }
    /// <p>Defines the association with an Amazon Chime SDK account ID, user ID, Voice Connector ID, or Voice Connector group ID.</p>
    pub fn get_name(&self) -> &::std::option::Option<crate::types::PhoneNumberAssociationName> {
        &self.name
    }
    /// <p>The timestamp of the phone number association, in ISO 8601 format.</p>
    pub fn associated_timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.associated_timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp of the phone number association, in ISO 8601 format.</p>
    pub fn set_associated_timestamp(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.associated_timestamp = input;
        self
    }
    /// <p>The timestamp of the phone number association, in ISO 8601 format.</p>
    pub fn get_associated_timestamp(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.associated_timestamp
    }
    /// Consumes the builder and constructs a [`PhoneNumberAssociation`](crate::types::PhoneNumberAssociation).
    pub fn build(self) -> crate::types::PhoneNumberAssociation {
        crate::types::PhoneNumberAssociation {
            value: self.value,
            name: self.name,
            associated_timestamp: self.associated_timestamp,
        }
    }
}
