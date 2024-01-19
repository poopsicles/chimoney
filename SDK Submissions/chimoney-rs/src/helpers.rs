use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;

use crate::{
    client::{ChimoneyError, ChimoneyResult},
    schema::responses::ResponseEnum,
};

/// This function transforms a response into a type <br>
/// We use a response enum (`ResponseEnum`) due to bad implementation
/// of the API because some errors sometimes return 200 with a `message` field.
/// E.g `<API>/payments/initiate` so we try to
/// encapsulate all of the possible response structures within the enum. Thanks Chimoney
pub async fn transform_response<T: DeserializeOwned>(r: Response) -> ChimoneyResult<T> {
    let status = r.status();
    match r.json::<ResponseEnum<T>>().await {
        Ok(response) => match response {
            ResponseEnum::Good(response) => Ok(response.data),
            ResponseEnum::Error(response) => match status {
                StatusCode::BAD_REQUEST => Err(ChimoneyError::Generic(response.error)),
                StatusCode::UNAUTHORIZED => Err(ChimoneyError::Unauthenticated(response.error)),
                StatusCode::FORBIDDEN => Err(ChimoneyError::Forbidden(response.error)),
                _ => Err(ChimoneyError::Undocumented(response.error)),
            },
            ResponseEnum::Message(response) => Err(ChimoneyError::Generic(response.message)),
        },
        Err(e) => Err(ChimoneyError::Deserialise(e.to_string())),
    }
}
