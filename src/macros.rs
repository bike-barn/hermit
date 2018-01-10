macro_rules! subcommand {
    (
        $name:ident, $fn_name:ident {
            $(
                $method:ident($($args:expr),*)
            );*
        }
    ) => {
        fn $fn_name<'a, 'b>(app: App<'a, 'b>)
                                                   -> App<'a, 'b> {
            let subcommand = SubCommand::with_name(stringify!($name))
                $(
                    .$method($($args),*)
                )*;
            app.subcommand(subcommand)
        }
    }
}
