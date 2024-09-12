// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_register_client_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::register_client::RegisterClientInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.client_name {
        object.key("clientName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.client_type {
        object.key("clientType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.entitled_application_arn {
        object.key("entitledApplicationArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.grant_types {
        let mut array_5 = object.key("grantTypes").start_array();
        for item_6 in var_4 {
            {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.issuer_url {
        object.key("issuerUrl").string(var_7.as_str());
    }
    if let Some(var_8) = &input.redirect_uris {
        let mut array_9 = object.key("redirectUris").start_array();
        for item_10 in var_8 {
            {
                array_9.value().string(item_10.as_str());
            }
        }
        array_9.finish();
    }
    if let Some(var_11) = &input.scopes {
        let mut array_12 = object.key("scopes").start_array();
        for item_13 in var_11 {
            {
                array_12.value().string(item_13.as_str());
            }
        }
        array_12.finish();
    }
    Ok(())
}
