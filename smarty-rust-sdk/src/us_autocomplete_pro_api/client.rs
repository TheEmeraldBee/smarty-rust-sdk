use crate::sdk::client::Client;
use crate::sdk::error::SDKError;
use crate::sdk::options::Options;
use crate::sdk::send_request;
use crate::us_autocomplete_pro_api::lookup::Lookup;
use crate::us_autocomplete_pro_api::suggestion::SuggestionListing;
use reqwest::Method;
use smarty_rust_proc_macro::smarty_api;
use url::{ParseError, Url};

#[smarty_api(
    api_path = "lookup",
    default_url = "https://us-autocomplete-pro.api.smartystreets.com/",
    lookup_style(lookup),
    lookup_type = "Lookup",
    result_type = "SuggestionListing"
)]
pub struct USAutocompleteProClient;
