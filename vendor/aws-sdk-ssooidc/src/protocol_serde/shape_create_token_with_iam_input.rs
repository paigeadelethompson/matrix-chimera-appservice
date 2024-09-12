// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_token_with_iam_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_token_with_iam::CreateTokenWithIamInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.assertion {
        object.key("assertion").string(var_1.as_str());
    }
    if let Some(var_2) = &input.client_id {
        object.key("clientId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.code {
        object.key("code").string(var_3.as_str());
    }
    if let Some(var_4) = &input.code_verifier {
        object.key("codeVerifier").string(var_4.as_str());
    }
    if let Some(var_5) = &input.grant_type {
        object.key("grantType").string(var_5.as_str());
    }
    if let Some(var_6) = &input.redirect_uri {
        object.key("redirectUri").string(var_6.as_str());
    }
    if let Some(var_7) = &input.refresh_token {
        object.key("refreshToken").string(var_7.as_str());
    }
    if let Some(var_8) = &input.requested_token_type {
        object.key("requestedTokenType").string(var_8.as_str());
    }
    if let Some(var_9) = &input.scope {
        let mut array_10 = object.key("scope").start_array();
        for item_11 in var_9 {
            {
                array_10.value().string(item_11.as_str());
            }
        }
        array_10.finish();
    }
    if let Some(var_12) = &input.subject_token {
        object.key("subjectToken").string(var_12.as_str());
    }
    if let Some(var_13) = &input.subject_token_type {
        object.key("subjectTokenType").string(var_13.as_str());
    }
    Ok(())
}
