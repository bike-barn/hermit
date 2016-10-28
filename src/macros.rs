macro_rules! subcommand {
    {
        $fn_name:ident ( $name:expr ) {
            $(
                $method:ident ( $($args:expr),* )
            )*
        }
    } => {
        fn $fn_name<'a, 'b, 'c, 'd, 'e, 'f>(app: App<'a, 'b, 'c, 'd, 'e, 'f>)
                                                   -> App<'a, 'b, 'c, 'd, 'e, 'f> {
            let subcommand = SubCommand::with_name($name)
                $(
                    .$method($($args),*)
                )*;
            app.subcommand(subcommand)
        }
    }
}
