macro_rules! subcommand {
    {
        $fn_name:ident ( $name:expr ) {
            $(
                $method:ident ( $($args:expr),* )
            )*
        }
    } => {
        fn $fn_name<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
            let subcommand = SubCommand::with_name($name)
                $(
                    .$method($($args),*)
                )*;
            app.subcommand(subcommand)
        }
    }
}
