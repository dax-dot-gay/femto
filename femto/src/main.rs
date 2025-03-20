use femto_common::{femto_cli, ProjectSource};

fn main() {
    println!("{:?}", femto_cli().get_matches().subcommand_matches("new").unwrap().get_one::<ProjectSource>("TEMPLATE"));
}
