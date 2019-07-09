#[cfg(test)]
mod tests {
    use amethyst::{
        core::TransformBundle,
        input::StringBindings,
        renderer::{types::DefaultBackend, RenderEmptyBundle},
        ui::UiBundle,
        Error, LogLevelFilter, LoggerConfig,
    };
    use amethyst_test::AmethystApplication;

    macro_rules! test_n {
        ($test_name:ident) => {
            #[test]
            fn $test_name() -> Result<(), Error> {
                amethyst::start_logger(LoggerConfig {
                    level_filter: LogLevelFilter::Debug,
                    ..Default::default()
                });

                AmethystApplication::blank()
                    .with_bundle(TransformBundle::new())
                    .with_bundle(UiBundle::<DefaultBackend, StringBindings>::new())
                    .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
                    // .run() // segfaults
                    .run_isolated() // doesn't segfault
            }
        };
    }

    test_n!(test_0);
    test_n!(test_1);
    test_n!(test_2);
    test_n!(test_3);
    test_n!(test_4);
    test_n!(test_5);
    test_n!(test_6);
    test_n!(test_7);
    test_n!(test_8);
    test_n!(test_9);

    test_n!(test_10);
    test_n!(test_11);
    test_n!(test_12);
    test_n!(test_13);
    test_n!(test_14);
    test_n!(test_15);
    test_n!(test_16);
    test_n!(test_17);
    test_n!(test_18);
    test_n!(test_19);

}
