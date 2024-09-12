// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the properties of a target tracking scaling policy.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AutoScalingTargetTrackingScalingPolicyConfigurationDescription {
    /// <p>Indicates whether scale in by the target tracking policy is disabled. If the value is true, scale in is disabled and the target tracking policy won't remove capacity from the scalable resource. Otherwise, scale in is enabled and the target tracking policy can remove capacity from the scalable resource. The default value is false.</p>
    pub disable_scale_in: ::std::option::Option<bool>,
    /// <p>The amount of time, in seconds, after a scale in activity completes before another scale in activity can start. The cooldown period is used to block subsequent scale in requests until it has expired. You should scale in conservatively to protect your application's availability. However, if another alarm triggers a scale out policy during the cooldown period after a scale-in, application auto scaling scales out your scalable target immediately.</p>
    pub scale_in_cooldown: ::std::option::Option<i32>,
    /// <p>The amount of time, in seconds, after a scale out activity completes before another scale out activity can start. While the cooldown period is in effect, the capacity that has been added by the previous scale out event that initiated the cooldown is calculated as part of the desired capacity for the next scale out. You should continuously (but not excessively) scale out.</p>
    pub scale_out_cooldown: ::std::option::Option<i32>,
    /// <p>The target value for the metric. The range is 8.515920e-109 to 1.174271e+108 (Base 10) or 2e-360 to 2e360 (Base 2).</p>
    pub target_value: f64,
}
impl AutoScalingTargetTrackingScalingPolicyConfigurationDescription {
    /// <p>Indicates whether scale in by the target tracking policy is disabled. If the value is true, scale in is disabled and the target tracking policy won't remove capacity from the scalable resource. Otherwise, scale in is enabled and the target tracking policy can remove capacity from the scalable resource. The default value is false.</p>
    pub fn disable_scale_in(&self) -> ::std::option::Option<bool> {
        self.disable_scale_in
    }
    /// <p>The amount of time, in seconds, after a scale in activity completes before another scale in activity can start. The cooldown period is used to block subsequent scale in requests until it has expired. You should scale in conservatively to protect your application's availability. However, if another alarm triggers a scale out policy during the cooldown period after a scale-in, application auto scaling scales out your scalable target immediately.</p>
    pub fn scale_in_cooldown(&self) -> ::std::option::Option<i32> {
        self.scale_in_cooldown
    }
    /// <p>The amount of time, in seconds, after a scale out activity completes before another scale out activity can start. While the cooldown period is in effect, the capacity that has been added by the previous scale out event that initiated the cooldown is calculated as part of the desired capacity for the next scale out. You should continuously (but not excessively) scale out.</p>
    pub fn scale_out_cooldown(&self) -> ::std::option::Option<i32> {
        self.scale_out_cooldown
    }
    /// <p>The target value for the metric. The range is 8.515920e-109 to 1.174271e+108 (Base 10) or 2e-360 to 2e360 (Base 2).</p>
    pub fn target_value(&self) -> f64 {
        self.target_value
    }
}
impl AutoScalingTargetTrackingScalingPolicyConfigurationDescription {
    /// Creates a new builder-style object to manufacture [`AutoScalingTargetTrackingScalingPolicyConfigurationDescription`](crate::types::AutoScalingTargetTrackingScalingPolicyConfigurationDescription).
    pub fn builder() -> crate::types::builders::AutoScalingTargetTrackingScalingPolicyConfigurationDescriptionBuilder {
        crate::types::builders::AutoScalingTargetTrackingScalingPolicyConfigurationDescriptionBuilder::default()
    }
}

/// A builder for [`AutoScalingTargetTrackingScalingPolicyConfigurationDescription`](crate::types::AutoScalingTargetTrackingScalingPolicyConfigurationDescription).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct AutoScalingTargetTrackingScalingPolicyConfigurationDescriptionBuilder {
    pub(crate) disable_scale_in: ::std::option::Option<bool>,
    pub(crate) scale_in_cooldown: ::std::option::Option<i32>,
    pub(crate) scale_out_cooldown: ::std::option::Option<i32>,
    pub(crate) target_value: ::std::option::Option<f64>,
}
impl AutoScalingTargetTrackingScalingPolicyConfigurationDescriptionBuilder {
    /// <p>Indicates whether scale in by the target tracking policy is disabled. If the value is true, scale in is disabled and the target tracking policy won't remove capacity from the scalable resource. Otherwise, scale in is enabled and the target tracking policy can remove capacity from the scalable resource. The default value is false.</p>
    pub fn disable_scale_in(mut self, input: bool) -> Self {
        self.disable_scale_in = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether scale in by the target tracking policy is disabled. If the value is true, scale in is disabled and the target tracking policy won't remove capacity from the scalable resource. Otherwise, scale in is enabled and the target tracking policy can remove capacity from the scalable resource. The default value is false.</p>
    pub fn set_disable_scale_in(mut self, input: ::std::option::Option<bool>) -> Self {
        self.disable_scale_in = input;
        self
    }
    /// <p>Indicates whether scale in by the target tracking policy is disabled. If the value is true, scale in is disabled and the target tracking policy won't remove capacity from the scalable resource. Otherwise, scale in is enabled and the target tracking policy can remove capacity from the scalable resource. The default value is false.</p>
    pub fn get_disable_scale_in(&self) -> &::std::option::Option<bool> {
        &self.disable_scale_in
    }
    /// <p>The amount of time, in seconds, after a scale in activity completes before another scale in activity can start. The cooldown period is used to block subsequent scale in requests until it has expired. You should scale in conservatively to protect your application's availability. However, if another alarm triggers a scale out policy during the cooldown period after a scale-in, application auto scaling scales out your scalable target immediately.</p>
    pub fn scale_in_cooldown(mut self, input: i32) -> Self {
        self.scale_in_cooldown = ::std::option::Option::Some(input);
        self
    }
    /// <p>The amount of time, in seconds, after a scale in activity completes before another scale in activity can start. The cooldown period is used to block subsequent scale in requests until it has expired. You should scale in conservatively to protect your application's availability. However, if another alarm triggers a scale out policy during the cooldown period after a scale-in, application auto scaling scales out your scalable target immediately.</p>
    pub fn set_scale_in_cooldown(mut self, input: ::std::option::Option<i32>) -> Self {
        self.scale_in_cooldown = input;
        self
    }
    /// <p>The amount of time, in seconds, after a scale in activity completes before another scale in activity can start. The cooldown period is used to block subsequent scale in requests until it has expired. You should scale in conservatively to protect your application's availability. However, if another alarm triggers a scale out policy during the cooldown period after a scale-in, application auto scaling scales out your scalable target immediately.</p>
    pub fn get_scale_in_cooldown(&self) -> &::std::option::Option<i32> {
        &self.scale_in_cooldown
    }
    /// <p>The amount of time, in seconds, after a scale out activity completes before another scale out activity can start. While the cooldown period is in effect, the capacity that has been added by the previous scale out event that initiated the cooldown is calculated as part of the desired capacity for the next scale out. You should continuously (but not excessively) scale out.</p>
    pub fn scale_out_cooldown(mut self, input: i32) -> Self {
        self.scale_out_cooldown = ::std::option::Option::Some(input);
        self
    }
    /// <p>The amount of time, in seconds, after a scale out activity completes before another scale out activity can start. While the cooldown period is in effect, the capacity that has been added by the previous scale out event that initiated the cooldown is calculated as part of the desired capacity for the next scale out. You should continuously (but not excessively) scale out.</p>
    pub fn set_scale_out_cooldown(mut self, input: ::std::option::Option<i32>) -> Self {
        self.scale_out_cooldown = input;
        self
    }
    /// <p>The amount of time, in seconds, after a scale out activity completes before another scale out activity can start. While the cooldown period is in effect, the capacity that has been added by the previous scale out event that initiated the cooldown is calculated as part of the desired capacity for the next scale out. You should continuously (but not excessively) scale out.</p>
    pub fn get_scale_out_cooldown(&self) -> &::std::option::Option<i32> {
        &self.scale_out_cooldown
    }
    /// <p>The target value for the metric. The range is 8.515920e-109 to 1.174271e+108 (Base 10) or 2e-360 to 2e360 (Base 2).</p>
    /// This field is required.
    pub fn target_value(mut self, input: f64) -> Self {
        self.target_value = ::std::option::Option::Some(input);
        self
    }
    /// <p>The target value for the metric. The range is 8.515920e-109 to 1.174271e+108 (Base 10) or 2e-360 to 2e360 (Base 2).</p>
    pub fn set_target_value(mut self, input: ::std::option::Option<f64>) -> Self {
        self.target_value = input;
        self
    }
    /// <p>The target value for the metric. The range is 8.515920e-109 to 1.174271e+108 (Base 10) or 2e-360 to 2e360 (Base 2).</p>
    pub fn get_target_value(&self) -> &::std::option::Option<f64> {
        &self.target_value
    }
    /// Consumes the builder and constructs a [`AutoScalingTargetTrackingScalingPolicyConfigurationDescription`](crate::types::AutoScalingTargetTrackingScalingPolicyConfigurationDescription).
    /// This method will fail if any of the following fields are not set:
    /// - [`target_value`](crate::types::builders::AutoScalingTargetTrackingScalingPolicyConfigurationDescriptionBuilder::target_value)
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::types::AutoScalingTargetTrackingScalingPolicyConfigurationDescription,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::types::AutoScalingTargetTrackingScalingPolicyConfigurationDescription {
            disable_scale_in: self.disable_scale_in,
            scale_in_cooldown: self.scale_in_cooldown,
            scale_out_cooldown: self.scale_out_cooldown,
            target_value: self.target_value.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "target_value",
                    "target_value was not specified but it is required when building AutoScalingTargetTrackingScalingPolicyConfigurationDescription",
                )
            })?,
        })
    }
}
