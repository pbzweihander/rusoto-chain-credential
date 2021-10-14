# rusoto-chain-credential

Generic rusoto chain credential provider

```rust
let rusoto_provider = rusoto_chain_credential::empty()
    .with(rusoto_sts::WebIdentityProvider::from_k8s_env())
    .with_default_chain_provider();
let rusoto_client = rusoto_core::Client::new(
    rusoto_provider,
    rusoto_core::HttpClient::new().expect("failed to create request dispatcher"),
);
```

## License

_rusoto-chain-credential_ is licensed under the terms of [MIT License](https://spdx.org/licenses/MIT.html).
