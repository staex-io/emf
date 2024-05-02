use contract_transcode::{ContractMessageTranscoder, Value};
use pallet_contracts_primitives::ContractExecResult;
use subxt::{
    backend::legacy::LegacyRpcMethods,
    config::{polkadot::H256, PolkadotExtrinsicParamsBuilder},
    error::{RpcError, TransactionError},
    ext::codec::Encode,
    tx::{Signer, TxPayload, TxStatus},
    utils::{AccountId32, MultiAddress},
    OnlineClient, PolkadotConfig,
};
use subxt_signer::sr25519::Keypair;

use crate::{
    emf_contract::api::{
        runtime_apis::contracts_api::types::Call,
        runtime_types::{
            contracts_node_runtime::RuntimeEvent, frame_system::EventRecord,
            sp_weights::weight_v2::Weight,
        },
        TransactionApi,
    },
    Error, Res,
};

pub(crate) async fn create_entity(
    api: &OnlineClient<PolkadotConfig>,
    rpc_legacy: &LegacyRpcMethods<PolkadotConfig>,
    keypair: &Keypair,
    contract_address: AccountId32,
) -> Res<()> {
    let message = "create_entity";
    let input_data_args: &[String] = &["".to_string(); 0];
    let dry_run_res =
        dry_run(rpc_legacy, contract_address.clone(), keypair, message, input_data_args).await?;

    let transcoder = init_transcoder()?;
    let data = transcoder.encode(message, input_data_args)?;
    let call = (TransactionApi {}).contracts().call(
        MultiAddress::Id(contract_address),
        0,
        dry_run_res.gas_required,
        None,
        data,
    );
    submit_tx(api, rpc_legacy, &call, keypair).await
}

pub(crate) async fn create_sub_entity(
    api: &OnlineClient<PolkadotConfig>,
    rpc_legacy: &LegacyRpcMethods<PolkadotConfig>,
    keypair: &Keypair,
    contract_address: AccountId32,
    sub_entity: AccountId32,
    location: String,
) -> Res<()> {
    let message = "create_sub_entity";
    let input_data_args: &[String] = &[format!("\"{sub_entity}\""), format!("\"{location}\"")];
    let dry_run_res =
        dry_run(rpc_legacy, contract_address.clone(), keypair, message, input_data_args).await?;

    let transcoder = init_transcoder()?;
    let data = transcoder.encode(message, input_data_args)?;
    let call = (TransactionApi {}).contracts().call(
        MultiAddress::Id(contract_address),
        0,
        dry_run_res.gas_required,
        None,
        data,
    );
    submit_tx(api, rpc_legacy, &call, keypair).await
}

#[cfg_attr(test, derive(Debug))]
struct DryRunResult {
    data: Value,
    gas_required: Weight,
}

impl DryRunResult {
    fn to_get_message_res(&self) -> Res<bool> {
        match &self.data {
            Value::Tuple(t) => {
                if t.values().count() != 1 {
                    return Err(format!("unexpected values count: {}", t.values().count()).into());
                }
                let value = t.values().last().ok_or::<&str>("last value is not found")?;
                match value {
                    Value::Bool(b) => Ok(*b),
                    _ => Err("unexpected response: value in tuple is not bool".into()),
                }
            }
            _ => Err("unexpected response: value is not tuple".into()),
        }
    }
}

fn init_transcoder() -> Res<ContractMessageTranscoder> {
    Ok(ContractMessageTranscoder::load("assets/emf_contract.metadata.json")?)
}

async fn submit_tx<Call: TxPayload, S: Signer<PolkadotConfig>>(
    api: &OnlineClient<PolkadotConfig>,
    rpc_legacy: &LegacyRpcMethods<PolkadotConfig>,
    call: &Call,
    signer: &S,
) -> Res<()> {
    let account_id = signer.account_id();
    let account_nonce = get_nonce(api, rpc_legacy, &account_id).await?;
    let params = PolkadotExtrinsicParamsBuilder::new().nonce(account_nonce).build();
    let mut tx = api.tx().create_signed(call, signer, params).await?.submit_and_watch().await?;
    while let Some(status) = tx.next().await {
        match status? {
            TxStatus::InBestBlock(_) | TxStatus::InFinalizedBlock(_) => {
                return Ok(());
            }
            TxStatus::Error { message } => return Err(TransactionError::Error(message).into()),
            TxStatus::Invalid { message } => return Err(TransactionError::Invalid(message).into()),
            TxStatus::Dropped { message } => return Err(TransactionError::Dropped(message).into()),
            _ => continue,
        }
    }
    Err(RpcError::SubscriptionDropped.into())
}

async fn dry_run(
    rpc_legacy: &LegacyRpcMethods<PolkadotConfig>,
    contract_addr: AccountId32,
    keypair: &Keypair,
    message: &str,
    input_data_args: &[String],
) -> Res<DryRunResult> {
    let transcoder = init_transcoder()?;
    let input_data = transcoder.encode(message, input_data_args)?;
    let args = Call {
        origin: keypair.public_key().to_account_id(),
        dest: contract_addr,
        gas_limit: None,
        storage_deposit_limit: None,
        value: 0,
        input_data,
    }
    .encode();
    let bytes = rpc_legacy.state_call("ContractsApi_call", Some(&args), None).await?;
    let exec_res: ContractExecResult<u128, EventRecord<RuntimeEvent, H256>> =
        scale::decode_from_bytes(bytes.clone().into())?;
    let exec_res_data = exec_res.result.map_err(|_| "failed to parse exec result".to_string())?;
    if exec_res_data.did_revert() {
        let data = transcoder.decode_message_return(message, &mut exec_res_data.data.as_slice())?;
        return Err(parse_revert(data));
    }
    let data = transcoder.decode_message_return(message, &mut exec_res_data.data.as_ref())?;
    Ok(DryRunResult {
        data,
        gas_required: Weight {
            ref_time: exec_res.gas_required.ref_time(),
            proof_size: exec_res.gas_required.proof_size(),
        },
    })
}

async fn get_nonce(
    api: &OnlineClient<PolkadotConfig>,
    rpc_legacy: &LegacyRpcMethods<PolkadotConfig>,
    account_id: &AccountId32,
) -> Res<u64> {
    let best_block = rpc_legacy
        .chain_get_block_hash(None)
        .await?
        .ok_or(subxt::Error::Other("best block not found".into()))?;
    let account_nonce = api.blocks().at(best_block).await?.account_nonce(account_id).await?;
    Ok(account_nonce)
}

fn parse_revert(value: Value) -> Error {
    if let Value::Tuple(value) = value {
        if let Some(Value::Tuple(value)) = value.values().next() {
            if let Some(Value::Tuple(value)) = value.values().next() {
                if let Some(ident) = value.ident() {
                    return format!("revert error: {ident}").into();
                }
            }
        }
    }
    "unknown response to parse revert state".into()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use subxt::backend::rpc;

    use super::*;

    #[tokio::test]
    #[ignore = "require substrate contracts node manual starting"]
    async fn test_entities_creation() {
        let rpc_url = "ws://127.0.0.1:9944";
        let api = OnlineClient::<PolkadotConfig>::from_url(rpc_url).await.unwrap();
        let rpc = rpc::RpcClient::from_url(rpc_url).await.unwrap();
        let rpc_legacy: LegacyRpcMethods<PolkadotConfig> = LegacyRpcMethods::new(rpc.clone());

        let contract_address: AccountId32 =
            AccountId32::from_str("5Da8RgQpGdFFVHuT7CcMiREL4AAikfXfH4hytTFywMX5LqEP").unwrap();
        let entity_keypair = subxt_signer::sr25519::dev::alice();
        let sub_entity_keypair = subxt_signer::sr25519::dev::bob();

        create_entity(&api, &rpc_legacy, &entity_keypair, contract_address.clone()).await.unwrap();
        create_sub_entity(
            &api,
            &rpc_legacy,
            &entity_keypair,
            contract_address,
            sub_entity_keypair.public_key().to_account_id(),
            "Berlin".to_string(),
        )
        .await
        .unwrap();
    }
}
