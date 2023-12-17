use proc_macro2::TokenStream as TokenStream2;

use quote::quote;
use syn;

pub fn impl_model_managment(model_name: &syn::Ident) -> TokenStream2 {
    quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn fmi3InstantiateCoSimulation(
            _instance_name: fmi3String,
            _instantiation_token: fmi3String,
            _resource_path: fmi3String,
            _visible: bool,
            _logging_on: bool,
            _event_mode_used: bool,
            _early_return_allowed: bool,
            _required_intermediate_variables: *const u32,
            _n_required_intermediate_variables: usize,
            _instance_environment: fmi3InstanceEnvironment,
            _log_message: fmi3LogMessageCallback,
            _intermediate_update: fmi3IntermediateUpdateCallback,
        ) -> *mut ffi::c_void {
            // The box is needed to avoid the model to be dropped when it goes out of scope.
            let mut model = Box::new(#model_name::default());

            let ptr = Box::into_raw(model) as *mut _;

            ptr as *mut ffi::c_void
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn fmi3FreeInstance(instance: *mut ffi::c_void) {
            if !instance.is_null() {
                let instance = instance as *mut #model_name;

                let _box = Box::from_raw(instance);
                // _box is dropped here and memory is deallocated
            }
        }
    }
}

