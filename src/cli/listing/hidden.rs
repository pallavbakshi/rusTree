use clap::Args;

#[derive(Args, Debug)]
pub struct AllFilesArgs {
    /// Show hidden files and directories (those starting with a `.`). (Original tree: -a)
    #[arg(short = 'a', long = "include-hidden")]
    pub show_hidden: bool,
}
