use scrypto::prelude::*;

/* Think we need to re-define this here? */
#[derive(ScryptoSbor, PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct PriceData {
    pub price: Decimal,
    pub timestamp: i64,
}

#[blueprint]
mod oracle_client {
    extern_blueprint! {
        "package_tdx_2_1p50j7463yhtpmq8e9t4vklw8jfuccl0xhe7g2564w8w74nrmrsacxs",
        Religant {
        fn get_price(&self) -> Option<PriceData>;
        }
    }

    const RELIGANT: Global<Religant> =
        global_component!(Religant, "component_tdx_2_1cpekt6s65g8025zgstwx4t0tpdsegafse0vtjnfms9k07mcmnr96cm");

    struct OracleClient {
        price_token_resource_address: ResourceAddress,
    }

    impl OracleClient {
        // Returns a bucket with a single price tracker token
        pub fn instantiate_client() -> Global<OracleClient> {
            let price_token = ResourceBuilder::new_fungible(OwnerRole::None)
                  .mint_roles(mint_roles!{
                      minter => rule!(allow_all);
                      minter_updater => rule!(allow_all);
                  })
                  .create_with_no_initial_supply();


            let component = Self {
                price_token_resource_address: price_token.address(),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize();
            component
        }

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
    }
}
