use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage,
};

use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, State};
/*
    Secret Number
    - Init
        - secret_number: Secret Number that has to be found
        - owner: Creator
    - GuessNumber
        - guess_number: Number that is compared to the secretNumber and returns
            - secret_number == guess_number - Congratulations, the secret number is ${guess_number}
            - secret_number > guess_number - The secret number is higher than ${guess_number}
            - secret_number < guess_number - The secret number is lower than ${guess_number}
    - ModifySecretNumber
        - new_secret_number: New Secret Number
*/

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        secret_number: msg.secret_number,
        owner: env.message.sender,
    };

    config(&mut deps.storage).save(&state)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::ModifySecretNumber { new_secret_number } => modify_number_function(deps, env, new_secret_number),
    }
}

pub fn modify_number_function<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    new_secret_number: i32,
) -> StdResult<HandleResponse> {
    config(&mut deps.storage).update(|mut state| {
        if env.message.sender != state.owner {
            return Err(StdError::Unauthorized { backtrace: None });
        }
        state.secret_number = new_secret_number;
        Ok(state)
    })?;
    Ok(HandleResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GuessNumber { guess_number } => to_binary(&guess_number_function(deps, guess_number)?),
    }
}

fn guess_number_function <S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>, guess_number: i32) -> StdResult<String> {
    let state = config_read(&deps.storage).load()?;

    if state.secret_number == guess_number {
        Ok(format!("Congratulations, the secret number is {}", guess_number))
    } else if state.secret_number > guess_number {
        Ok(format!("The secret number is higher than {}", guess_number))
    } else {
        Ok(format!("The secret number is lower than {}", guess_number))
    }
}