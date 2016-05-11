use call_args::CallArgs;
use model::{ApiList, CountrySpec};
use super::ApiCall;
use {Result, StripeClient};

#[derive(Debug)]
pub struct ListCountrySpecCall<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> ListCountrySpecCall<'a> {
    pub fn new(client: &'a StripeClient) -> ListCountrySpecCall<'a> {
        ListCountrySpecCall {
            client: client,
            args: CallArgs::new()
        }
    }

    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.args.add_arg("ending_before", ending_before);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.args.add_arg("limit", limit);
        self
    }

    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.args.add_arg("starting_after", starting_after);
        self
    }
}

impl<'a> ApiCall<ApiList<CountrySpec>> for ListCountrySpecCall<'a> {
    fn call(self) -> Result<ApiList<CountrySpec>> {
        self.client.get("country_specs", &self.args)
    }
}

#[derive(Debug)]
pub struct RetrieveCountrySpecCall<'a> {
    client: &'a StripeClient,
    iso_code: String
}

impl<'a> RetrieveCountrySpecCall<'a> {
    pub fn new(client: &'a StripeClient, iso_code: String) -> RetrieveCountrySpecCall<'a> {
        RetrieveCountrySpecCall {
            client: client,
            iso_code: iso_code
        }
    }
}

impl<'a> ApiCall<CountrySpec> for RetrieveCountrySpecCall<'a> {
    fn call(self) -> Result<CountrySpec> {
        self.client.get(format!("/country_specs/{}", self.iso_code), &())
    }
}
