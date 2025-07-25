mod bindings;
mod pure_rust_types;
mod winrt_rust_types;

use std::collections::HashMap;
use windows::{Win32::Foundation::*, Win32::System::WinRT::*, core::*};

use crate::{
    bindings::{BasicTest, BasicTestDatabase},
    winrt_rust_types::{BasicTestDatabaseFactory, BasicTestFactory},
};

// We need an instantiation for each class we want to produce.
static BASIC_TEST_FACTORY: StaticComObject<BasicTestFactory> = BasicTestFactory.into_static();
static BASIC_TEST_DATABASE_FACTORY: StaticComObject<BasicTestDatabaseFactory> =
    BasicTestDatabaseFactory.into_static();

#[unsafe(no_mangle)]
unsafe extern "system" fn DllGetActivationFactory(
    name: Ref<HSTRING>,
    factory: OutRef<IActivationFactory>,
) -> HRESULT {
    let mut class_factories: HashMap<&'static str, IActivationFactory> = HashMap::new();
    class_factories.insert(
        BasicTest::NAME,
        BASIC_TEST_FACTORY.to_interface::<IActivationFactory>(),
    );
    class_factories.insert(
        BasicTestDatabase::NAME,
        BASIC_TEST_DATABASE_FACTORY.to_interface::<IActivationFactory>(),
    );

    match class_factories.get(&*name.to_string_lossy()) {
        Some(found_factory) => return factory.write(Some(found_factory.clone())).into(),
        None => {
            _ = factory.write(None);
            return CLASS_E_CLASSNOTAVAILABLE;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bindings;

    // I've had some difficulty with this test being successful in the past due to Runtime class not being found,
    // but hopefully just works now.
    #[test]
    fn basic_test_impl_test() -> Result<()> {
        let basic: bindings::BasicTest =
            bindings::BasicTest::CreateInstance(&Into::<HSTRING>::into("PING"))?;
        let result = basic.Run(&Into::<HSTRING>::into("PING")).unwrap();
        assert_eq!(result, Into::<HSTRING>::into("PONG"));
        return Ok(());
    }

    #[test]
    fn basic_test_property_test_impl_test() -> Result<()> {
        let basic: bindings::BasicTest =
            bindings::BasicTest::CreateInstance(&Into::<HSTRING>::into("DEFAULTVALUE"))?;
        let result = basic.PropertyTest().unwrap();
        assert_eq!(result, Into::<HSTRING>::into("DEFAULTVALUE"));
        basic
            .SetPropertyTest(&Into::<HSTRING>::into("NEWVALUE"))
            .unwrap();
        let result2: HSTRING = basic.PropertyTest().unwrap();
        assert_eq!(result2, Into::<HSTRING>::into("NEWVALUE"));
        return Ok(());
    }
}
