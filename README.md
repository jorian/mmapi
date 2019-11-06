# MMAPI

A Rusty layer that talks to the [AtomicDEX API](https://github.com/KomodoPlatform/atomicDEX-API).

#### Supported calls
- [x]   buy
- [ ]   cancel_all_orders
- [ ]   cancel_order
- [ ]   coins_needed_for_kickstart
- [ ]   disable_coin
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

#### to do
- [ ]   ability to provide own electrum server (url:port)
- [ ]   error handling
    - [x]   have custom error
    - [ ]   ability to catch an error as returned by atomicdex-api
- [ ]   expose a cleaner response output