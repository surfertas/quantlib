use super::CashFlow;
use crate::daycounters::DayCounter;
use crate::time::Date;

pub trait Coupon: CashFlow {
    fn rate(&self) -> f64;
    fn accrued_amount(&self, _date: Date) -> f64;
    fn accrual_period(&self) -> f64;
    fn accrual_days(&self) -> i64;
}

pub struct CouponFields {
    pub nominal: f64,
    pub day_counter: Box<dyn DayCounter>,
    pub payment_date: Date,
    pub accrual_start_date: Date,
    pub accrual_end_date: Date,
    pub reference_period_start: Date,
    pub reference_period_end: Date,
}