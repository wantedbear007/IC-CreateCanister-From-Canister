use candid::{encode_one, CandidType, Nat, Principal};
use ic_cdk::{
    api::{
        call::{call_with_payment128, CallResult},
        canister_version,
        management_canister::main::{CanisterInstallMode, WasmModule},
    },
    println, update,
};
use serde::{Deserialize, Serialize};

// use crate::api::call::{call_with_payment128, CallResult};

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default,
)]
pub struct CanisterSettings {
    pub controllers: Option<Vec<Principal>>,

    pub compute_allocation: Option<Nat>,

    pub memory_allocation: Option<Nat>,

    pub freezing_threshold: Option<Nat>,

    pub reserved_cycles_limit: Option<Nat>,
}

#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone,
)]
pub(crate) struct InstallCodeArgumentExtended {
    /// See [CanisterInstallMode].
    pub mode: CanisterInstallMode,
    /// Principal of the canister.
    pub canister_id: CanisterId,
    /// Code to be installed.
    pub wasm_module: WasmModule,
    /// The argument to be passed to `canister_init` or `canister_post_upgrade`.
    pub arg: Vec<u8>,
    /// sender_canister_version must be set to ic_cdk::api::canister_version()
    pub sender_canister_version: Option<u64>,
}

#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default,
)]
pub struct CreateCanisterArgument {
    /// See [CanisterSettings].
    pub settings: Option<CanisterSettings>,
}

#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone,
)]
pub struct InstallCodeArgument {
    /// See [CanisterInstallMode].
    pub mode: CanisterInstallMode,
    /// Principal of the canister.
    pub canister_id: CanisterId,
    /// Code to be installed.
    pub wasm_module: WasmModule,
    /// The argument to be passed to `canister_init` or `canister_post_upgrade`.
    pub arg: Vec<u8>,
}

pub type CanisterId = Principal;

#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy,
)]
pub struct CanisterIdRecord {
    /// Principal of the canister.
    pub canister_id: CanisterId,
}

#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default,
)]
pub(crate) struct CreateCanisterArgumentExtended {
    /// See [CanisterSettings].
    pub settings: Option<CanisterSettings>,
    /// sender_canister_version must be set to ic_cdk::api::canister_version()
    pub sender_canister_version: Option<u64>,
}

// create canister
async fn create_canister(
    arg: CreateCanisterArgument, // cycles: u128,
) -> CallResult<(CanisterIdRecord,)> {
    let extended_arg = CreateCanisterArgumentExtended {
        settings: arg.settings,
        sender_canister_version: Some(canister_version()),
    };
    let cycles: u128 = 100_000_000_000;
    call_with_payment128(
        Principal::management_canister(),
        "create_canister",
        (extended_arg,),
        cycles,
    )
    .await
}

async fn deposit_cycles(arg: CanisterIdRecord, cycles: u128) -> CallResult<()> {
    call_with_payment128(
        Principal::management_canister(),
        "deposit_cycles",
        (arg,),
        cycles,
    )
    .await
}

async fn install_code(arg: InstallCodeArgument) -> CallResult<()> {
    // let wasm_base64: &str = "3831fb07143cd43c3c51f770342d2b7d0a594311529f5503587bf1544ccd44be";
    // let wasm_module_sample: Vec<u8> = base64::decode(wasm_base64).expect("Decoding failed");

    // let wasm_module_sample: Vec<u8> =
    // //     include_bytes!("../../../../.dfx/local/canisters/ref_canister/ref_canister.wasm").to_vec();

    let wasm_module_sample: Vec<u8> =
    include_bytes!("/home/wantedbear/Desktop/canister/canister_creater/.dfx/local/canisters/ref_canister/ref_canister.wasm").to_vec();



    let cycles: u128 = 10_000_000_000;

    let extended_arg = InstallCodeArgumentExtended {
        mode: arg.mode,
        canister_id: arg.canister_id,
        wasm_module: wasm_module_sample,
        arg: arg.arg,
        sender_canister_version: Some(canister_version()),
    };

    call_with_payment128(
        Principal::management_canister(),
        "install_code",
        (extended_arg,),
        cycles,
    )
    .await
}

#[update]
pub async fn create() -> Result<(), String> {
    let data: String = String::from("testing");

    // let dao_detail_bytes: Vec<u8> = match encode_one(&data) {
    //     Ok(bytes) => bytes,
    //     Err(e) => {
    //         return Err(format!("Failed to serialize DaoInput: {}", e));
    //     }
    // };

    let arg = CreateCanisterArgument { settings: None };
    let (canister_id,) = match create_canister(arg).await {
        Ok(id) => id,
        Err((_, err_string)) => {
            ic_cdk::println!("error in canister id");
            return Err(err_string);
        }
    };

    let _addcycles = deposit_cycles(canister_id, 100000000).await;
    let canister_id_principal = canister_id.canister_id;

    let arg1 = InstallCodeArgument {
        mode: CanisterInstallMode::Install,
        canister_id: canister_id_principal,
        wasm_module: vec![],
        arg: Vec::new(),
    };
    let _installcode = install_code(arg1).await;
    ic_cdk::println!("Canister ID: {:?}", canister_id_principal.to_string());

    Ok(())
}
