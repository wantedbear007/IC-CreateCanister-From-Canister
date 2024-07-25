dfx stop && dfx start --clean --background

dfx identity use default

dfx canister create ref_canister
dfx build ref_canister

dfx deploy ref_canister
dfx deploy canister_creater_backend