use itertools::Itertools;
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use quote::ToTokens;
use syn::ItemStruct;

/// Create a `TokenTree::Group` with the given delimiter and contents.
pub fn new_tokentree_group_with(delimiter: Delimiter, contents: Vec<TokenTree>) -> TokenTree {
    let mut contents_stream = TokenStream::new();
    contents_stream.extend(contents);
    TokenTree::Group(Group::new(delimiter, contents_stream))
}

/// Create a `TokenStream` causing a compile error at the given span.
pub(crate) fn create_compile_error_at(span: Span, message: &str) -> TokenStream {
    let mut tokens: Vec<TokenTree> = vec![
        TokenTree::Ident(Ident::new("compile_error", span)),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        new_tokentree_group_with(
            Delimiter::Parenthesis,
            vec![TokenTree::Literal(Literal::string(message))],
        ),
        TokenTree::Punct(Punct::new(';', Spacing::Alone)),
    ];

    for token in tokens.iter_mut() {
        token.set_span(span);
    }

    let mut ret = TokenStream::new();
    ret.extend(tokens);
    ret
}

/// Create a stream with the implementation of `compiled` for the given structure.
fn make_compiled_fn(fields: &[String]) -> TokenStream {
    format!(r"
    /// Compile all filters within `self`.
    pub fn compiled(self) -> Result<Self, Error> {{
        Ok(Self {{
            {}
        }})
    }}",
    fields.iter().map(|name| format!("{}: self.{}.compiled()?", name, name)).join(",")
    ).parse().unwrap()
}

/// Create a stream with the implementation of `is_none` for the given structure.
fn make_is_none_fn(fields: &[String]) -> TokenStream {
    format!(r"
    /// Check whether all filters are set to `Filter::None`.
    pub fn is_none(&self) -> bool {{
        {}
    }}",
    fields.iter().map(|name| format!("self.{}.is_none()", name)).join("&&")
    ).parse().unwrap()
}

/// Create a stream with the implementation of `into_fn_vec` for the given structure.
fn make_into_fn_vec_fn(fields: &[String], filtered_type: &str) -> TokenStream {
    format!(r"
    /// Return a `Vec` of closures for each non-`None` filter in `self`.
    /// Should be faster than invoking each and every filter each time.
    /// This method must not be called if there are uncompiled filters.
    pub fn into_fn_vec(self) -> Vec<Box<dyn Fn(&{}) -> bool + 'a>> {{
        [ {} ].into_iter().flatten().collect()
    }}",
    filtered_type,
    fields.iter().map(|name| format!("self.{}.into_fn(|value: &{}| &value.{})", name, filtered_type, name)).join(",")
    ).parse().unwrap()
}

/// Create a stream with an `impl` block for the given filter with its methods.
fn make_impl(fields: &[String], structure: &ItemStruct, filtered_type: &str) -> TokenStream {
    let mut stream = TokenStream::new();
    // `impl<generics> Name<generics>
    stream.extend::<TokenStream>(TokenTree::Ident(Ident::new("impl", Span::call_site())).into());
    stream.extend::<TokenStream>(structure.generics.to_token_stream().into());
    stream.extend::<TokenStream>(structure.ident.to_token_stream().into());
    stream.extend::<TokenStream>(structure.generics.to_token_stream().into());

    // Add functions, one by one.
    let mut impl_stream = TokenStream::new();
    impl_stream.extend(make_compiled_fn(fields));
    impl_stream.extend(make_is_none_fn(fields));
    impl_stream.extend(make_into_fn_vec_fn(fields, filtered_type));

    // Make a group out of all the methods.
    stream.extend::<TokenStream>(TokenTree::Group(Group::new(Delimiter::Brace, impl_stream)).into());
    stream
}

/// A macro to add on structures containing multiple `Filter`s.
///
/// Provides an `impl` for the structure with helpful methods to consider all filters as a single
/// one:
///     - `fn compiled(self) -> Result<Self, Error>`
///       Compile all filters within `self`.
///     - `fn is_none(&self) -> bool`
///       Check whether all fiilters are set to `Filter::None`.
///     - `fn into_fn_vec(self) -> Vec<Box<dyn Fn(&{}) -> bool + 'a>>`
///       Return a `Vec` of closures for each non-`None` filter in `self`.
///       Should be faster than invoking each and every filter each time.
///       This method must not be called if there are uncompiled filters.
///
/// The identifier of the type this filter is to be used upon must be given as an attribute
/// parameter of the macro: `#[api_filter(FooItem)]` will create methods to filter `FooItem`s.
pub fn api_filter(attr: TokenStream, item: TokenStream) -> Result<TokenStream, TokenStream> {
    // Retrieve the name of the type to filter from the attribute.
    let mut attr = attr.into_iter();
    let filtered_type = if let (Some(TokenTree::Ident(filtered_type)), None) = (attr.next(), attr.next()) {
        filtered_type.to_string()
    } else {
        return Err(create_compile_error_at(Span::call_site(), "Missing filtered type in attribute"));
    };

    // Parse the item as a structure and get a list of its fields.
    let structure = match syn::parse::<ItemStruct>(item) {
        Ok(x) => x,
        Err(x) => return Err(TokenStream::from(x.to_compile_error())),
    };
    let field_names = structure
        .fields
        .iter()
        .filter_map(|field| field.ident.as_ref().map(|id| id.to_string()))
        .collect_vec();

    // Copy the structure we decorate as-is, then add an `impl` block with the methods we need.
    let mut ret: TokenStream = structure.to_token_stream().into();
    ret.extend(make_impl(&field_names, &structure, &filtered_type));
    Ok(ret)
}
