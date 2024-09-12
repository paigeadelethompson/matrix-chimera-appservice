// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_global_table_settings_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_global_table_settings::UpdateGlobalTableSettingsInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.global_table_name {
        object.key("GlobalTableName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.global_table_billing_mode {
        object.key("GlobalTableBillingMode").string(var_2.as_str());
    }
    if let Some(var_3) = &input.global_table_provisioned_write_capacity_units {
        object.key("GlobalTableProvisionedWriteCapacityUnits").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.global_table_provisioned_write_capacity_auto_scaling_settings_update {
        #[allow(unused_mut)]
        let mut object_5 = object.key("GlobalTableProvisionedWriteCapacityAutoScalingSettingsUpdate").start_object();
        crate::protocol_serde::shape_auto_scaling_settings_update::ser_auto_scaling_settings_update(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.global_table_global_secondary_index_settings_update {
        let mut array_7 = object.key("GlobalTableGlobalSecondaryIndexSettingsUpdate").start_array();
        for item_8 in var_6 {
            {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_global_table_global_secondary_index_settings_update::ser_global_table_global_secondary_index_settings_update(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    if let Some(var_10) = &input.replica_settings_update {
        let mut array_11 = object.key("ReplicaSettingsUpdate").start_array();
        for item_12 in var_10 {
            {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::protocol_serde::shape_replica_settings_update::ser_replica_settings_update(&mut object_13, item_12)?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    Ok(())
}
