use progenitor::generate_api;

generate_api!(
    spec = "../rfd-api-spec.json",
    interface = Builder,
    inner_type = (),
);
