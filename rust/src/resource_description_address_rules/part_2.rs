// Generated from complete js/src/resource-description-address-rules.js. Do not edit by hand.
use super::MlAcceptResourceAddressVersionSixJoin6;
use super::MlAcceptResourceAddressVersionSixLoop4Result;
use super::MlAcceptResourceAddressVersionSixLoop5Result;
use super::MlAcceptResourceAddressVersionSixLoop7Result;
use super::accept_resource_address_decimal_tail;
use super::accept_resource_address_hexadecimal;
use super::ml;
use super::ml_accept_resource_address_version_six_loop5;
use super::ml_accept_resource_address_version_six_loop7;
use super::ml_array;
use super::ml_math;
use super::read_resource_address_unit;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_resource_address_version_six_loop4(units: &[f64], mut end: f64, mut position: f64, mut groups: f64, mut compressed: bool) -> self::MlAcceptResourceAddressVersionSixLoop4Result {
    loop {
        return if position < end {
            {
                let beginning = position;
                {
                    let decimal = false;
                    {
                        let ml_s7 = self::ml_accept_resource_address_version_six_loop5(units, end, position, decimal);
                        match ml_s7 {
                            self::MlAcceptResourceAddressVersionSixLoop5Result::MlAcceptResourceAddressVersionSixLoop5Done(position_2, decimal_2) => {
                                {
                                    let ml_s9 = if decimal_2 {
                                        if (position_2 != end) || !(self::accept_resource_address_decimal_tail(units, beginning, end)) {
                                            self::MlAcceptResourceAddressVersionSixJoin6::MlAcceptResourceAddressVersionSixJoin6Return(false)
                                        } else {
                                            {
                                                let groups_2 = groups + 2f64;
                                                self::MlAcceptResourceAddressVersionSixJoin6::MlAcceptResourceAddressVersionSixJoin6Next(groups_2)
                                            }
                                        }
                                    } else if (position_2 == beginning) || ((position_2 - beginning) > 4f64) {
                                            self::MlAcceptResourceAddressVersionSixJoin6::MlAcceptResourceAddressVersionSixJoin6Return(false)
                                        } else {
                                            {
                                                let at = beginning;
                                                {
                                                    let ml_s10 = self::ml_accept_resource_address_version_six_loop7(units, position_2, at);
                                                    match ml_s10 {
                                                        self::MlAcceptResourceAddressVersionSixLoop7Result::MlAcceptResourceAddressVersionSixLoop7Done => {
                                                            {
                                                                let groups_3 = groups + 1f64;
                                                                self::MlAcceptResourceAddressVersionSixJoin6::MlAcceptResourceAddressVersionSixJoin6Next(groups_3)
                                                            }
                                                        }
                                                        self::MlAcceptResourceAddressVersionSixLoop7Result::MlAcceptResourceAddressVersionSixLoop7Return(ml_result7) => {
                                                            self::MlAcceptResourceAddressVersionSixJoin6::MlAcceptResourceAddressVersionSixJoin6Return(ml_result7)
                                                        }
                                                    }
                                                }
                                            }
                                        };
                                    match ml_s9 {
                                        self::MlAcceptResourceAddressVersionSixJoin6::MlAcceptResourceAddressVersionSixJoin6Next(groups_4) => {
                                            if groups_4 > 8f64 {
                                                self::MlAcceptResourceAddressVersionSixLoop4Result::MlAcceptResourceAddressVersionSixLoop4Return(false)
                                            } else if position_2 == end {
                                                    self::MlAcceptResourceAddressVersionSixLoop4Result::MlAcceptResourceAddressVersionSixLoop4Done(groups_4, compressed)
                                                } else if ((position_2 + 1f64) < end) && ((self::read_resource_address_unit(units, (position_2 + 1f64))) == 58f64) {
                                                        if compressed {
                                                            self::MlAcceptResourceAddressVersionSixLoop4Result::MlAcceptResourceAddressVersionSixLoop4Return(false)
                                                        } else {
                                                            {
                                                                let compressed_2 = true;
                                                                {
                                                                    let position_3 = position_2 + 2f64;
                                                                    {
                                                                        (position, groups, compressed) = (position_3, groups_4, compressed_2);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {
                                                            let position_4 = position_2 + 1f64;
                                                            if position_4 == end {
                                                                self::MlAcceptResourceAddressVersionSixLoop4Result::MlAcceptResourceAddressVersionSixLoop4Return(false)
                                                            } else {
                                                                {
                                                                    (position, groups) = (position_4, groups_4);
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                    }
                                        }
                                        self::MlAcceptResourceAddressVersionSixJoin6::MlAcceptResourceAddressVersionSixJoin6Return(ml_result6) => {
                                            self::MlAcceptResourceAddressVersionSixLoop4Result::MlAcceptResourceAddressVersionSixLoop4Return(ml_result6)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlAcceptResourceAddressVersionSixLoop4Result::MlAcceptResourceAddressVersionSixLoop4Done(groups, compressed)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlAcceptResourceAddressVersionSixJoin3 {
    MlAcceptResourceAddressVersionSixJoin3Next(f64, bool),
    MlAcceptResourceAddressVersionSixJoin3Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_resource_address_future_version_loop8(units: &[f64], mut end: f64, mut position: f64) -> f64 {
    loop {
        if (position < end) && self::accept_resource_address_hexadecimal(self::read_resource_address_unit(units, position)) {
            {
                let position_2 = position + 1f64;
                {
                    position = position_2;
                    continue;
                }
            }
        }
return {
            position
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptResourceAddressFutureVersionLoop9Result {
    MlAcceptResourceAddressFutureVersionLoop9Done,
    MlAcceptResourceAddressFutureVersionLoop9Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_resource_address_future_version_loop9(units: &[f64], mut end: f64, mut position: f64) -> self::MlAcceptResourceAddressFutureVersionLoop9Result {
    loop {
        return if position < end {
            {
                let value = self::read_resource_address_unit(units, position);
                if (((48f64..=57f64).contains(&value)) || ((65f64..=90f64).contains(&value))) || ((97f64..=122f64).contains(&value)) {
                    {
                        let position_2 = position + 1f64;
                        {
                            position = position_2;
                            continue;
                        }
                    }
                }
{
                    if (((((((((((((((value == 45f64) || (value == 46f64)) || (value == 95f64)) || (value == 126f64)) || (value == 58f64)) || (value == 33f64)) || (value == 36f64)) || (value == 38f64)) || (value == 39f64)) || (value == 40f64)) || (value == 41f64)) || (value == 42f64)) || (value == 43f64)) || (value == 44f64)) || (value == 59f64)) || (value == 61f64) {
                        {
                            let position_3 = position + 1f64;
                            {
                                position = position_3;
                                continue;
                            }
                        }
                    }
{
                        self::MlAcceptResourceAddressFutureVersionLoop9Result::MlAcceptResourceAddressFutureVersionLoop9Return(false)
                    }
                }
            }
        } else {
            self::MlAcceptResourceAddressFutureVersionLoop9Result::MlAcceptResourceAddressFutureVersionLoop9Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_project_resource_address_authority_loop12(units: &[f64], mut colon: f64) -> f64 {
    loop {
        if (colon < (units.len() as f64)) && ((self::read_resource_address_unit(units, colon)) != 58f64) {
            {
                let colon_2 = colon + 1f64;
                {
                    colon = colon_2;
                    continue;
                }
            }
        }
return {
            colon
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_project_resource_address_authority_loop13(units: &[f64], mut end: f64) -> f64 {
    loop {
        if (((end < (units.len() as f64)) && ((self::read_resource_address_unit(units, end)) != 47f64)) && ((self::read_resource_address_unit(units, end)) != 63f64)) && ((self::read_resource_address_unit(units, end)) != 35f64) {
            {
                let end_2 = end + 1f64;
                {
                    end = end_2;
                    continue;
                }
            }
        }
return {
            end
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_project_resource_address_authority_loop14(units: &[f64], mut end: f64, mut host: f64, mut at: f64) -> f64 {
    loop {
        if at < end {
            if (self::read_resource_address_unit(units, at)) == 64f64 {
                {
                    let host_2 = at + 1f64;
                    {
                        let at_2 = at + 1f64;
                        {
                            (host, at) = (host_2, at_2);
                            continue;
                        }
                    }
                }
            }
{
                {
                    let at_3 = at + 1f64;
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        }
return {
            host
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_project_resource_address_authority_loop15(units: &[f64], mut end: f64, mut close: f64) -> f64 {
    loop {
        if (close < end) && ((self::read_resource_address_unit(units, close)) != 93f64) {
            {
                let close_2 = close + 1f64;
                {
                    close = close_2;
                    continue;
                }
            }
        }
return {
            close
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlProjectResourceAddressAuthorityLoop11Result {
    MlProjectResourceAddressAuthorityLoop11Done,
    MlProjectResourceAddressAuthorityLoop11Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_project_resource_address_authority_loop11(ml_values10: &[f64], mut ml_index10: self::ml::Big) -> self::MlProjectResourceAddressAuthorityLoop11Result {
    loop {
        return if ml_index10 < (self::ml::Big::from_u128(ml_values10.len() as u128)) {
            {
                let value = self::ml_array::at(ml_values10, ml_index10.to_index());
                if (!(self::ml_math::is_safe_integer(value)) || (value < 0f64)) || (value > 65_535f64) {
                    self::MlProjectResourceAddressAuthorityLoop11Result::MlProjectResourceAddressAuthorityLoop11Return(Vec::<f64>::new())
                } else {
                    {
                        let ml_index10_2 = ml_index10.add(&self::ml::Big::from_i128(1));
                        {
                            ml_index10 = ml_index10_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlProjectResourceAddressAuthorityLoop11Result::MlProjectResourceAddressAuthorityLoop11Done
        };
    }
}
