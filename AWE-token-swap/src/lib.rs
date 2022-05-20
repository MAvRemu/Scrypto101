use scrypto::prelude::*;

blueprint! {
    struct TokenSwap {
        // Define what resources and data will be managed by Hello components
        awesome_token_vault: Vault,
        collected_xrd_vault: Vault,
        awesome_token_price: Decimal,
    }

    impl TokenSwap {
        // Implement the functions and methods which will manage those resources and data

        // This is a function, and can be called directly on the blueprint once deployed
        pub fn instantiate_awesome_token(price: Decimal) -> ComponentAddress {
            // Create a new token called "HelloToken," with a fixed supply of 1000, and put that supply into a bucket
            let my_bucket: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", "Awesome")
                .metadata("symbol", "AWE")
                .initial_supply(1000000);

            // Instantiate a Hello component, populating its vault with our supply of 1000 HelloToken
            Self {
                awesome_token_vault: Vault::with_bucket(my_bucket),
                awesome_token_price: price,
                collected_xrd_vault: Vault::new(RADIX_TOKEN),

            }
            .instantiate()
            .globalize()
        }

        // This is a method, because it needs a reference to self.  Methods can only be called on components
        pub fn free_token(&mut self) -> Bucket {
            info!(
                "My balance is: {} HelloToken. Now giving away a token!",
                self.awesome_token_vault.amount()
            );
            // If the semi-colon is omitted on the last line, the last value seen is automatically returned
            // In this case, a bucket containing 1 HelloToken is returned
            self.awesome_token_vault.take(1*2)
        }

        pub fn swap_token_count(&mut self, mut payment:Bucket, count:Decimal) -> (Bucket, Bucket) {

            info!(
                "Your maximum payment is: {}",
                payment.amount()
            );

            self.collected_xrd_vault.put(payment.take(self.awesome_token_price));


            info!(
                "The payment was: {}",
                self.awesome_token_price
            );

            let ticket = self.awesome_token_vault.take(self.awesome_token_price*count);

            info!(
                "Your change is: {}",
                payment.amount()
            );

            (ticket, payment)

         }
         pub fn swap_token_based_on_price(&mut self, mut payment:Bucket) -> (Bucket, Bucket) {

            let amount = payment.amount();
            self.collected_xrd_vault.put(payment.take(amount));

            let ticket = self.awesome_token_vault.take(amount*2);

            info!("Your payment in XRD is {} and you are getting {} AWE tokens :)", amount, ticket.amount());

            (ticket, payment)
         }
    }
}
