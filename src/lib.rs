use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Lit, NestedMeta};

#[proc_macro_attribute]
pub fn main(args: TokenStream, _input: TokenStream) -> TokenStream {
    let input_path = match &parse_macro_input!(args as AttributeArgs)[..] {
        [NestedMeta::Lit(Lit::Int(day))] => format!("../../inputs/{}.in", day.token()),
        _ => panic!("Expected one integer argument"),
    };

    // let mut aoc_solution = parse_macro_input!(input as ItemFn);
    // aoc_solution.sig.ident = Ident::new("aoc_solution", aoc_solution.sig.ident.span());

    let tokens = quote! {
      const INPUT: &str = include_str!(#input_path);
      const INPUT_BYTES: &[u8] = include_bytes!(#input_path);
      // #aoc_solution
      fn main() {
        let now = ::std::time::Instant::now();
        let p1 = part1(INPUT);
        let elapsed_p1 = now.elapsed();


        let now = ::std::time::Instant::now();
        let p2 = part2(INPUT);
        let elapsed_p2 = now.elapsed();
        println!("Part one: {:>9} Time: {:>5}μs", p1, elapsed_p1.as_micros());
        println!("Part two: {:>9} Time: {:>5}μs", p2, elapsed_p2.as_micros());

      }
    };
    TokenStream::from(tokens)
}
