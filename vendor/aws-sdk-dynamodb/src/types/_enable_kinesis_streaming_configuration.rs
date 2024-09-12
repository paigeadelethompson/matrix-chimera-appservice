// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Enables setting the configuration for Kinesis Streaming.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EnableKinesisStreamingConfiguration {
    /// <p>Toggle for the precision of Kinesis data stream timestamp. The values are either <code>MILLISECOND</code> or <code>MICROSECOND</code>.</p>
    pub approximate_creation_date_time_precision: ::std::option::Option<crate::types::ApproximateCreationDateTimePrecision>,
}
impl EnableKinesisStreamingConfiguration {
    /// <p>Toggle for the precision of Kinesis data stream timestamp. The values are either <code>MILLISECOND</code> or <code>MICROSECOND</code>.</p>
    pub fn approximate_creation_date_time_precision(&self) -> ::std::option::Option<&crate::types::ApproximateCreationDateTimePrecision> {
        self.approximate_creation_date_time_precision.as_ref()
    }
}
impl EnableKinesisStreamingConfiguration {
    /// Creates a new builder-style object to manufacture [`EnableKinesisStreamingConfiguration`](crate::types::EnableKinesisStreamingConfiguration).
    pub fn builder() -> crate::types::builders::EnableKinesisStreamingConfigurationBuilder {
        crate::types::builders::EnableKinesisStreamingConfigurationBuilder::default()
    }
}

/// A builder for [`EnableKinesisStreamingConfiguration`](crate::types::EnableKinesisStreamingConfiguration).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct EnableKinesisStreamingConfigurationBuilder {
    pub(crate) approximate_creation_date_time_precision: ::std::option::Option<crate::types::ApproximateCreationDateTimePrecision>,
}
impl EnableKinesisStreamingConfigurationBuilder {
    /// <p>Toggle for the precision of Kinesis data stream timestamp. The values are either <code>MILLISECOND</code> or <code>MICROSECOND</code>.</p>
    pub fn approximate_creation_date_time_precision(mut self, input: crate::types::ApproximateCreationDateTimePrecision) -> Self {
        self.approximate_creation_date_time_precision = ::std::option::Option::Some(input);
        self
    }
    /// <p>Toggle for the precision of Kinesis data stream timestamp. The values are either <code>MILLISECOND</code> or <code>MICROSECOND</code>.</p>
    pub fn set_approximate_creation_date_time_precision(
        mut self,
        input: ::std::option::Option<crate::types::ApproximateCreationDateTimePrecision>,
    ) -> Self {
        self.approximate_creation_date_time_precision = input;
        self
    }
    /// <p>Toggle for the precision of Kinesis data stream timestamp. The values are either <code>MILLISECOND</code> or <code>MICROSECOND</code>.</p>
    pub fn get_approximate_creation_date_time_precision(&self) -> &::std::option::Option<crate::types::ApproximateCreationDateTimePrecision> {
        &self.approximate_creation_date_time_precision
    }
    /// Consumes the builder and constructs a [`EnableKinesisStreamingConfiguration`](crate::types::EnableKinesisStreamingConfiguration).
    pub fn build(self) -> crate::types::EnableKinesisStreamingConfiguration {
        crate::types::EnableKinesisStreamingConfiguration {
            approximate_creation_date_time_precision: self.approximate_creation_date_time_precision,
        }
    }
}
