// This file is generated by atrium-codegen. Do not edit.
//! Definitions for the `com.atproto.sync.getRepo` namespace.

/// Gets the repo state.
#[async_trait::async_trait]
pub trait GetRepo: crate::xrpc::XrpcClient {
    async fn get_repo(&self, params: Parameters) -> Result<(), Box<dyn std::error::Error>> {
        let body = crate::xrpc::XrpcClient::send(
            self,
            http::Method::GET,
            "com.atproto.sync.getRepo",
            Some(serde_urlencoded::to_string(&params)?),
            None,
            None,
        )
        .await?;
        serde_json::from_slice(&body).map_err(|e| e.into())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    /// The DID of the repo.
    pub did: String,
    /// The earliest commit in the commit range (not inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earliest: Option<String>,
    /// The latest commit in the commit range (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest: Option<String>,
}


pub enum Error {
}