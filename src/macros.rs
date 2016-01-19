macro_rules! configure_subcommand {
    (
        $name:ident, $fn_name:ident {
            $(
                $method:ident($($args:expr),*)
            );*
        }
    ) => {
        fn $fn_name<'a, 'b, 'c, 'd, 'e, 'f>(app: App<'a, 'b, 'c, 'd, 'e, 'f>)
                                                   -> App<'a, 'b, 'c, 'd, 'e, 'f> {
            let subcommand = SubCommand::with_name(stringify!($name))
                $(
                    .$method($($args),*)
                )*;
            app.subcommand(subcommand)
        }
    }
}
