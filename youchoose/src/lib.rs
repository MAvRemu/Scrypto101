use scrypto::prelude::*;

blueprint! {
    struct YouChoose {
        // Define what resources and data will be managed by Hello components
        xrd_vault: Vault,
        generous_vault: Vault,
        desperate_vault: Vault,
        greedy_vault: Vault,
    }

    impl YouChoose {

        pub fn in_youchoose() -> ComponentAddress {
            let generous_bucket = ResourceBuilder::new_fungible()
            .metadata("name", "generous")
            .metadata("ticket", "GRS")
            .metadata("description", "stack KARMA for a peaceful life :)")
            .metadata("URL", "www.youchoose.app")
            .initial_supply(1000000);

            let desperate_bucket = ResourceBuilder::new_fungible()
            .metadata("name", "desperate")
            .metadata("ticket", "DPR")
            .metadata("description", "stack KARMA for a peaceful life :)")
            .metadata("URL", "www.youchoose.app")
            .initial_supply(1000000);

            let greedy_bucket = ResourceBuilder::new_fungible()
            .metadata("name", "greedy")
            .metadata("ticket", "GRY")
            .metadata("description", "stack KARMA for a peaceful life :)")
            .metadata("URL", "www.youchoose.app")
            .initial_supply(1000000);

            Self {
                xrd_vault: Vault::new(RADIX_TOKEN),
                generous_vault: Vault::with_bucket(generous_bucket),
                desperate_vault: Vault::with_bucket(desperate_bucket),
                greedy_vault: Vault::with_bucket(greedy_bucket),
            }
            .instantiate()
            .globalize()
        }

        pub fn greedy(&mut self, mut payment: Bucket) -> (Bucket, Bucket, Bucket) {
            // give doublevthe payment with a 50% change
            let amount = payment.amount();
            let x = Runtime::generate_uuid();
            let y = (x % 999999) % 999; //info!("y: {}", y);

            info!("the XRD vault contains: {}", self.xrd_vault.amount());

            if self.xrd_vault.amount() > amount {
                if y <500 {
                let ticket = self.xrd_vault.take(amount*2);
                let ticket2 = self.greedy_vault.take(amount*2);
                info!("You won, you are receiving double you stack: {} XRD and {} GRY", ticket.amount(), ticket2.amount());

                (ticket, ticket2, payment)
                } else {
                let ticket = self.xrd_vault.take(0);
                let ticket2 = self.greedy_vault.take(0);
                self.xrd_vault.put(payment.take(amount));
                info!("You lost, your money is gone");
                (ticket, ticket2, payment)
                }
            } else {
                let ticket = self.xrd_vault.take(0);
                let ticket2 = self.greedy_vault.take(0);
                info!("Sorry the XRD vault only contains: {} XRD", self.xrd_vault.amount());
                (ticket, ticket2, payment)
            }

        }

        pub fn generous(&mut self, mut payment: Bucket) -> (Bucket, Bucket)  {
            // put payment in xrd_vault ang give KARMA back
            let amount = payment.amount();
            self.xrd_vault.put(payment.take(amount));
            let ticket = self.generous_vault.take(amount);
            info!("your payment of {} XRD is taken, the following amount of KARMA is returned: {}", amount, ticket.amount());
            info!("the XRD vault now contains: {}", self.xrd_vault.amount());
            (ticket, payment)
        }

        pub fn desperate(&mut self) -> (Bucket, Bucket) {
            // give 1 xrd for the catcha work
            let ticket = self.xrd_vault.take(1);
            let ticket2 = self.desperate_vault.take(1);
            info!("You have been given: {} XRD and {} DPR for your captcha work", ticket.amount(), ticket2.amount());
            (ticket, ticket2)
        }
    }
}
