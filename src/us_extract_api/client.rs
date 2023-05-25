use reqwest::Method;
use url::{ParseError, Url};
use crate::sdk::client::Client;
use crate::sdk::error::SDKError;
use crate::sdk::options::Options;
use crate::sdk::send_request;
use crate::us_extract_api::extraction::ExtractionResult;
use crate::us_extract_api::lookup::Lookup;

pub struct USExtractClient {
    pub(crate) client: Client
}

impl USExtractClient {
    pub fn new(options: Options) -> Result<Self, ParseError> {
        Self::new_custom_base_url("https://us-extract.api.smartystreets.com/".parse()?, options)
    }

    pub fn new_custom_base_url(base_url: Url, options: Options) -> Result<Self, ParseError> {
        Ok(Self { client: Client::new(base_url, options, "")? })
    }

    pub async fn send(&self, lookup: &mut Lookup) -> Result<(), SDKError> {
        let mut req = self.client.reqwest_client.request(Method::POST, self.client.url.clone());
        req = self.client.build_request(req);
        req = req.header("Content-Type", "text/plain");
        //TODO: Implement this as params.
        //req = req.query(&lookup.clone().into_param_array());
        req = req.body(lookup.text.clone());

        let response = send_request(req).await?;

        let result = match response.json::<ExtractionResult>().await {
            Ok(result) => result,
            Err(err) => { return Err(SDKError { code: None, detail: Some(format!("{:?}", err)) }) }
        };

        lookup.result = result;

        Ok(())
    }
}
