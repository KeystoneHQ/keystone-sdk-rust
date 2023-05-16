use alloc::format;
use alloc::string::{String, ToString};

#[derive(Clone, Debug, Default)]
struct ScriptExpressionValue<'a> {
    tag_value: u32,
    expression: &'a str,
}

impl ScriptExpressionValue<'_> {
    pub(crate) fn get_tag_value(&self) -> u32 {
        self.tag_value
    }

    pub(crate) fn get_expression(&self) -> String {
        self.expression.to_string()
    }
}

const SCRIPT_HASH: ScriptExpressionValue = ScriptExpressionValue {
    tag_value: 400,
    expression: "sh",
};
const WITNESS_SCRIPT_HASH: ScriptExpressionValue = ScriptExpressionValue {
    tag_value: 401,
    expression: "wsh",
};
const PUBLIC_KEY: ScriptExpressionValue = ScriptExpressionValue {
    tag_value: 402,
    expression: "pk",
};
const PUBLIC_KEY_HASH: ScriptExpressionValue = ScriptExpressionValue {
    tag_value: 403,
    expression: "pkh",
};
const WITNESS_PUBLIC_KEY_HASH: ScriptExpressionValue = ScriptExpressionValue {
    tag_value: 404,
    expression: "wpkh",
};
const COMBO: ScriptExpressionValue = ScriptExpressionValue {
    tag_value: 405,
    expression: "combo",
};
const MULTI_SIG: ScriptExpressionValue = ScriptExpressionValue {
    tag_value: 406,
    expression: "multi",
};
const SORTED_MULTI_SIG: ScriptExpressionValue = ScriptExpressionValue {
    tag_value: 407,
    expression: "sorted",
};
const ADDRESS: ScriptExpressionValue = ScriptExpressionValue {
    tag_value: 307,
    expression: "addr",
};
const RAW_SCRIPT: ScriptExpressionValue = ScriptExpressionValue {
    tag_value: 408,
    expression: "raw",
};

#[derive(Clone, Debug, PartialEq)]
pub enum ScriptExpression {
    ScriptHash,
    WitnessScriptHash,
    PublicKey,
    PublicKeyHash,
    WitnessPublicKeyHash,
    COMBO,
    MultiSig,
    SortedMultiSig,
    Address,
    RawScript,
    Undefine(u64),
}

impl ScriptExpression {
    pub fn from(tag_value: u64) -> Self {
        match tag_value {
            400 => ScriptExpression::ScriptHash,
            401 => ScriptExpression::WitnessScriptHash,
            402 => ScriptExpression::PublicKey,
            403 => ScriptExpression::PublicKeyHash,
            404 => ScriptExpression::WitnessPublicKeyHash,
            405 => ScriptExpression::COMBO,
            406 => ScriptExpression::MultiSig,
            407 => ScriptExpression::SortedMultiSig,
            307 => ScriptExpression::Address,
            408 => ScriptExpression::RawScript,
            _ => ScriptExpression::Undefine(tag_value),
        }
    }

    pub fn get_tag_value(&self) -> u32 {
        match self {
            ScriptExpression::ScriptHash => SCRIPT_HASH.get_tag_value(),
            ScriptExpression::WitnessScriptHash => WITNESS_SCRIPT_HASH.get_tag_value(),
            ScriptExpression::PublicKey => PUBLIC_KEY.get_tag_value(),
            ScriptExpression::PublicKeyHash => PUBLIC_KEY_HASH.get_tag_value(),
            ScriptExpression::WitnessPublicKeyHash => WITNESS_PUBLIC_KEY_HASH.get_tag_value(),
            ScriptExpression::COMBO => COMBO.get_tag_value(),
            ScriptExpression::MultiSig => MULTI_SIG.get_tag_value(),
            ScriptExpression::SortedMultiSig => SORTED_MULTI_SIG.get_tag_value(),
            ScriptExpression::Address => ADDRESS.get_tag_value(),
            ScriptExpression::RawScript => RAW_SCRIPT.get_tag_value(),
            ScriptExpression::Undefine(tag_value) => *tag_value as u32,
        }
    }

    pub fn get_expression(self) -> String {
        match self {
            ScriptExpression::ScriptHash => SCRIPT_HASH.get_expression(),
            ScriptExpression::WitnessScriptHash => WITNESS_SCRIPT_HASH.get_expression(),
            ScriptExpression::PublicKey => PUBLIC_KEY.get_expression(),
            ScriptExpression::PublicKeyHash => PUBLIC_KEY_HASH.get_expression(),
            ScriptExpression::WitnessPublicKeyHash => WITNESS_PUBLIC_KEY_HASH.get_expression(),
            ScriptExpression::COMBO => COMBO.get_expression(),
            ScriptExpression::MultiSig => MULTI_SIG.get_expression(),
            ScriptExpression::SortedMultiSig => SORTED_MULTI_SIG.get_expression(),
            ScriptExpression::Address => ADDRESS.get_expression(),
            ScriptExpression::RawScript => RAW_SCRIPT.get_expression(),
            ScriptExpression::Undefine(tag_value) => {
                format!("tag value is {}, undefine expression", tag_value)
            }
        }
    }
}
