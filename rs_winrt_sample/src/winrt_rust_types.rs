use crate::bindings;
use crate::pure_rust_types::*;

use std::sync::Mutex;
use std::vec;

use windows::{Win32::Foundation::*, Win32::System::WinRT::*, core::*};

// The `implement` attribute provides the boilerplate COM and WinRT implementation support.
#[implement(bindings::BasicTest)]
struct BasicTest {
    // Winrt classes are already reference counted
    internal: Mutex<RustBasicTest>,
}

impl bindings::IBasicTest_Impl for BasicTest_Impl {
    fn Run(&self, _value: &HSTRING) -> Result<HSTRING> {
        return Ok("PONG".into());
    }

    fn PropertyTest(&self) -> windows_core::Result<windows_core::HSTRING> {
        match self.internal.lock() {
            Ok(prop_test) => {
                let test = prop_test.property_test();
                let s: &str = test.as_str();
                return Ok(Into::<HSTRING>::into(s));
            }
            Err(_) => return Err(E_ILLEGAL_STATE_CHANGE.into()),
        }
    }

    fn SetPropertyTest(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        match self.internal.lock() {
            Ok(mut proptest_mut) => {
                proptest_mut.set_property_test(&value.to_string_lossy());
            }
            Err(_) => return Err(E_ILLEGAL_STATE_CHANGE.into()),
        }
        return Ok(());
    }
}

// The `BasicTestFactory` struct represents the implementation of the `BasicTest` class factory.
#[implement(IActivationFactory, bindings::IBasicTestFactory)]
pub struct BasicTestFactory;

// The BasicTest class doesn't provide a default constructor but WinRT still requires an
// implementation of `IActivationFactory`.
impl IActivationFactory_Impl for BasicTestFactory_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Err(E_NOTIMPL.into())
    }
}

// Implement the `IBasicTestFactory` interface.
impl bindings::IBasicTestFactory_Impl for BasicTestFactory_Impl {
    fn CreateInstance(&self, value: &HSTRING) -> Result<bindings::BasicTest> {
        Ok(BasicTest {
            internal: Mutex::new(RustBasicTest {
                property_test: value.to_string_lossy(),
            })
            .into(),
        }
        .into())
    }
}

#[implement(bindings::BasicTestDatabase)]
struct BasicTestDatabase {}

impl bindings::IBasicTestDatabase_Impl for BasicTestDatabase_Impl {
    fn GetCollection(
        &self,
        size: u32,
    ) -> windows_core::Result<windows_core::Array<bindings::BasicTest>> {
        let mut vector = vec![];
        for i in 0..size {
            vector.push(Some(bindings::BasicTest::CreateInstance(
                &Into::<HSTRING>::into(i.to_string()),
            )?));
        }
        return Ok(windows_core::Array::<bindings::BasicTest>::from_slice(
            vector.as_mut_slice(),
        ));
    }
}

// The `BasicTestFactory` struct represents the implementation of the `BasicTest` class factory.
#[implement(IActivationFactory, bindings::IBasicTestDatabaseFactory)]
pub struct BasicTestDatabaseFactory;

// Implement the default constructor
impl IActivationFactory_Impl for BasicTestDatabaseFactory_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Ok(BasicTestDatabase {}.into())
    }
}

// Implement the `IBasicTestDatabaseFactory` interface.
impl bindings::IBasicTestDatabaseFactory_Impl for BasicTestDatabaseFactory_Impl {
    fn CreateInstance(&self, _value: bool) -> Result<bindings::BasicTestDatabase> {
        Ok(BasicTestDatabase {}.into())
    }
}

mod test {

    use super::*;
    use bindings;

    // I've had some difficulty with this test being successful in the past due to Runtime class not being found,
    // but hopefully just works now.
    #[test]
    fn basic_test_impl_test() -> Result<()> {
        let basic: bindings::BasicTest =
            bindings::BasicTest::CreateInstance(&Into::<HSTRING>::into("FIRSTVALUE"))?;
        let result = basic.Run(&Into::<HSTRING>::into("FIRSTVALUE")).unwrap();
        assert_eq!(result, Into::<HSTRING>::into("PONG"));
        return Ok(());
    }

    #[test]
    fn basic_test_property_test_impl_test() -> Result<()> {
        let basic: bindings::BasicTest =
            bindings::BasicTest::CreateInstance(&Into::<HSTRING>::into("FIRSTVALUE"))?;
        let result = basic.PropertyTest().unwrap();
        assert_eq!(result, Into::<HSTRING>::into("FIRSTVALUE"));
        let basic_clone = basic.clone();
        basic
            .SetPropertyTest(&Into::<HSTRING>::into("NEWVALUE"))
            .unwrap();
        let result2: HSTRING = basic.PropertyTest().unwrap();
        assert_eq!(result2, Into::<HSTRING>::into("NEWVALUE"));
        let result_clone: HSTRING = basic_clone.PropertyTest().unwrap();
        assert_eq!(result_clone, result2);
        return Ok(());
    }

    #[test]
    fn basic_test_database_impl_test() -> Result<()> {
        let basic_test_database = bindings::BasicTestDatabase::CreateInstance(false)?;
        let basic = basic_test_database.GetCollection(16)?;
        assert_eq!(basic.len(), 16);
        return Ok(());
    }
    
    #[test]
    fn basic_test_database_default_constructor_impl_test() -> Result<()> {
        let basic_test_database = bindings::BasicTestDatabase::new()?;
        let basic = basic_test_database.GetCollection(16)?;
        assert_eq!(basic.len(), 16);
        return Ok(());
    }
}
