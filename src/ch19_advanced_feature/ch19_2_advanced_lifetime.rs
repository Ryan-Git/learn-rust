//lifetime_subtyping
struct Context<'s>(&'s str);

struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

//lifetime bounds
struct Ref<'a, T: 'a>(&'a T);

//trait object lifetime
trait Foo {}

struct Bar<'a> {
    x: &'a i32,
}

impl<'a> Foo for Bar<'a> {}

#[test]
fn trait_object_lifetime() {
    //The default lifetime of a trait object is 'static.
    //If we have &'a X or &'a mut X, then the default is 'a.
    //If we have a single T: 'a clause, then the default is 'a.
    //If we have multiple T: 'a-like clauses, then there is no default; we must be explicit.
    let num = 5;
    let obj = Box::new(Bar { x: &num }) as Box<Foo>;
}