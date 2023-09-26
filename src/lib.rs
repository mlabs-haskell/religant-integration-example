use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData)]
struct PriceTokenData {
    #[mutable]
    price: Decimal,
    #[mutable]
    timestamp: i64,
}

/* Think we need to re-define this here? */
#[derive(ScryptoSbor, PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct PriceData {
    pub price: Decimal,
    pub timestamp: i64,
}

#[blueprint]
mod oracle_client {
    extern_blueprint! {
        "package_tdx_e_1p5ckcfghvdtd6yk6x4qtef2xj58le8s7x2696rfdhx53w3h5jq7tec",
        Religant {
        fn get_price(&self) -> Option<PriceData>;
        }
    }

    const RELIGANT: Global<Religant> =
        global_component!(Religant, "component_tdx_e_1czevc8l4ym02gtcp8z5k66gfwkvnvd34qu0gutnfv8tg3232qcmv0j");

    struct OracleClient {
        price_token_resource_address: ResourceAddress,
        token_admin_badge: Vault,
    }

    impl OracleClient {
        // Returns a bucket with a single price tracker token
        pub fn instantiate_client() -> (Global<OracleClient>, Bucket) {
            let admin_badge: Bucket = Bucket::from(
                ResourceBuilder::new_fungible(OwnerRole::None)
                    .divisibility(DIVISIBILITY_NONE)
                    .mint_initial_supply(1),
            );

            let admin_badge_address: ResourceAddress = admin_badge.resource_address();

            let price_token: Bucket = Bucket::from(ResourceBuilder::new_integer_non_fungible::<PriceTokenData>(OwnerRole::None)
              .metadata(metadata! {
                init {
                    "name" => "FEED_PRICE", locked;
                    "description" => "token to facilitate testnet testing of Religant oracle", locked;
                }
            }).non_fungible_data_update_roles(non_fungible_data_update_roles!(
                  non_fungible_data_updater => rule!(require(admin_badge_address));
                  non_fungible_data_updater_updater => rule!(require(admin_badge_address));
              )).mint_initial_supply(vec![
                  (IntegerNonFungibleLocalId::new(1), PriceTokenData{price: dec!("0"), timestamp: 0})
              ]));

            let component = Self {
                price_token_resource_address: price_token.resource_address(),
                token_admin_badge: Vault::with_bucket(admin_badge),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize();
            (component,price_token)
        }

        pub fn update_token(&self) -> () {
            match RELIGANT.get_price() {
                None => {}
                Some(price_data) => {
                    self.token_admin_badge
                        .as_fungible()
                        .authorize_with_amount(1, || {
                            let resource_manager =
                                ResourceManager::from(self.price_token_resource_address);
                            resource_manager.update_non_fungible_data(
                                &NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(1)),
                                "price",
                                price_data.price,
                            );
                            resource_manager.update_non_fungible_data(
                                &NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(1)),
                                "timestamp",
                                price_data.timestamp,
                            );
                        })
                }
            }
        }
    }
}
