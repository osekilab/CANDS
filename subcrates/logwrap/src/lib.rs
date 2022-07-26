use proc_macro::TokenStream;

use syn::{ Block, FnArg, GenericParam, Ident, ItemFn, Pat };

use quote::{ format_ident, quote, ToTokens };

#[proc_macro_attribute]
pub fn logwrap(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let orig: ItemFn = syn::parse(item).unwrap();

    let mut wrapped = orig.clone();
    let mut wrapper = orig.clone();

    //  Change original function name from `<func>` to `__logwrap_internal_<func>`
    let wrapped_ident = format_ident!("__logwrap_internal__{}", orig.sig.ident);
    wrapped.sig.ident = wrapped_ident.clone();

    //  Does the original function have a `self` receiver?
    let has_receiver = wrapper.sig.receiver().is_some();

    //  Get the list of arguments.
    let args: Vec<Ident> = wrapper.sig.inputs.iter()
        .map(|fn_arg| match fn_arg {
            &FnArg::Receiver(_) => None,
            &FnArg::Typed(ref pat_type) => match *(pat_type.pat) {
                Pat::Ident(ref pat_ident) => Some(pat_ident.ident.clone()),
                _ => None,
            }
        })
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect();

    //  Get the list of generics.
    let generics: Vec<Box<dyn ToTokens>> = wrapper.sig.generics.params.iter()
        .map(|generic_param| match generic_param {
            &GenericParam::Type(ref ty) =>
                Some(Box::new(ty.ident.clone()) as Box<dyn ToTokens>),

            &GenericParam::Lifetime(ref lt_def) =>
                Some(Box::new(lt_def.lifetime.clone()) as Box<dyn ToTokens>),

            _ => None,
        })
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect();

    //  Change wrapper code.
    let new_block: Block = syn::parse(
        quote! {
            {
                inc!();
                let res = #wrapped_ident :: < #(#generics),* > ( #(#args),* );
                dec!();
                res
            }
        }.into()
    ).unwrap();
    wrapper.block = Box::new(new_block);

    let strm1: TokenStream = wrapped.into_token_stream().into();
    let strm2: TokenStream = wrapper.into_token_stream().into();

    //  https://doc.rust-lang.org/proc_macro/struct.TokenStream.html#impl-FromIterator%3CTokenStream%3E
    TokenStream::from_iter([ strm1, strm2 ])
}