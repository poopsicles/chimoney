use crate::{helpers::transform_response, schema::responses::AirtimeCountriesResponse};

use super::{ChimoneyClient, ChimoneyError, ChimoneyResult};

impl<'a> ChimoneyClient<'a> {
    /// Get a list of all supported airtime countries.
    pub async fn get_airtime_countries(&self) -> ChimoneyResult<AirtimeCountriesResponse> {
        match self
            .reqwest_client
            .get(format!("{}{}", self.server, "/v0.2/info/airtime-countries"))
            .send()
            .await
        {
            Err(e) => Err(ChimoneyError::Generic(e.to_string())),
            Ok(r) => transform_response(r).await,
        }
    }
}
