use crate::sdk::client::Client;
use crate::sdk::error::SmartyError;
use crate::sdk::options::Options;
use crate::sdk::send_request;
use crate::us_autocomplete_pro_api::lookup::Lookup;
use crate::us_autocomplete_pro_api::suggestion::SuggestionListing;
use reqwest::Method;
use smarty_rust_proc_macro::smarty_api;

#[smarty_api(
    api_path = "lookup",
    default_url = "https://us-autocomplete-pro.api.smarty.com/",
    lookup_style(lookup),
    lookup_type = "Lookup",
    result_type = "SuggestionListing"
)]
pub struct USAutocompleteProClient;
