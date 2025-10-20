use proc_macro::*;

fn parse_expr(tokens: &[TokenTree]) -> [usize; 4] {
	let start = tokens.as_ptr().addr();

	let mut tokens = tokens;
	let mut give_up = false;

	macro_rules! try_give_up {
		($tt:tt) => {
			if let [TokenTree::Punct(p0), TokenTree::Punct(p1), rest @ ..] = tokens {
				if p0.as_char() == '&' && p0.spacing() == Spacing::Joint && p1.as_char() == '&' {
					give_up = true;
					tokens = rest;
					$tt;
				}
			}

			if let [TokenTree::Punct(p0), rest @ ..] = tokens {
				if p0.as_char() == '|' {
					give_up = true;
					tokens = rest;
					$tt;
				}
			}
		};
	}

	macro_rules! skip_generics {
		() => {
			if let [TokenTree::Punct(p0), TokenTree::Punct(p1), TokenTree::Punct(p2), rest @ ..] = tokens {
				if p0.as_char() == ':'
					&& p0.spacing() == Spacing::Joint
					&& p1.as_char() == ':'
					&& p1.spacing() == Spacing::Joint
					&& p2.as_char() == '<'
				{
					tokens = rest;

					while let [tt, rest @ ..] = tokens {
						tokens = rest;
						if let TokenTree::Punct(p0) = tt {
							if p0.as_char() == '>' {
								break;
							}
						}
					}

					continue;
				}
			}
			if let [TokenTree::Punct(p0), TokenTree::Punct(p1), rest @ ..] = tokens {
				if p0.as_char() == ':' && p0.spacing() == Spacing::Joint && p1.as_char() == ':' {
					tokens = rest;
					continue;
				}
			}
		};
	}

	macro_rules! offset {
		() => {
			(tokens.as_ptr().addr() - start) / size_of::<TokenTree>()
		};
	}

	let lhs;
	let op;
	let rhs;

	'main: loop {
		skip_generics!();
		try_give_up!({
			op = 0;
			lhs = 0;
			break;
		});

		// shift
		for c in ['<', '>'] {
			if let [TokenTree::Punct(p0), TokenTree::Punct(p1), rest @ ..] = tokens {
				if p0.as_char() == c && p0.spacing() == Spacing::Joint && p1.as_char() == c {
					tokens = rest;
					continue 'main;
				}
			}
		}

		// arrow
		if let [TokenTree::Punct(p0), TokenTree::Punct(p1), rest @ ..] = tokens {
			if p0.as_char() == '-' && p0.spacing() == Spacing::Joint && p1.as_char() == '>' {
				tokens = rest;
				continue 'main;
			}
		}

		for c in ['<', '>', '=', '!'] {
			if let [TokenTree::Punct(p0), TokenTree::Punct(p1), rest @ ..] = tokens {
				if p0.as_char() == c && p0.spacing() == Spacing::Joint && p1.as_char() == '=' {
					lhs = offset!();
					op = lhs + 2;

					tokens = rest;
					break 'main;
				}
			}
		}
		for c in ['<', '>', '~'] {
			if let [TokenTree::Punct(p0), rest @ ..] = tokens {
				if p0.as_char() == c {
					lhs = offset!();
					op = lhs + 1;

					tokens = rest;
					break 'main;
				}
			}
		}

		if let [TokenTree::Punct(p0), rest @ ..] = tokens {
			if p0.as_char() == ':' {
				lhs = offset!();
				tokens = rest;
				'op: loop {
					skip_generics!();
					if let [tt, rest @ ..] = tokens {
						tokens = rest;
						if let TokenTree::Punct(p0) = tt {
							if p0.as_char() == ':' {
								op = offset!();
								break 'op;
							}
						}
					}
				}
				break 'main;
			}
		}

		// comma
		if let [TokenTree::Punct(p0), ..] = tokens {
			if p0.as_char() == ',' {
				lhs = offset!();
				return [lhs, lhs, lhs, lhs + 1];
			}
		}
		if let [_, rest @ ..] = tokens {
			tokens = rest;
			continue 'main;
		}
		lhs = offset!();
		return [lhs, lhs, lhs, lhs];
	}

	'main: loop {
		skip_generics!();
		try_give_up!(continue);

		if let [TokenTree::Punct(p0), ..] = tokens {
			if p0.as_char() == ',' {
				rhs = offset!();
				if give_up {
					return [rhs, rhs, rhs, rhs + 1];
				}
				return [lhs, op, rhs, rhs + 1];
			}
		}
		if let [_, rest @ ..] = tokens {
			tokens = rest;
			continue 'main;
		}
		rhs = offset!();
		if give_up {
			return [rhs, rhs, rhs, rhs];
		}
		return [lhs, op, rhs, rhs];
	}
}

fn parse(tokens: &[TokenTree]) -> (TokenStream, usize) {
	let [lhs, op, rhs, next] = parse_expr(tokens);
	if lhs < op {
		let args = [TokenTree::Group(Group::new(
			Delimiter::Parenthesis,
			TokenStream::from_iter(
				tokens[..lhs]
					.iter()
					.cloned()
					.chain([TokenTree::Punct(Punct::new(',', Spacing::Alone))])
					.chain(tokens[op..rhs].iter().cloned()),
			),
		))];

		if let TokenTree::Punct(p) = &tokens[lhs] {
			if p.as_char() == ':' {
				return (
					TokenStream::from_iter(
						tokens[lhs + 1..op - 1]
							.iter()
							.cloned()
							.chain([TokenTree::Punct(Punct::new(',', Spacing::Alone))])
							.chain(args),
					),
					next,
				);
			}
		}

		if let TokenTree::Punct(p) = &tokens[lhs] {
			if p.as_char() == '~' {
				return (
					TokenStream::from_iter(
						[
							TokenTree::Ident(Ident::new("approx_eq", p.span())),
							TokenTree::Punct(Punct::new(',', Spacing::Alone)),
							TokenTree::Punct(Punct::new('~', Spacing::Alone)),
						]
						.into_iter()
						.chain(args),
					),
					next,
				);
			}
		}

		return (TokenStream::from_iter(tokens[lhs..op].iter().cloned().chain(args)), next);
	}
	assert_eq!(lhs, op);
	assert_eq!(op, rhs);

	let tokens = &tokens[..lhs];

	if let [TokenTree::Ident(f), TokenTree::Group(g)] = tokens {
		if matches!(&*f.to_string(), "any" | "all") {
			let mut start = 0;
			let mut cond = vec![];
			let g = &*Vec::from_iter(g.stream());
			while start < g.len() {
				let (c, next) = parse(&g[start..]);
				cond.push(TokenTree::Group(Group::new(Delimiter::Parenthesis, c)));
				start += next;
			}
			return (
				TokenStream::from_iter([
					TokenTree::Ident(f.clone()),
					TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::from_iter(cond))),
				]),
				next,
			);
		}
	}

	(TokenStream::from_iter(tokens.iter().cloned()), next)
}

#[proc_macro]
pub fn assert(item: TokenStream) -> TokenStream {
	let mut item = item.into_iter();
	let Some(TokenTree::Group(krate)) = item.next() else { panic!() };
	let item = &*Vec::from_iter(item);
	let (cond, next) = parse(item);

	let stream = TokenStream::from_iter(
		krate.stream().into_iter().chain([
			TokenTree::Punct(Punct::new(':', Spacing::Joint)),
			TokenTree::Punct(Punct::new(':', Spacing::Alone)),
			TokenTree::Ident(Ident::new("do_panic", Span::call_site())),
			TokenTree::Group(Group::new(
				Delimiter::Parenthesis,
				TokenStream::from_iter(
					TokenStream::new()
						.into_iter()
						.chain([
							TokenTree::Ident(Ident::new("const", Span::call_site())),
							TokenTree::Group(Group::new(
								Delimiter::Brace,
								TokenStream::from_iter(
									TokenStream::new()
										.into_iter()
										.chain([TokenTree::Punct(Punct::new('&', Spacing::Alone))])
										.chain(krate.stream())
										.chain([
											TokenTree::Punct(Punct::new(':', Spacing::Joint)),
											TokenTree::Punct(Punct::new(':', Spacing::Alone)),
											TokenTree::Ident(Ident::new("WithSource", Span::call_site())),
											TokenTree::Group(Group::new(
												Delimiter::Brace,
												TokenStream::new()
													.into_iter()
													.chain([
														TokenTree::Ident(Ident::new("file", Span::call_site())),
														TokenTree::Punct(Punct::new(':', Spacing::Alone)),
														TokenTree::Punct(Punct::new(':', Spacing::Joint)),
														TokenTree::Punct(Punct::new(':', Spacing::Alone)),
														TokenTree::Ident(Ident::new("core", Span::call_site())),
														TokenTree::Punct(Punct::new(':', Spacing::Joint)),
														TokenTree::Punct(Punct::new(':', Spacing::Alone)),
														TokenTree::Ident(Ident::new("file", Span::call_site())),
														TokenTree::Punct(Punct::new('!', Spacing::Alone)),
														TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
														TokenTree::Punct(Punct::new(',', Spacing::Alone)),
													])
													.chain([
														TokenTree::Ident(Ident::new("line", Span::call_site())),
														TokenTree::Punct(Punct::new(':', Spacing::Alone)),
														TokenTree::Punct(Punct::new(':', Spacing::Joint)),
														TokenTree::Punct(Punct::new(':', Spacing::Alone)),
														TokenTree::Ident(Ident::new("core", Span::call_site())),
														TokenTree::Punct(Punct::new(':', Spacing::Joint)),
														TokenTree::Punct(Punct::new(':', Spacing::Alone)),
														TokenTree::Ident(Ident::new("line", Span::call_site())),
														TokenTree::Punct(Punct::new('!', Spacing::Alone)),
														TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
														TokenTree::Punct(Punct::new(',', Spacing::Alone)),
													])
													.chain([
														TokenTree::Ident(Ident::new("col", Span::call_site())),
														TokenTree::Punct(Punct::new(':', Spacing::Alone)),
														TokenTree::Punct(Punct::new(':', Spacing::Joint)),
														TokenTree::Punct(Punct::new(':', Spacing::Alone)),
														TokenTree::Ident(Ident::new("core", Span::call_site())),
														TokenTree::Punct(Punct::new(':', Spacing::Joint)),
														TokenTree::Punct(Punct::new(':', Spacing::Alone)),
														TokenTree::Ident(Ident::new("column", Span::call_site())),
														TokenTree::Punct(Punct::new('!', Spacing::Alone)),
														TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
														TokenTree::Punct(Punct::new(',', Spacing::Alone)),
													])
													.chain([
														TokenTree::Ident(Ident::new("source", Span::call_site())),
														TokenTree::Punct(Punct::new(':', Spacing::Alone)),
													])
													.chain(krate.stream())
													.chain([
														TokenTree::Punct(Punct::new(':', Spacing::Joint)),
														TokenTree::Punct(Punct::new(':', Spacing::Alone)),
														TokenTree::Ident(Ident::new("source_imp", Span::call_site())),
														TokenTree::Punct(Punct::new('!', Spacing::Alone)),
														TokenTree::Group(Group::new(Delimiter::Parenthesis, cond.clone())),
														TokenTree::Punct(Punct::new(',', Spacing::Alone)),
													])
													.collect(),
											)),
										]),
								),
							)),
							TokenTree::Punct(Punct::new(',', Spacing::Alone)),
						])
						.chain(krate.stream())
						.chain([
							TokenTree::Punct(Punct::new(':', Spacing::Joint)),
							TokenTree::Punct(Punct::new(':', Spacing::Alone)),
							TokenTree::Ident(Ident::new("assert_imp", Span::call_site())),
							TokenTree::Punct(Punct::new('!', Spacing::Alone)),
							TokenTree::Group(Group::new(Delimiter::Parenthesis, cond.clone())),
							TokenTree::Punct(Punct::new(',', Spacing::Alone)),
						])
						.chain(krate.stream())
						.chain([
							TokenTree::Punct(Punct::new(':', Spacing::Joint)),
							TokenTree::Punct(Punct::new(':', Spacing::Alone)),
							TokenTree::Ident(Ident::new("fmt_imp", Span::call_site())),
							TokenTree::Punct(Punct::new('!', Spacing::Alone)),
							TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::from_iter(item[next..].iter().cloned()))),
							TokenTree::Punct(Punct::new(',', Spacing::Alone)),
						]),
				),
			)),
		]),
	);

	stream
}
