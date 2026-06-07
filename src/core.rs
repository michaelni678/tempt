use std::collections::HashMap;

use proc_macro2::{Group, Ident, TokenStream, TokenTree};
use tout::diagnostic::{error, error_ranged};
use tout::extension::{GroupExt, PunctExt, TokenTreeExt};
use tout::parser::Parser;
use tout::visitor::{Hooks, Visitor};

pub fn expand(input: TokenStream) -> Result<TokenStream, TokenStream> {
    let mut parser = Parser::new(input);
    let tables = tables(&mut parser)?;

    Ok(template(&mut parser, tables))
}

type Table = HashMap<Ident, Group>;

fn tables(parser: &mut Parser) -> Result<Vec<Table>, TokenStream> {
    let mut tables: Vec<Table> = Vec::new();

    let semicolon = loop {
        if let Some(semicolon) = parser.next_punct_if(|punct| punct.is_char(';')) {
            break semicolon;
        }

        let placeholders: Vec<Ident> = parser.next_idents().collect();
        let replacements: Vec<Group> = parser.next_groups().collect();

        if placeholders.is_empty() && replacements.is_empty() {
            return Err(parser.error("expected tables or `;`"));
        }

        if placeholders.is_empty()
            && let Some(left) = replacements.first()
            && let Some(right) = replacements.last()
        {
            return Err(error_ranged(
                left.span(),
                right.span(),
                "replacements are missing placeholders",
            ));
        }

        if replacements.is_empty()
            && let Some(first) = placeholders.first()
        {
            return Err(error(first.span(), "expected replacements or `;`"));
        }

        let remainder = replacements.len() % placeholders.len();

        if remainder != 0
            && let Some(left) = placeholders.get(remainder)
            && let Some(right) = placeholders.last()
        {
            return Err(error_ranged(
                left.span(),
                right.span(),
                "placeholders are missing replacements",
            ));
        }

        let substitutions = placeholders.into_iter().cycle().zip(replacements);

        for (placeholder, replacement) in substitutions {
            let table = tables
                .iter_mut()
                .find(|table| !table.contains_key(&placeholder));

            let table = match table {
                Some(table) => table,
                None => tables.push_mut(Table::new()),
            };

            table.insert(placeholder, replacement);
        }
    };

    if tables.len() >= 2
        && let Some(first) = tables.first()
        && let Some(last) = tables.last()
        && first.len() != last.len()
    {
        return Err(error(
            semicolon.span(),
            "all placeholders must have the same number of replacements",
        ));
    }

    Ok(tables)
}

fn template(parser: &mut Parser, tables: Vec<Table>) -> TokenStream {
    let mut output = TokenStream::new();

    let mut hooks = Hooks::new().punct(|output, punct, parser| {
        if punct.is_char('#')
            && let Some((repetition, _)) = parser.next2_if_map_trees_and(
                TokenTree::into_group,
                Group::is_parenthesized,
                TokenTree::into_punct,
                |punct| punct.is_char('*'),
            )
        {
            for table in &tables {
                let mut hooks = Hooks::new().punct(|output, punct, parser| {
                    if punct.is_char('#')
                        && let Some(replacement) =
                            parser.next_if_map_ident(|ident| table.get(&ident).ok_or(ident))
                    {
                        output.extend(replacement.stream());
                        return None;
                    }

                    Some(punct)
                });

                hooks.visit_stream(output, repetition.stream());
            }

            return None;
        }

        Some(punct)
    });

    parser.visit(&mut hooks, &mut output);

    output
}
