use proc_macro2::TokenStream as TokenStream2;

use quote::quote;
use syn;

pub fn impl_model_managment(model_name: &syn::Ident) -> TokenStream2 {
    quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn fmi3InstantiateCoSimulation(
            _instanceName: fmi3String,
            _instantiationToken: fmi3String,
            _resourcePath: fmi3String,
            _visible: bool,
            _loggingOn: bool,
            _eventModeUsed: bool,
            _earlyReturnAllowed: bool,
            _requiredIntermediateVariables: *const fmi3ValueReference,
            _nRequiredIntermediateVariables: usize,
            _instanceEnvironment: fmi3InstanceEnvironment,
            _logMessage: fmi3LogMessageCallback,
            _intermediateUpdate: fmi3IntermediateUpdateCallback,
        ) -> fmi3Instance {
            // The box is needed to avoid the model to be dropped when it goes out of scope.
            let mut model = Box::new(#model_name::default());

            let ptr = Box::into_raw(model) as *mut _;

            ptr as fmi3Instance
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn fmi3FreeInstance(instance: fmi3Instance) {
            if !instance.is_null() {
                let instance = instance as *mut #model_name;

                let _box = Box::from_raw(instance);
                // _box is dropped here and memory is deallocated
            }
        }
    }
}

