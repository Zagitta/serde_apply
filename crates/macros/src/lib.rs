use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Field, GenericParam, Generics};

#[proc_macro_derive(SerdeApply)]
pub fn derive_serde_apply(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let partial_name = format_ident!("__Partial{}", name);

    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    //let where_clause = where_clause.cloned().unwrap_or_else(|| parse_quote!());

    let fields = if let Data::Struct(s) = &input.data {
        &s.fields
    } else {
        panic!("Nope")
    };

    let struct_fields = fields.iter().map(|f| {
        let Field {
            attrs,
            vis,
            ident,
            colon_token,
            ty,
        } = f;

        quote! {#[serde(default)]  #(#attrs)* #vis #ident #colon_token <#ty as ::serde_apply::SerdeApply>::PartialType}
    });
    let field_applies = fields.iter().map(|f| {
        let Field { ident, .. } = f;
        quote! {::serde_apply::SerdeApply::apply(&mut self.#ident, with.#ident);}
    });

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        #[doc(hidden)]
        #[derive(serde::Deserialize, Default)]
        struct #partial_name #ty_generics #where_clause
        {
            #(#struct_fields,)*
        }

        #[automatically_derived]
        impl #impl_generics ::serde_apply::SerdeApply for #name #ty_generics #where_clause {
            type PartialType = #partial_name #ty_generics;

            fn apply(&mut self, with: Self::PartialType)
            {
                #(#field_applies;)*
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

/// Add a bound `T: SerdeApply` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(::serde_apply::SerdeApply));
        }
    }
    generics
}
