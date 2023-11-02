#[cfg(feature = "ssr")]
use {
    std::env, twilio::apis::api20100401_message_api::*, twilio::apis::configuration::*,
    twilio::models::ApiPeriodV2010PeriodAccountPeriodMessage,
};

#[cfg(feature = "ssr")]
pub async fn send_message(
    message: String,
    to: String,
) -> Option<ApiPeriodV2010PeriodAccountPeriodMessage> {
    let account_sid =
        env::var("TWILIO_ACCOUNT_SID").expect("env variable `TWILIO_ACCOUNT_SID` should be set");
    let api_key = env::var("TWILIO_API_KEY").expect("env variable `TWILIO_API_KEY` should be set");
    let api_key_secret = env::var("TWILIO_API_KEY_SECRET")
        .expect("env variable `TWILIO_API_KEY_SECRET` should be set");
    let from =
        env::var("TWILIO_PHONE_NUMBER").expect("env variable `TWILIO_PHONE_NUMBER` should be set");

    // Create a new configuration for your Twilio client.
    let twilio_config = Configuration {
        basic_auth: Some((api_key, Some(api_key_secret))),
        ..Default::default()
    };

    // Define the message that you wish to send
    let message_params = CreateMessageParams {
        account_sid,
        to,
        from: Some(from),
        body: Some(message),
        ..Default::default()
    };

    // Asynchronously send the message from your Twilio phone number.
    create_message(&twilio_config, message_params).await.ok()
}
