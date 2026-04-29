use google_calendar3::CalendarHub;
use hyper_rustls::HttpsConnectorBuilder;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use yup_oauth2::{ read_service_account_key, ServiceAccountAuthenticator };

pub async fn get_hub() -> CalendarHub<hyper_rustls::HttpsConnector<HttpConnector>> {
    let secret = read_service_account_key("credentials.json").await.expect(
        "Failed to read credentials.json. Make sure the file exists!"
    );

    let auth = ServiceAccountAuthenticator::builder(secret)
        .build().await
        .expect("Failed to create Google authenticator");

    let client = Client::builder(TokioExecutor::new()).build(
        HttpsConnectorBuilder::new()
            .with_native_roots()
            .expect("Failed to create HTTPS connector")
            .https_or_http()
            .enable_http1()
            .build()
    );

    CalendarHub::new(client, auth)
}
