mod args;
mod printer;

use printer::Printer;
use args::Args;

fn main() {
    let input = Args::get_input();

    // Option::as_deref() to go from Option<String> to Option<&str>
    let value = Args::get_string(input.as_deref());

    Printer::display(value);
}
