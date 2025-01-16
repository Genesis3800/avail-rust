#!/bin/bash

# List of sub-members
members=("dataAvailability_createApplicationKey" "dataAvailability_submitData" "dataAvailability_appKeys" "dataAvailability_nextAppId" "balances_transferKeepAlive" "balances_transferAllowDeath" "balances_transferAll" "system_account" "staking_bond" "staking_nominate" "staking_unbond" "staking_activeEra")

# Initialize each sub-member
for member in "${members[@]}"; do
    cargo new --bin $member
done

# Add members to the root `Cargo.toml`
echo -e "\n[workspace]\nmembers = [" >> Cargo.toml
for member in "${members[@]}"; do
    echo "    \"$member\"," >> Cargo.toml
done
echo "]" >> Cargo.toml