use call_args::CallArgs;
use model::{ApiList, Coupon, CouponDuration, Currency, Delete};
use std::collections::BTreeMap;
use super::ApiCall;
use {Result, StripeClient, TimeConstraint};

#[derive(Debug)]
pub struct CreateCouponRequest<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> CreateCouponRequest<'a> {
    pub fn new(client: &'a StripeClient, duration: CouponDuration) -> CreateCouponRequest<'a> {
        let mut args = CallArgs::new();
        args.add_arg("duration", duration);
        CreateCouponRequest {
            client: client,
            args: args
        }
    }

    pub fn id(mut self, id: String) -> Self {
        self.args.add_arg("id", id);
        self
    }

    pub fn amount_off(mut self, amount_off: i64) -> Self {
        self.args.add_arg("amount_off", amount_off);
        self
    }

    pub fn currency(mut self, currency: Currency) -> Self {
        self.args.add_arg("currency", currency);
        self
    }

    pub fn duration_in_month(mut self, duration_in_month: i64) -> Self {
        self.args.add_arg("duration_in_month", duration_in_month);
        self
    }

    pub fn max_redemptions(mut self, max_redemptions: i64) -> Self {
        self.args.add_arg("max_redemptions", max_redemptions);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }

    pub fn percent_off(mut self, percent_off: i64) -> Self {
        self.args.add_arg("percent_off", percent_off);
        self
    }

    pub fn redeem_by(mut self, redeem_by: i64) -> Self {
        self.args.add_arg("redeem_by", redeem_by);
        self
    }
}

impl<'a> ApiCall<Coupon> for CreateCouponRequest<'a> {
    fn call(self) -> Result<Coupon> {
        self.client.post("/coupons", &self.args)
    }
}

#[derive(Debug)]
pub struct RetrieveCouponRequest<'a> {
    client: &'a StripeClient,
    coupon_id: String
}

impl<'a> RetrieveCouponRequest<'a> {
    pub fn new(client: &'a StripeClient, coupon_id: String) -> RetrieveCouponRequest<'a> {
        RetrieveCouponRequest {
            client: client,
            coupon_id: coupon_id
        }
    }
}

impl<'a> ApiCall<Coupon> for RetrieveCouponRequest<'a> {
    fn call(self) -> Result<Coupon> {
        self.client.get(format!("/coupons/{}", self.coupon_id), &())
    }
}

#[derive(Debug)]
pub struct UpdateCouponRequest<'a> {
    client: &'a StripeClient,
    coupon_id: String,
    args: CallArgs
}

impl<'a> UpdateCouponRequest<'a> {
    pub fn new(client: &'a StripeClient, coupon_id: String) -> UpdateCouponRequest<'a> {
        UpdateCouponRequest {
            client: client,
            coupon_id: coupon_id,
            args: CallArgs::new()
        }
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }
}

impl<'a> ApiCall<Coupon> for UpdateCouponRequest<'a> {
    fn call(self) -> Result<Coupon> {
        self.client.post(format!("/coupons/{}", self.coupon_id), &self.args)
    }
}

#[derive(Debug)]
pub struct DeleteCouponRequest<'a> {
    client: &'a StripeClient,
    coupon_id: String
}

impl<'a> DeleteCouponRequest<'a> {
    pub fn new(client: &'a StripeClient, coupon_id: String) -> DeleteCouponRequest<'a> {
        DeleteCouponRequest {
            client: client,
            coupon_id: coupon_id
        }
    }
}

impl<'a> ApiCall<Delete> for DeleteCouponRequest<'a> {
    fn call(self) -> Result<Delete> {
        self.client.delete(format!("/coupons/{}", self.coupon_id))
    }
}

#[derive(Debug)]
pub struct ListCouponsRequest<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> ListCouponsRequest<'a> {
    pub fn new(client: &'a StripeClient) -> ListCouponsRequest<'a> {
        ListCouponsRequest {
            client: client,
            args: CallArgs::from(("include[]", "total_count"))
        }
    }

    pub fn created(mut self, created: TimeConstraint) -> Self {
        self.args.add_object("created", created);
        self
    }

    pub fn created_exact(mut self, created: i64) -> Self {
        self.args.add_arg("created", created);
        self
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

impl<'a> ApiCall<ApiList<Coupon>> for ListCouponsRequest<'a> {
    fn call(self) -> Result<ApiList<Coupon>> {
        self.client.get("/coupons", &self.args)
    }
}
