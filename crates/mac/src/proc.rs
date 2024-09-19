use proc_macro::TokenStream;

#[proc_macro]
pub fn cmd_execute(input: TokenStream) -> TokenStream {
    // 只接受一个字符串参数
    let input: syn::LitStr = syn::parse(input).unwrap();

    #[cfg(target_os="windows")]
    let sh = "cmd";
    #[cfg(not(target_os="windows"))]
    let sh = "bash";

    let mut cmd = std::process::Command::new(sh);

    #[cfg(target_os="windows")]
    cmd.arg("/c");
    #[cfg(not(target_os="windows"))]
    cmd.arg("-c");

    cmd.arg(input.value());
    let output = match cmd.output() {
        Ok(out) => out,
        Err(e) => panic!("{}", e),
    };
    // println!("output: {:?}", output);
    if !output.status.success() {
        panic!("The command's output is: {:?}", output);
    }

    let stdout = output.stdout;

    quote::quote! {
        &[
            #(#stdout,)*
        ]
    }.into()
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn short_commit() {
//         let commit_hash = cmd_execute!("git rev-parse --short HEAD");
//     }
// }
