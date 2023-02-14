use crate::common::MY_CALLER_ID;
use anyhow::Result;
use candid::{CandidType, Deserialize, Encode, Principal};
use iccrypt_backend::smart_vaults::user_safe::UserSafe;
use pretty_assertions::{assert_eq, assert_ne};

use crate::common::{get_dfx_agent, get_iccrypt_backend_canister};

#[derive(CandidType)]
struct CreateUserArg {
    user_id: Principal,
}

#[derive(CandidType, Deserialize)]
struct ExampleArgSet {
    canister_id: Principal,
    controllers: Option<Vec<Principal>>,
    amount: Option<candid::Nat>,
}

pub async fn test_smart_vaults() -> Result<()> {
    dbg!("Testing smart vaults");
    test_user_lifecycle().await?;
    Ok(())
}

async fn test_user_lifecycle() -> anyhow::Result<()> {
    let user = Principal::from_text(MY_CALLER_ID).expect("Could not decode the principal.");

    // create a new user.
    let agent = get_dfx_agent().unwrap();
    agent.fetch_root_key().await?;
    let canister = get_iccrypt_backend_canister();
    let _res: Vec<u8> = agent
        .update(&canister, "create_new_user")
        //.with_arg(&Encode!(&user)?)
        .with_arg(&Encode!(&user)?)
        .call_and_wait()
        .await
        .unwrap();

    // get user safe of newly created user
    let res: Vec<u8> = agent
        .query(&canister, "get_user_safe")
        .with_arg(&Encode!(&user)?)
        //.with_arg(&Encode!(&canister)?)
        .call()
        .await?;
    let mut res_deserialized = candid::de::IDLDeserialize::new(&res)?;

    if let Some(user_safe) = res_deserialized.get_value::<Option<UserSafe>>().unwrap() {
        // check we have the right owner
        assert_eq!(&user_safe.owner().to_string(), MY_CALLER_ID);

        // check there are no secrets yet
        assert!(&user_safe.secrets().is_empty());
    } else {
        return Err(anyhow::format_err!("User Safe not found"));
    }

    Ok(())
}
