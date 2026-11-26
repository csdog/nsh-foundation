use heck::ToSnakeCase;
use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Span, TokenStream as Tokens};
use quote::{ToTokens, format_ident, quote};
use rstml::{
    Parser, ParserConfig,
    node::{Node, NodeAttribute, NodeElement, NodeName},
};
use syn::{
    Expr, Ident, Result, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

/// Builds Native Shell elements with TSX-style tags inside Rust.
#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    expand(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn expand(input: TokenStream) -> Result<Tokens> {
    let library = match crate_name("components") {
        Ok(FoundCrate::Name(name)) => name,
        _ => "components".to_owned(),
    };
    let library_ident = format_ident!("{library}");
    let core = quote!(::#library_ident::__nsh_core::prelude);
    let parser = Parser::new(
        ParserConfig::new()
            .transform_block(move |input| style_object(input, &library))
            .macro_call_pattern(quote!(rsx! {%%})),
    );
    let nodes = parser.parse_recoverable(input).into_result()?;
    for node in &nodes {
        if !matches!(
            node,
            Node::Element(_) | Node::Fragment(_) | Node::Comment(_)
        ) {
            return Err(syn::Error::new_spanned(
                node,
                "expected a TSX-style tag or fragment at the root of rsx!",
            ));
        }
    }
    let emitter = Emitter {
        core: core.clone(),
        library: quote!(::#library_ident),
    };
    let tree = match nodes.as_slice() {
        [node] => emitter.node(node)?.unwrap_or_else(|| quote!(#core::view())),
        _ => emitter.view(&nodes)?,
    };
    Ok(quote!({
        #[allow(unused_imports)]
        use #core::{Accessible as _, Element as _, Interactive as _, ParentElement as _, Styled as _};
        #tree
    }))
}

struct Emitter {
    core: Tokens,
    library: Tokens,
}

impl Emitter {
    fn view(&self, children: &[Node]) -> Result<Tokens> {
        let core = &self.core;
        self.children(quote!(#core::view()), children)
    }

    fn children(&self, mut parent: Tokens, nodes: &[Node]) -> Result<Tokens> {
        for node in nodes {
            if let Some(child) = self.node(node)? {
                parent.extend(quote!(.child(#child)));
            }
        }
        Ok(parent)
    }

    fn node(&self, node: &Node) -> Result<Option<Tokens>> {
        let core = &self.core;
        Ok(Some(match node {
            Node::Element(element) => self.element(element)?,
            Node::Fragment(fragment) => self.view(&fragment.children)?,
            Node::Text(text) => {
                let text = &text.value;
                quote!(#core::text(#text))
            }
            Node::RawText(text) => {
                let text = text.to_string_best();
                quote!(#core::text(#text))
            }
            Node::Block(block) => block.to_token_stream(),
            Node::Comment(_) => return Ok(None),
            _ => {
                return Err(syn::Error::new_spanned(
                    node,
                    "expected an NSH element or Rust expression",
                ));
            }
        }))
    }

    fn element(&self, element: &NodeElement<rstml::node::Infallible>) -> Result<Tokens> {
        let NodeName::Path(path) = element.name() else {
            return Err(syn::Error::new_spanned(
                element.name(),
                "expected a component name",
            ));
        };
        let Some(name) = path.path.get_ident() else {
            return Err(syn::Error::new_spanned(
                path,
                "import the component builder before using its tag",
            ));
        };
        let name_string = name.to_string();
        let tag = name_string.as_str();
        let core = &self.core;
        let library = &self.library;
        let drawer = quote!(#library::drawer);
        if tag == "DrawerRoot" {
            let mut state = quote!(#drawer::Drawer::new(#core::signal(false)));
            for attr in element.attributes() {
                let (key, value) = attribute(attr)?;
                if key != "drawer" {
                    return Err(syn::Error::new_spanned(
                        key,
                        "expected the drawer attribute on DrawerRoot",
                    ));
                }
                state = quote!(#value);
            }
            let body = self.view(&element.children)?;
            return Ok(quote!(#drawer::DrawerRoot::new(#state, move || #body)));
        }
        let builder = format_ident!("{}", tag.to_snake_case(), span = name.span());
        let mut parent = match tag {
            "View" => quote!(#core::view()),
            "Text" => quote!(#core::text("")),
            "Button" => quote!(#library::button::button()),
            "Badge" => quote!(#library::badge::badge()),
            "Card" | "CardHeader" => quote!(#library::card::#name::new()),
            "CardTitle" | "CardDescription" => {
                let label = text_content(&element.children, name)?;
                quote!(#library::card::#name::new(#label))
            }
            "CardAction" | "CardContent" | "CardFooter" => {
                let body = self.view(&element.children)?;
                quote!(#library::card::#name::new(#body))
            }
            "Carousel" => quote!(#library::carousel::carousel_root()),
            "CarouselContent" | "CarouselPrevious" | "CarouselNext" => {
                quote!(#library::carousel::#name::new())
            }
            "CarouselItem" => {
                let body = self.view(&element.children)?;
                quote!(#library::carousel::CarouselItem::new(#body))
            }
            "DrawerTrigger" | "DrawerContent" | "DrawerClose" | "DrawerHeader" | "DrawerFooter"
            | "DrawerTitle" | "DrawerDescription" => quote!(#drawer::#builder()),
            _ => quote!(#builder()),
        };
        let mut button_surface = Tokens::new();
        let mut button_action = None;
        for attr in element.attributes() {
            let (key, value) = attribute(attr)?;
            let method = match key.to_string().as_str() {
                "class" => "app_class".into(),
                "style" => "style_with".into(),
                "accessibilityLabel" if matches!(tag, "View" | "Text") => "label".into(),
                "testID" => "test_id".into(),
                key => key.to_snake_case(),
            };
            let method = format_ident!("{method}", span = key.span());
            match (tag, method.to_string().as_str()) {
                ("Button", "on_press" | "href") => {
                    if button_action.replace(quote!(.#method(#value))).is_some() {
                        return Err(syn::Error::new_spanned(
                            key,
                            "expected only one Button action",
                        ));
                    }
                }
                ("Button", "app_class" | "test_id") => {
                    button_surface.extend(quote!(.#method(#value)));
                }
                _ => parent.extend(quote!(.#method(#value))),
            }
        }
        if tag == "DrawerContent" {
            let body = self.view(&element.children)?;
            return Ok(quote!(#parent.child(move || #body)));
        }
        match tag {
            "CardTitle" | "CardDescription" | "CardAction" | "CardContent" | "CardFooter"
            | "CarouselItem" => Ok(parent),
            "Card" | "CardHeader" | "Carousel" | "CarouselContent" => {
                self.slots(parent, tag, &element.children)
            }
            "Button" => {
                let parent = self.children(parent, &element.children)?;
                match button_action {
                    Some(action) => Ok(quote!(#parent #action #button_surface)),
                    None if !button_surface.is_empty() => {
                        Ok(quote!(#parent.surface() #button_surface))
                    }
                    None => Ok(parent),
                }
            }
            _ => self.children(parent, &element.children),
        }
    }

    fn slots(&self, mut parent: Tokens, tag: &str, children: &[Node]) -> Result<Tokens> {
        for child in children {
            if matches!(child, Node::Comment(_)) {
                continue;
            }
            let child_name = match child {
                Node::Element(element) => element.name().to_token_stream().to_string(),
                _ => String::new(),
            };
            let method = match (tag, child_name.as_str()) {
                ("Card", "CardHeader") => "header",
                ("Card", "CardContent") | ("Carousel", "CarouselContent") => "content",
                ("Card", "CardFooter") => "footer",
                ("CardHeader", "CardTitle") => "title",
                ("CardHeader", "CardDescription") => "description",
                ("CardHeader", "CardAction") => "action",
                ("Carousel", "CarouselPrevious") => "previous",
                ("Carousel", "CarouselNext") => "next",
                ("CarouselContent", "CarouselItem" | "") => "item",
                _ => {
                    return Err(syn::Error::new_spanned(
                        child,
                        format!("unsupported child of {tag}"),
                    ));
                }
            };
            let method = format_ident!("{method}");
            let child = self.node(child)?;
            parent.extend(quote!(.#method(#child)));
        }
        Ok(parent)
    }
}

fn text_content(children: &[Node], name: &Ident) -> Result<Tokens> {
    match children {
        [Node::Text(text)] => Ok(text.value.to_token_stream()),
        [Node::RawText(text)] => {
            let text = text.to_string_best();
            Ok(quote!(#text))
        }
        [Node::Block(block)] => Ok(block.to_token_stream()),
        _ => Err(syn::Error::new_spanned(
            name,
            "expected text or a Rust text expression",
        )),
    }
}

fn attribute(attr: &NodeAttribute) -> Result<(&Ident, Tokens)> {
    let NodeAttribute::Attribute(attr) = attr else {
        return Err(syn::Error::new_spanned(attr, "expected a named attribute"));
    };
    let NodeName::Path(path) = &attr.key else {
        return Err(syn::Error::new_spanned(
            &attr.key,
            "expected an attribute name",
        ));
    };
    let Some(key) = path.path.get_ident() else {
        return Err(syn::Error::new_spanned(path, "expected an attribute name"));
    };
    let value = match attr.value() {
        Some(Expr::Block(block)) if block.block.stmts.len() == 1 => match &block.block.stmts[0] {
            syn::Stmt::Expr(expr, None) => expr.to_token_stream(),
            _ => block.to_token_stream(),
        },
        Some(value) => value.to_token_stream(),
        None => quote!(true),
    };
    Ok((key, value))
}

struct StyleField {
    key: Ident,
    value: Expr,
}

impl Parse for StyleField {
    fn parse(input: ParseStream) -> Result<Self> {
        let key = input.parse()?;
        input.parse::<Token![:]>()?;
        Ok(Self {
            key,
            value: input.parse()?,
        })
    }
}

// Rstml normally parses braced values as Rust. Translate a TSX style object
// into a native style closure before it reaches the Rust expression parser.
fn style_object(input: ParseStream, library: &str) -> Result<Option<Tokens>> {
    if !input.peek(syn::token::Brace) {
        return Ok(None);
    }
    let fork = input.fork();
    let fields;
    syn::braced!(fields in fork);
    if !fork.is_empty()
        || !fields.peek(Ident)
        || !fields.peek2(Token![:])
        || fields.peek2(Token![::])
    {
        return Ok(None);
    }
    let fields;
    syn::braced!(fields in input);
    let fields = Punctuated::<StyleField, Token![,]>::parse_terminated(&fields)?;
    let library = format_ident!("{library}");
    let style_arg = Ident::new("__nsh_style", Span::mixed_site());
    let mut style = quote!(#style_arg);
    for StyleField { key, value } in fields {
        let method = match key.to_string().as_str() {
            "padding" => "p",
            "paddingHorizontal" => "px",
            "paddingVertical" => "py",
            "gap" => "gap",
            "fontSize" => "text_size",
            "width" => "w",
            "height" => "h",
            "borderRadius" => "rounded",
            _ => {
                return Err(syn::Error::new(
                    key.span(),
                    "unsupported RSX inline style property",
                ));
            }
        };
        let method = format_ident!("{method}", span = key.span());
        style.extend(quote!(.#method(::#library::__nsh_core::prelude::px((#value) as f32))));
    }
    Ok(Some(quote!(|#style_arg| #style)))
}
