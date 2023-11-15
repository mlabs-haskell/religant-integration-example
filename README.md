## Overview 

This repository contains an example Radix application which integrates the Religant Radix Oracle. The instructions here are intended for Radix developers who wish to integrate the Religant Oracle into their own applications.

## Overview of the Protocol 

The Religant Oracle protocol enables a _decentralized_ feed of price data. A set of data nodes query exchanges for up-to-date transactions and submit feed updates to the contract, which employs a _consensus mechanism_ to determine the aggregate price. Data nodes running the Religant node backend software submit feed updates either on a regular interval (if the price is stable), or as soon as the new aggregate price calculated by a node exceeds a specific divergence threshold. 

At this time, the divergence threshold is set at 2%. This guarantees that the rate retrieved from the Religant oracle component's feed is always within 2% of the rates being used in real transactions on real exchanges. 

## Religant Component API 

The user-facing API of the Religant Oracle consists in a single method: 

``` rust
pub fn get_price(&self) -> Option<PriceData> 
```

which takes no arguments returns a data structure: 

``` rust
pub struct PriceData {
    pub price: Decimal,
    pub timestamp: i64,
}
```

Where the `price` field is the current XRD/USD exchange rate, and the `timestamp` field is a POSIX timestamp that that identifies the moment at which the the exchange rate was calculated. 

Note that the `Option` wrapper exists primarily for type safety. The `get_price` method will __always__ return `Some(_)` after the first round of Oracle feed aggregation. Consequently, it should generally be safe to assume that you will never receive a `None` as a result of calling the method on an active Religant oracle component. 

(You may, however, wish to check the timestamp. While we aim to provide high reliability, a major internet or cloud hosting outage could cause delays in feed updates, though this would be an extraordinary situation.)

## Integration Guide 

NOTE: These instructions are current as of 11/13/2023. Some of the official documentation may refer to a different method (using different macros), but as of today the method illustrated below is the only one known to work. 

Integrating with the Religant Oracle component requires a few small changes to your application's blueprint(s): 

__FIRST__: You must __define the `PriceData` struct__ *outside of your blueprint.* E.g., by adding: 
``` rust 
#[derive(ScryptoSbor, PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct PriceData {
    pub price: USDValue,
    pub timestamp: POSIXTime,
}
```

at the top of the source file containing the component where you wish to use the Religant Oracle method. 


__SECOND__: You must __declare the component's API__ using an `extern_blueprint!` macro __inside of your blueprint module__: 

``` rust
#[blueprint]
mod oracle_client {
    extern_blueprint! {
        "<TODO: UPDATE WITH MAINNET PACKAGE ADDRESS>",
        Religant {
        fn get_price(&self) -> Option<PriceData>;
        }
    }
...
```

__THIRD__: You must instantiate a reference to the global Religant component as a `const` inside of your blueprint. It should be possible to instantiate this reference either at the top level of your blueprint module, or inside of a method or function. Here, we do it at the top level: 

``` rust
 const RELIGANT: Global<Religant> =
        global_component!(Religant, "<UPDATE WITH MAINNET COMPONENT ADDRESS>");
```

__FINALLY__: You are free to use the methods of the instantiated component in your own method, for example: 

``` rust
        pub fn cash_xrd(&self) -> Bucket {
            match RELIGANT.get_price() {
                None => {Bucket::new(self.price_token_resource_address)},
                Some(price_data) => {
                            let resource_manager =
                                ResourceManager::from(self.price_token_resource_address);
                            resource_manager.mint(price_data.price)
                }
            }
        }
```

In this example, the `cash_xrd` method calls the Religant `get_price` method and mints a number of tokens equal to the current XRD/USD exchange rate. 

The full integration example contract can be found in this repository's `src/lib.rs` module.
