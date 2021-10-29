extern crate proc_macro;
use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::{DeriveInput, Field, Fields, Index, ItemStruct};

#[proc_macro_derive(PushToStack)]
pub fn push_to_stack(input: TokenStream) -> TokenStream {
    let ast: ItemStruct = syn::parse(input).unwrap();

    let name = ast.ident;
    let gens = ast.generics;

    let fields = match ast.fields {
        Fields::Named(named) => named
            .named
            .into_iter()
            .map(|field: Field| {
                let field_name = field.ident.clone().unwrap();
                quote! { #field_name }
            })
            .collect::<Vec<_>>(),
        Fields::Unnamed(unnamed) => {
            let fields = unnamed.unnamed.len();
            (0..fields)
                .into_iter()
                .map(|field| {
                    let field = Index::from(field);
                    quote! {
                        #field
                    }
                })
                .collect::<Vec<_>>()
        }
        Fields::Unit => {
            panic!("Can not push unit to stack")
        }
    };
    let (impl_generics, ty_generics, where_clause) = gens.split_for_impl();
    let push_to_stack_impl = quote! {
         impl #impl_generics PushToStack for #name #ty_generics #where_clause {

                fn push_to_stack(self, stack: &mut Stack) {
                    #(
                    self.#fields.push_to_stack(stack);
                    )*
                }

        }
    };

    let expanded = quote! {
        #push_to_stack_impl
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(PopFromStack)]
pub fn pop_from_stack(input: TokenStream) -> TokenStream {
    let ast: ItemStruct = syn::parse(input).unwrap();

    let name = ast.ident;
    let gens = ast.generics;

    let (fields, types) = match &ast.fields {
        Fields::Named(named) => (
            named
                .named
                .iter()
                .map(|field: &Field| {
                    let field_name = field.ident.clone().unwrap();
                    quote! { #field_name }
                })
                .rev()
                .collect::<Vec<_>>(),
            named
                .named
                .iter()
                .map(|field: &Field| {
                    let field_name = field.ty.clone();
                    quote! { #field_name }
                })
                .rev()
                .collect::<Vec<_>>(),
        ),
        Fields::Unnamed(unnamed) => {
            let fields = unnamed.unnamed.len();
            (
                (0..fields)
                    .into_iter()
                    .map(|field| {
                        quote! {
                            #field
                        }
                    })
                    .rev()
                    .collect::<Vec<_>>(),
                unnamed
                    .unnamed
                    .iter()
                    .map(|field: &Field| {
                        let field_name = field.ty.clone();
                        quote! { #field_name }
                    })
                    .rev()
                    .collect::<Vec<_>>(),
            )
        }
        Fields::Unit => {
            panic!("Can not push unit to stack")
        }
    };

    let (impl_generics, ty_generics, where_clause) = gens.split_for_impl();

    let push_to_stack_impl = quote! {
        impl #impl_generics PopFromStack for #name #gens #where_clause {

                fn pop_from_stack(stack: &mut Stack) -> Option<Self> {
                    Some(Self {
                        #(
                        #fields: stack.pop::<#types>()?
                        ),*
                    })
                }

        }
    };

    let expanded = quote! {
        #push_to_stack_impl
    };

    TokenStream::from(expanded)
}
