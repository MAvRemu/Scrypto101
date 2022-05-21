use scrypto::prelude::*;

blueprint! {
    struct GreedGenerous {
        // Define what resources and data will be managed by Hello components
        xrd_vault: Vault,
        sample_vault: Vault,

    }

    impl GreedGenerous {

        pub fn instantiate_greed_generous() -> ComponentAddress {
            // Create a new token called "HelloToken," with a fixed supply of 1000, and put that supply into a bucket
            let my_bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "HelloToken")
                .metadata("symbol", "HT")
                .initial_supply(1000);

            // Instantiate a Hello component, populating its vault with our supply of 1000 HelloToken
            Self {
                sample_vault: Vault::with_bucket(my_bucket),
                xrd_vault: Vault::new(RADIX_TOKEN),
            }
            .instantiate()
            .globalize()
        }

        //pub fn iamgreedy() -> Bucket {
        //    // give 110% of the payment
        //}

        pub fn iamgenerous(&mut self, mut payment: Bucket) -> Bucket  {
            // put payment in xrd_vault

            let amount = payment.amount();
            self.xrd_vault.put(payment.take(amount));
            info!("your payment of {} is taken, the following is returned: {}", amount, payment.amount());
            info!("the XRD vault now contains: {}", self.xrd_vault.amount());
            payment
        }


        pub fn free_token(&mut self) -> Bucket {
            info!("My balance is: {} HelloToken. Now giving away a token!", self.sample_vault.amount());
            self.sample_vault.take(1)
        }
    }
}
