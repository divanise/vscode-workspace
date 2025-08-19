use std::marker::PhantomData;

pub struct Services {
    phantom: PhantomData<()>,
}

impl Services {
    pub fn new() -> Self {
        Services {
            phantom: PhantomData,
        }
    }
}

impl crate::routes::Services for Services {
    async fn greet(&self, name: Option<String>) -> String {
        let mut greeting = String::with_capacity(32);

        greeting.push_str("Hello, ");
        greeting.push_str(match &name {
            Some(name) => name,
            None => "world",
        });
        greeting.push('!');

        greeting
    }
}
