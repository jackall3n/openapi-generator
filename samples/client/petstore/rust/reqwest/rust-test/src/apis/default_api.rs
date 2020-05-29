/*
 * Rust client test spec
 *
 * Special testing for the Rust client generator
 *
 * The version of the OpenAPI document: 1.0.7
 * 
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused_imports)]
use std::rc::Rc;
use std::borrow::Borrow;
use std::option::Option;

use reqwest;

use super::{Error, configuration};

pub struct DefaultApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl DefaultApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> DefaultApiClient {
        DefaultApiClient {
            configuration,
        }
    }
}


pub trait DefaultApi {
    fn dummy_get(&self, ) -> Result<(), Error>;
}

impl DefaultApi for DefaultApiClient {
    fn dummy_get(&self, ) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/dummy", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let mut resp = client.execute(req)?;
        if resp.status().is_success() {
            Ok(())
        } else {
            let status = resp.status();
            let content = resp.text()?;
            let entity: Option<serde_json::Value> = serde_json::from_str(&content).ok();
            let error = crate::apis::ResponseErrorContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

}
