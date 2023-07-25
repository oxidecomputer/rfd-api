// Based on the work of https://github.com/ecliptical/tracing-slog and https://github.com/tokio-rs/tracing/tree/master/tracing-log

use slog::{Drain, Key, Serializer, KV};
use std::collections::HashMap;
use tracing_core::{
    callsite, dispatcher, field, identify_callsite, subscriber, Callsite, Event, Kind, Level,
    Metadata,
};
use valuable::Valuable;

pub struct Bridge {
    values: HashMap<String, String>,
}

impl Serializer for Bridge {
    fn emit_arguments(&mut self, key: Key, val: &std::fmt::Arguments) -> slog::Result {
        self.values.insert(format!("{}", key), format!("{}", val));
        Ok(())
    }
}

#[derive(Debug)]
pub struct BridgeDrain;

impl Drain for BridgeDrain {
    type Ok = ();
    type Err = slog::Never;

    fn log(
        &self,
        record: &slog::Record<'_>,
        values: &slog::OwnedKVList,
    ) -> Result<Self::Ok, Self::Err> {
        dispatcher::get_default(|dispatch| {
            let bridge = get_callsite(record.level());
            let cs = bridge.callsite();

            if dispatch.enabled(cs.metadata()) {
                let mut serializer = Bridge {
                    values: HashMap::new(),
                };
                values.serialize(record, &mut serializer).unwrap();

                dispatch.event(&Event::new(
                    cs.metadata(),
                    &cs.metadata().fields().value_set(&[
                        (
                            &bridge.field("message"),
                            Some(record.msg() as &dyn field::Value),
                        ),
                        (&bridge.field("slog.module"), Some(&record.module())),
                        (&bridge.field("slog.file"), Some(&record.file())),
                        (&bridge.field("slog.line"), Some(&record.line())),
                        (&bridge.field("slog.column"), Some(&record.column())),
                        (
                            &bridge.field("slog.kv"),
                            Some(&serializer.values.as_value()),
                        ),
                    ]),
                ));
            }
        });

        Ok(())
    }
}

static FIELD_NAMES: &[&str] = &[
    "message",
    "slog.module",
    "slog.file",
    "slog.line",
    "slog.column",
    "slog.kv",
];

fn get_callsite(level: slog::Level) -> &'static dyn BridgeCallsite {
    match level {
        slog::Level::Trace => &TraceCallsite,
        slog::Level::Debug => &DebugCallsite,
        slog::Level::Info => &InfoCallsite,
        slog::Level::Warning => &WarnCallsite,
        slog::Level::Error => &ErrorCallsite,
        slog::Level::Critical => &ErrorCallsite,
    }
}

trait BridgeCallsite {
    fn field(&self, field_name: &str) -> field::Field;
    fn callsite(&'static self) -> &'static dyn callsite::Callsite;
}

macro_rules! level_callsite {
    ($callsite:ident, $level:expr, $meta:ident) => {
        struct $callsite;
        static $meta: Metadata<'static> = Metadata::new(
            "slog event",
            "slog",
            $level,
            // File, line, and module all need to be statically defined. To provide these, they need to
            // be exposed separately and subscribers need to be aware of where to get them
            None,
            None,
            None,
            field::FieldSet::new(FIELD_NAMES, identify_callsite!(&$callsite)),
            Kind::EVENT,
        );

        impl BridgeCallsite for $callsite {
            fn field(&self, field_name: &str) -> field::Field {
                self.metadata().fields().field(field_name).unwrap()
            }

            fn callsite(&'static self) -> &'static dyn callsite::Callsite {
                self
            }
        }

        impl callsite::Callsite for $callsite {
            fn set_interest(&self, _: subscriber::Interest) {}
            fn metadata(&self) -> &'static Metadata<'static> {
                &$meta
            }
        }
    };
}

level_callsite!(TraceCallsite, Level::TRACE, TRACE_META);
level_callsite!(DebugCallsite, Level::DEBUG, DEBUG_META);
level_callsite!(InfoCallsite, Level::INFO, INFO_META);
level_callsite!(WarnCallsite, Level::WARN, WARN_META);
level_callsite!(ErrorCallsite, Level::ERROR, ERROR_META);
