pub struct RustBasicTest {
    pub property_test: String,
}

pub trait RustBasicTestImpl {
    fn property_test(&self) -> String;
    fn set_property_test(&mut self, value: &String);
}

impl RustBasicTestImpl for RustBasicTest {
    fn property_test(&self) -> String {
        return self.property_test.clone();
    }

    fn set_property_test(&mut self, value: &String) {
        self.property_test = value.clone();
    }
}

mod test {
    use super::*;
    use std::{sync::Mutex};

    #[test]
    fn rust_basic_test_property_test_impl_test() {
        let basic: Mutex<RustBasicTest> = Mutex::new(RustBasicTest {
            property_test: "DEFAULTVALUE".to_string(),
        });
        let result = basic.lock().unwrap().property_test();
        assert_eq!(result, "DEFAULTVALUE".to_string());
        basic
            .lock()
            .unwrap()
            .set_property_test(&"NEWVALUE".to_string());
        let result2 = basic.lock().unwrap().property_test();
        assert_eq!(result2, "NEWVALUE".to_string());
        return;
    }
}
