use serde::Serialize;
use crate::us_extract_api::extraction::ExtractionResult;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Lookup {
    pub text: String,
    pub html: HTMLPayload,
    pub aggressive: bool,
    #[serde(rename = "addr_line_breaks")]
    pub addresses_with_line_breaks: bool, // addr_line_breaks
    #[serde(rename = "addr_per_line")]
    pub addresses_per_line: i32, //addr_per_line
    #[serde(skip_serializing)]
    pub result: ExtractionResult
}

impl Default for Lookup {
    fn default() -> Self {
        Lookup {
            text: String::default(),
            html: HTMLPayload::HTMLUnspecified,
            aggressive: false,
            addresses_with_line_breaks: false,
            addresses_per_line: 1,
            result: ExtractionResult::default()
        }
    }
}

// TODO: Implement this like the one in the python sdk.
// impl Lookup {
//     pub(crate) fn into_param_array(self) -> Vec<(String, String)> {
//         let mut max_candidates_string = self.max_candidates.to_string();
//
//         if self.max_candidates <= 0 {
//             max_candidates_string = String::default();
//         }
//
//         if self.match_strategy == MatchStrategy::Enhanced {
//             max_candidates_string = 5.to_string();
//         }
//
//         vec![
//             has_param("street".to_string(), self.street),
//             has_param("street2".to_string(), self.street2),
//             has_param("secondary".to_string(), self.secondary),
//             has_param("city".to_string(), self.city),
//             has_param("state".to_string(), self.state),
//             has_param("zipcode".to_string(), self.zipcode),
//             has_param("lastline".to_string(), self.last_line),
//             has_param("adressee".to_string(), self.adressee),
//             has_param("urbanization".to_string(), self.urbanization),
//             has_param("input_id".to_string(), self.input_id),
//             has_param("candidates".to_string(), max_candidates_string),
//             has_param("match".to_string(), self.match_strategy.to_string()),
//         ].iter()
//             .filter_map(Option::clone)
//             .collect::<Vec<_>>()
//     }
// }

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum HTMLPayload {
    #[serde(rename = "")]
    HTMLUnspecified,
    #[serde(rename = "true")]
    HTMLYes,
    #[serde(rename = "false")]
    HTMLNo
}