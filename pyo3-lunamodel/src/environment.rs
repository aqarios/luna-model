//! C-API-backed PyO3 environment conversion.

capi_wrapper! {
    wrapper: PyEnvironment,
    public: Environment,
    inner: ::rs_lunamodel::Env,
    raw: ::lunamodel_capi::LmEnv,
    attr: "_env",
    from_py: "_from_pyenv",
}
