struct Component;
impl bindings::foo::Foo for Component {
    fn hello_world(input: bindings::foo::Input){
      println!("FIELD A {:?}", input.a);
      println!("FIELD B {:?}", input.b);
    }
}

bindings::export!(Component);
