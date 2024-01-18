use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;

use crate::{
    client::{ChimoneyError, ChimoneyResult},
    schema::responses::{ErrorResponse, GoodResponse},
};

pub async fn transform_response<T: DeserializeOwned>(r: Response) -> ChimoneyResult<T> {
    match r.status() {
        StatusCode::OK => r.json::<GoodResponse<T>>().await.map_or_else(
            |e| Err(ChimoneyError::Deserialise(e.to_string())),
            |r| Ok(r.data),
        ),

        // This is better but Fumnanya is a retard - Dami ;(
        // StatusCode::OK => r
        //     .json::<GoodResponse<_>>()
        //     .await
        //     .map(|x| x.data)
        //     .map_err(|e| ChimoneyError::Deserialise(e.to_string())),
        status => {
            let error = r
                .json::<ErrorResponse>()
                .await
                .map_err(|e| ChimoneyError::Deserialise(e.to_string()))?;

            match status {
                StatusCode::BAD_REQUEST => Err(ChimoneyError::Generic(error.message)),

                StatusCode::UNAUTHORIZED => Err(ChimoneyError::Unauthenticated(error.message)),

                StatusCode::FORBIDDEN => Err(ChimoneyError::Forbidden(error.message)),

                _ => Err(ChimoneyError::Undocumented(error.message)),
            }
        }
    }
}
