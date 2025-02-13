// Structs contain data, but can also have logic. In this exercise, we have
// defined the `Package` struct, and we want to test some logic attached to it.

#[derive(Debug)]

// We have a package struct with sender_country, recipient_country, and weight_in_grams fields.
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: u32,
}

// We implement Package struct with functions new, is_international, and get_fees.
// the new function expects sender_country, recipient_country, and weight_in_grams 
// its return type is Self meaning it returns the Package struct with the given values.
// the new function is used to create a new package with the given values and it panics if the weight_in_grams is less than 10.

// the is_international function does not have a return type, it is used to check if the
// if the package is an international package. 
// a package is considered international if the sender_country is not equal to the recipient_country, based on the tests
impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Self {
        if weight_in_grams < 10 {
            // This isn't how you should handle errors in Rust, but we will
            // learn about error handling later.
            panic!("Can't ship a package with weight below 10 grams");
        }

        Self {
            sender_country,
            recipient_country,
            weight_in_grams,
        }
    }

    // Added the correct return type bool to the function signature, based on the tests
    fn is_international(&self)-> bool {
        // Read the tests and came to the conclusion that 
        // the package is international if the sender_country is not equal to the recipient_country

        // to implement this check we simply use != to compare them with the self keyword 
        self.sender_country != self.recipient_country
    }

    // Added the correct return type u32 the function signature, based on the tests
    fn get_fees(&self, cents_per_gram: u32)-> u32 {
        // TODO: Calculate the package's fees.
        // the fees are calculated by multiplying the weight_in_grams by cents_per_gram
        // we use the self keyword to access the weight_in_grams field of the package
        self.weight_in_grams * cents_per_gram
    }
}

// in conclusion, we learned about implementing structs and using functions to add logic to the struct 
// while correctly implementing the function signatures

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");

        Package::new(sender_country, recipient_country, 5);
    }

    #[test]
    // this test checks if the package is international by 
    // checking if the sender_country is not equal to the recipient_country

    fn create_international_package() {
        // first we create a package with sender_country as Spain and recipient_country as Russia
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        // then we create a package with the given values, sender_country, recipient_country, and weight_in_grams
        let package = Package::new(sender_country, recipient_country, 1200);
        // lastly we use assert! to check if the package is international
        // we do this by callinng the is_international function on the package
        // if assert! returns true, the test passes
        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        // first we create a package with sender_country as Canada and recipient_country as Canada
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();
        // then we create a package with the given values, sender_country, recipient_country, and weight_in_grams
        let package = Package::new(sender_country, recipient_country, 1200);
        // lastly we use assert! to check if the package is not international
        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        // to calculate transport fees we need to know the cents_per_gram
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");
        
        let cents_per_gram = 3;
        // we create a package and set weight_in_grams to 1500
        let package = Package::new(sender_country, recipient_country, 1500);
        // lastly we use assert_eq! to check if the fees are calculated correctly by calling the get_fees function on the package
        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}
