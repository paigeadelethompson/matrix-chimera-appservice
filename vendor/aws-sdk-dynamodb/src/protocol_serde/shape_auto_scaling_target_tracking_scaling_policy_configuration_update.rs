// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_auto_scaling_target_tracking_scaling_policy_configuration_update(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AutoScalingTargetTrackingScalingPolicyConfigurationUpdate,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.disable_scale_in {
        object.key("DisableScaleIn").boolean(*var_1);
    }
    if let Some(var_2) = &input.scale_in_cooldown {
        object.key("ScaleInCooldown").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.scale_out_cooldown {
        object.key("ScaleOutCooldown").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    {
        object.key("TargetValue").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((input.target_value).into()),
        );
    }
    Ok(())
}
