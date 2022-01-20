
principal=$(dfx identity get-principal)
# cap_principal=$(cat .dfx/local/canister_ids.json | jq ".cap.local" -r)

# echo "principal: $principal"
# echo "cap_principal: $cap_principal"

# echo yes | dfx deploy cap --no-wallet --mode=reinstall

echo yes | dfx deploy nft --no-wallet --mode=reinstall  --argument "(principal \"$principal\", \"tkn\", \"token\", principal \"$cap_principal\")"