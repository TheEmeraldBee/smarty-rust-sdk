pub mod client;

pub mod lookup;

pub mod candidate;

// Tests
#[cfg(test)]
mod tests {
    use crate::sdk::batch::Batch;
    use crate::sdk::options::Options;
    use crate::us_zipcode_api::client::USZipcodeClient;
    use crate::us_zipcode_api::lookup::Lookup;

    #[test]
    fn client_test() {
        let options = Options::default();
        let client = USZipcodeClient::new(options).unwrap();

        assert_eq!(client.client.url.to_string(), "https://us-zipcode.api.smartystreets.com/lookup".to_string())
    }

    #[test]
    fn lookup_test() {
        let lookup = Lookup {
            city: "Provo".to_string(),
            state: "UT".to_string(),
            zipcode: "84604".to_string(),
            ..Default::default()
        };

        let expected_result = vec! [
            ("city".to_string(), "Provo".to_string()),
            ("state".to_string(), "UT".to_string()),
            ("zipcode".to_string(), "84604".to_string()),
        ];

        assert_eq!(lookup.to_param_array(), expected_result);
    }

    #[test]
    fn batch_test() {
        let lookup = Lookup {
            city: "Provo".to_string(),
            state: "UT".to_string(),
            zipcode: "84604".to_string(),
            ..Default::default()
        };

        let expected_result = vec! [
            ("city".to_string(), "Provo".to_string()),
            ("state".to_string(), "UT".to_string()),
            ("zipcode".to_string(), "84604".to_string()),
        ];

        let mut batch = Batch::default();
        batch.push(lookup.clone()).unwrap();
        batch.push(lookup.clone()).unwrap();
        batch.push(lookup.clone()).unwrap();

        assert_eq!(batch.records()[0].to_param_array(), expected_result);
        assert_eq!(batch.length(), 3);
    }
}