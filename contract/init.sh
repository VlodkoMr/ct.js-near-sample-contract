#!/bin/sh

NEAR_ID=vlodkow.testnet
CONTRACT_ID=$(<neardev/dev-account)

echo "--- Init Contract:"
near call $CONTRACT_ID new_default_meta '{"owner_id":"'$NEAR_ID'"}' --accountId $NEAR_ID

near call $CONTRACT_ID add_ship_series '{"title": "Spectrum", "media": "11.png", "max_supply": 1000}' --accountId $NEAR_ID
near call $CONTRACT_ID add_ship_series '{"title": "Polaris", "media": "21.png", "max_supply": 1000}' --accountId $NEAR_ID
near call $CONTRACT_ID add_ship_series '{"title": "Valkyrie", "media": "31.png", "max_supply": 1000}' --accountId $NEAR_ID
near call $CONTRACT_ID add_ship_series '{"title": "Sparrow", "media": "41.png", "max_supply": 1000}' --accountId $NEAR_ID
near call $CONTRACT_ID add_ship_series '{"title": "Alto", "media": "51.png", "max_supply": 1000}' --accountId $NEAR_ID

#near call $CONTRACT_ID nft_mint '{"series_id": 1}' --accountId $NEAR_ID --deposit 0.1
