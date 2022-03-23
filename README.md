# Mockmail

Implementation of mock service for testing emails in Rust using

- enums,
- generics, and
- trait objects.

You can find the relevant code in the identically named workspaces above.

## Using enums

Our email service should take either a `RealClient` which sends a *real* email in production or a `MockClient`  which sends a *mock* email while testing. An enum with a `Real` and a `Mock` variant is a natural way to achieve this:

``` rust
pub enum EmailClient {
    Real(RealClient),
    Mock(MockClient),
}
```

We can improve on this by using conditional compilation on the `Mock` variant:

``` rust
pub enum EmailClient {
    Real(RealClient),
    #[cfg(test)]
    Mock(MockClient),
}
```

This way the additional `Mock` variant does compile for unit testing. However, the `Real` variant is the only variant for a release build, leading to (slightly) better runtime performance.

Finally, our service takes an `EmailClient` as follows:

``` rust
pub async fn email_service(client: &EmailClient) -> Result<(), anyhow::Error> {
    let email = Email::default();
    client.send(email).await
}
```

where `EmailClient` is either a `RealClient` or a `MockClient`.

## Using traits

Instead of enums you can use traits as well:

``` rust
#[async_trait]
pub trait SendEmail {
    async fn send(&self, email: Email) -> Result<(), anyhow::Error>;
}

```

The trait has to be implemented for the `RealClient` as well as the `MockClient` and can then be used via

1. generics (static dispatch), or
1. trait objects (dynamic dispatch).

On the one hand, static dispatch has better runtime performance through monomorphization while dynamic dispatch has a slight performance overhead through a vtable lookup. On the other hand, static dispatch is leading to a sligthly longer compilation time and larger binary size compared to dynamic dispatch.

### Using generics

In this case, the service is generic over parameter `T` with trait bound `SendEmail`:

``` rust
pub async fn email_service<T: SendEmail>(client: &T) -> Result<(), anyhow::Error> {
    let email = Email::default();
    client.send(email).await
}
```

A more tense and in this case equivalent notation uses the `impl` keyword:

``` rust
pub async fn email_service(client: &impl SendEmail) -> Result<(), anyhow::Error> {
    let email = Email::default();
    client.send(email).await
}
```

### Using trait objects

Using dynamic dispatch, the trait object is denoted by the `dyn` keyword:

``` rust
pub async fn email_service(client: &dyn SendEmail) -> Result<(), anyhow::Error> {
    let email = Email::default();
    client.send(email).await
}
```

## Testing

Run tests in all workspaces:

``` bash
cargo test --workspace
```

Run tests in all workspaces and show output:

``` bash
cargo test --workspace -- --show-output
```
