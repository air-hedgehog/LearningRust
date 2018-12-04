pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        //the double columns ('::') used here to reference the
        //top 'client' declaration module, asif that was java it
        //would be like 'lib.this.client.connect();'
        ::client::connect();
        //we can use 'super::client::connect();' as well
    }
}
