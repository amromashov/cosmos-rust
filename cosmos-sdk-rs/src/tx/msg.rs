//! Transaction messages

use crate::{prost_ext::MessageExt, proto, Error, Result};
use prost_types::Any;

/// Transaction messages
#[derive(Clone, Debug, PartialEq)]
pub struct Msg(pub(crate) Any);

impl Msg {
    /// Create a new message type
    pub fn new(type_url: impl Into<String>, value: impl Into<Vec<u8>>) -> Self {
        Msg(Any {
            type_url: type_url.into(),
            value: value.into(),
        })
    }
}

impl Eq for Msg {}

impl From<Any> for Msg {
    fn from(any: Any) -> Msg {
        Msg(any)
    }
}

impl From<Msg> for Any {
    fn from(msg: Msg) -> Any {
        msg.0
    }
}

/// Message types that can be converted to/from a [`Msg`].
pub trait MsgType {
    /// Attempt to parse this value from a [`Msg`].
    fn from_msg(msg: &Msg) -> Result<Self>
    where
        Self: Sized;

    /// Serialize this value as a [`Msg`].
    fn to_msg(&self) -> Result<Msg>;
}

/// Proto types which can be used as a [`Msg`].
pub trait MsgProto: Default + MessageExt {
    /// Type URL value
    const TYPE_URL: &'static str;
}

impl<T> MsgType for T
where
    T: MsgProto,
{
    fn from_msg(msg: &Msg) -> Result<Self>
    where
        Self: Sized,
    {
        if msg.0.type_url == Self::TYPE_URL {
            Ok(Self::decode(&*msg.0.value)?)
        } else {
            Err(Error::MsgType {
                expected: Self::TYPE_URL,
                found: msg.0.type_url.clone(),
            }
            .into())
        }
    }

    fn to_msg(&self) -> Result<Msg> {
        self.to_bytes().map(|bytes| Msg::new(Self::TYPE_URL, bytes))
    }
}

impl MsgProto for proto::cosmos::bank::v1beta1::MsgSend {
    const TYPE_URL: &'static str = "/cosmos.bank.v1beta1.MsgSend";
}

impl MsgProto for proto::cosmos::distribution::v1beta1::MsgSetWithdrawAddress {
    const TYPE_URL: &'static str = "/cosmos.distribution.v1beta1.MsgSetWithdrawAddress";
}

impl MsgProto for proto::cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward {
    const TYPE_URL: &'static str = "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward";
}

impl MsgProto for proto::cosmos::distribution::v1beta1::MsgWithdrawValidatorCommission {
    const TYPE_URL: &'static str = "/cosmos.distribution.v1beta1.MsgWithdrawValidatorCommission";
}

impl MsgProto for proto::cosmos::distribution::v1beta1::MsgFundCommunityPool {
    const TYPE_URL: &'static str = "/cosmos.distribution.v1beta1.MsgFundCommunityPool";
}

impl MsgProto for proto::cosmos::staking::v1beta1::MsgDelegate {
    const TYPE_URL: &'static str = "/cosmos.staking.v1beta1.MsgDelegate";
}

impl MsgProto for proto::cosmos::staking::v1beta1::MsgUndelegate {
    const TYPE_URL: &'static str = "/cosmos.staking.v1beta1.MsgUndelegate";
}

impl MsgProto for proto::cosmos::staking::v1beta1::MsgBeginRedelegate {
    const TYPE_URL: &'static str = "/cosmos.staking.v1beta1.MsgBeginRedelegate";
}

#[cfg(feature = "cosmwasm")]
impl MsgProto for proto::cosmwasm::wasm::v1beta1::MsgStoreCode {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1beta1.MsgStoreCode";
}

#[cfg(feature = "cosmwasm")]
impl MsgProto for proto::cosmwasm::wasm::v1beta1::MsgInstantiateContract {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1beta1.MsgInstantiateContract";
}

#[cfg(feature = "cosmwasm")]
impl MsgProto for proto::cosmwasm::wasm::v1beta1::MsgExecuteContract {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1beta1.MsgExecuteContract";
}

#[cfg(feature = "cosmwasm")]
impl MsgProto for proto::cosmwasm::wasm::v1beta1::MsgMigrateContract {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1beta1.MsgMigrateContract";
}

#[cfg(feature = "cosmwasm")]
impl MsgProto for proto::cosmwasm::wasm::v1beta1::MsgUpdateAdmin {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1beta1.MsgUpdateAdmin";
}

#[cfg(feature = "cosmwasm")]
impl MsgProto for proto::cosmwasm::wasm::v1beta1::MsgClearAdmin {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1beta1.MsgClearAdmin";
}
