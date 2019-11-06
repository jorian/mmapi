# MMAPI

A Rusty layer that talks to the [AtomicDEX API](https://github.com/KomodoPlatform/atomicDEX-API).

### Supported calls
- [x]   buy
- [x]   cancel_all_orders
- [x]   cancel_order
- [x]   coins_needed_for_kickstart
- [x]   disable_coin
- [x]   electrum
- [ ]   enable
- [x]   get_enabled_coins
- [ ]   get_trade_fee
- [ ]   help
- [x]   my_balance
- [ ]   my_orders
- [ ]   my_recent_swaps
- [ ]   my_swap_status
- [ ]   my_tx_history
- [ ]   order_status
- [ ]   orderbook
- [ ]   recover_funds_of_swap
- [ ]   sell
- [ ]   send_raw_transaction
- [ ]   setprice
- [ ]   set_required_confirmations
- [ ]   stop
- [ ]   version
- [ ]   withdraw

### to do
- [ ]   ability to provide own electrum server (url:port)
- [ ]   error handling
    - [x]   have custom error
    - [ ]   ability to catch an error as returned by atomicdex-api
- [ ]   expose a cleaner response output
- [ ]   give better names to Error types
    - do not use Other for everything

### bugs
- [ ]   send2 doesn't properly parse error

### Response variations
The atomicdex api has a plethora of different API responses

#### Successful responses
A successful [cancel_order](https://developers.atomicdex.io/basic-docs/atomicdex/atomicdex-api.html#cancel-order)
```json
{
  "result": "success"
}
```

A successful [recovery of stuck funds](https://developers.atomicdex.io/basic-docs/atomicdex/atomicdex-api.html#recover-funds-of-swap) (omitted some fields for space sake)
```json
{
  "result": {
    "action": "SpentOtherPayment",
    "tx_hex":"0400008085202f890113591b1feb52878f8aea53b658cf9948ba89b0cb27ad0cf30b59b5d3ef6d8ef700000000d8483045022100eda93472c1f6aa18aacb085e456bc47b75ce88527ed01c279ee1a955e85691b702201adf552cfc85cecf588536d5b8257d4969044dde86897f2780e8c122e3a705e40120576fa34d308f39b7a704616656cc124232143565ca7cf1c8c60d95859af8f22d004c6b63042555555db1752102631dcf1d4b1b693aa8c2751afc68e4794b1e5996566cfc701a663f8b7bbbe640ac6782012088a9146e602d4affeb86e4ee208802901b8fd43be2e2a4882102031d4256c4bc9f99ac88bf3dba21773132281f65f9bf23a59928bce08961e2f3ac68ffffffff0198929800000000001976a91405aab5342166f8594baf17a7d9bef5d56744332788ac0238555d000000000000000000000000000000"
  }
}
```

A successful call to [needed coins for kickstart](https://developers.atomicdex.io/basic-docs/atomicdex/atomicdex-api.html#coins-needed-for-kick-start)
```json
{ "result": ["BTC", "KMD"] }
```

A successful [broadcast of a raw transaction](https://developers.atomicdex.io/basic-docs/atomicdex/atomicdex-api.html#send-raw-transaction)
```json
{
  "tx_hash": "0b024ea6997e16387c0931de9f203d534c6b2b8500e4bda2df51a36b52a3ef33"
}
```

A successful [version query](https://developers.atomicdex.io/basic-docs/atomicdex/atomicdex-api.html#version)
```json
{
  "result": "2.0.996_mm2_3bb412578_Linux"
}
```

A successful [withdraw](https://developers.atomicdex.io/basic-docs/atomicdex/atomicdex-api.html#withdraw) (omitted some fields for space sake)
```json
{
  "block_height": 0,
  "tx_hex": "0400008085202f890207a8e96978acfb8f0d002c3e4390142810dc6568b48f8cd6d8c71866ad8743c5010000006a47304402201960a7089f2d93480fff68ce0b7ca7bb7a32a52915753ac7ae780abd6162cb1d02202c9b11d442e5f72a532f44ceb10122898d486b1474a10eb981c60c5538b9c82d012102031d4256c4bc9f99ac88bf3dba21773132281f65f9bf23a59928bce08961e2f3ffffffff97f56bf3b0f815bb737b7867e71ddb8198bba3574bb75737ba9c389a4d08edc6000000006a473044022055199d80bd7e2d1b932e54f097c6a15fc4b148d21299dc50067c1da18045f0ed02201d26d85333df65e6daab40a07a0e8a671af9d9b9d92fdf7d7ef97bd868ca545a012102031d4256c4bc9f99ac88bf3dba21773132281f65f9bf23a59928bce08961e2f3ffffffff0200ca9a3b000000001976a91464ae8510aac9546d5e7704e31ce177451386455588acad2a0d02000000001976a91405aab5342166f8594baf17a7d9bef5d56744332788ac00000000000000000000000000000000000000"
}
```

A successful [electrum](https://developers.atomicdex.io/basic-docs/atomicdex/atomicdex-api.html#electrum) call (omitted some fields for space sake)
```json
{
  "coin": "HELLOWORLD",
  "required_confirmations":1,
  "result": "success"
}
```

#### Error responses
Errors come in 3 variations:
```json
{
  "error":"lp_coins:943] lp_coins:693] mm2 param is not set neither in coins config nor enable request, assuming that coin is not supported"
}
```

A [disable](https://developers.atomicdex.io/basic-docs/atomicdex/atomicdex-api.html#disable-coin) call has more information to an error
```json
{
  "error":"There're currently matching orders using RICK",
  "orders": {
    "matching": ["d88d0a0e-f8bd-40ab-8edd-fe20801ef349"],
    "cancelled": ["c88d0a0e-f8bd-40ab-8edd-fe20801ef349"]
  }
}
```
as well as

```json
{
  "error": "There're active swaps using RICK",
  "swaps": ["d88d0a0e-f8bd-40ab-8edd-fe20801ef349"]
}
```
and also just the error
```json
{
  "error": "No such coin: RICK"
}
```