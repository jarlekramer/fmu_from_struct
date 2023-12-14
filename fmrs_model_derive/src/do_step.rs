use proc_macro2::TokenStream as TokenStream2;

use quote::quote;
use syn;

pub fn impl_do_step(structure_name: &syn::Ident) -> TokenStream2 {
    let tokens = quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn fmi3DoStep(
            instance: fmi3Instance,
            currentCommunicationPoint: f64,
            communicationStepSize: f64,
            _noSetFMUStatePriorToCurrentPoint: bool,
            _eventHandlingNeeded: *mut bool,
            _terminateSimulation: *mut bool,
            _earlyReturn: *mut bool,
            _lastSuccessfulTime: *mut f64,
        ) -> fmi3Status {
            unsafe {
                let model: &mut #structure_name = &mut *(instance as *mut #structure_name);

                model.do_step(currentCommunicationPoint, communicationStepSize);
            }
            
            fmi3Status::fmi3OK
        }
    
    };

    tokens
}