use clap::Parser as _;
use ls::cli;
use ls::fsops;
use ls::display;

fn main() {
    let args = cli::Args::parse();
    let search_ctx = fsops::SearchContext::new(args.clone());
    let display_ctx = display::DisplayContext::new(args.long, args.classify);

    let entries = search_ctx.search().unwrap();
    if args.long {
        display_ctx.display_entries_long(&entries);
    } else {
        for entry in entries {
            display_ctx.display_entry(&entry);
        }
    }
}
