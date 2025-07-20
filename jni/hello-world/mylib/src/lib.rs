use jni::{
    objects::{JClass, JString},
    sys::jstring,
    JNIEnv,
};

#[no_mangle]
pub extern "system" fn Java_HelloWorld_hello<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    name: JString<'local>,
) -> jstring {
    let rust_name: String = env.get_string(&name).expect("failed to get name").into();
    println!("[rust] Got name: {rust_name}");

    let res = env
        .new_string(rust_name.to_uppercase())
        .expect("couldn't create a new string");

    res.into_raw()
}
