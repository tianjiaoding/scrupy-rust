use std::marker::PhantomData;

struct Brace{
    x: i32,
}

impl Brace{
	fn transform(&self, n: i32) -> Devil{
        Devil{
            hp: self.x + n,
            weapon: None,
        }
    }
}

struct Bar<'a>{
    tasty: &'a str,
}

struct Foo<'a, B>
where B: 'a + Into<Bar<'a>>
{
	brace: Brace,
	buz: Option<B>, // buz is of generic type B, and is able to be turned into bar.
	phantom: PhantomData<&'a B>, // A marker that is used to resolve 'unused lifetime parameter a'
}

impl<'a, B: Into<Bar<'a>>> Foo<'a, B> {
	fn transform_and_arm(self){ // line B
		let brace1: Brace = self.brace;
		let mut devil: Devil = brace1.transform(12345); // line A
		let buz = self.buz.unwrap();
		// Before this line, it passes the compiler.
		// Uncommenting the following line causes compiler to argue that the brace1 at line A doesn't live long enough. It says that borrowed value must be valid for the lifetime 'a as defined on the body at line B, but the borrowed value only lives until line C.
		// devil = devil.arm(buz);
		// Although adding the above line fails, making the weapon directly won't cause the compiler to complain.
		// The following line passes compiler.
		// let weapon = buz.into();




		// The compiler stops the devil from arming itself before I even try to write the following line.
		// devil.slay_the_world();
	} // line C
}



struct Devil<'a> {
    hp: i32,
	weapon: Option<Bar<'a>>,
}
impl<'a> Devil<'a>
{
	fn arm<B: Into<Bar<'a>>>(mut self, biu: B) -> Devil<'a>{
		self.weapon = Some(biu.into());
		self
	}

	fn slay_the_world(self){
		unimplemented!()
	}
}

// The fool method is designed to do the following: It consumes an instance of Foo, by taking away brace and buz of it. It calls brace.get(_) to make brace a Devil. It strengthen the devil by feeding the devil with buz.unwrap().
//
//
// It seems that it question is much related with the lifetime. If there were no 'a for Devil, then all these problems would vanish. The fool method would consume the instance of Foo.
