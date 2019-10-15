// a generic function takes an argument T for any type
fn foo<T>(arg: T) {}

// a concrete type `A`
struct A;
struct S(A);
struct SGen<T>(T);

fn reg_fn(_s: S) {}
fn gen_spec_t(_s: SGen<A>) {}
fn gen_spec_i32(_s: SGen<i32>) {}
fn generic<T>(_s: SGen<T>) {}

// `Single` is a concrete type
struct Single(A);

// `SingleGen` is a generic type
struct SingleGen<T>(T);

struct CS; // concrete type
struct GenericVal<T>(T);

impl GenericVal<f32> {} // Specify f32
impl GenericVal<CS> {} // specify CS

// `<T>` must precede the type to remain generic
impl<T> GenericVal<T> {}

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    // concrete type and explicitly takes `A`
    let _s = Single(A);

    let _char: SingleGen<char> = SingleGen('a');

    let _t = SingleGen(A); // A
    let _i32 = SingleGen(6); // i32
    let _char = SingleGen('c'); // char

    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));

    generic(SGen('c'));
    generic::<char>(SGen('c'));

    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };
    println!("{}, {}", x.value(), y.value());
}

// 具有 Copy trait 的 type，说明其数据存储在栈上，数据赋值给其他变量不会发生所有权的转移（其实是复制给了其他变量）
