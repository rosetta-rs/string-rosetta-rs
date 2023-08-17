fn main() {
    println!("String {} bytes", std::mem::size_of::<String>());
    println!("&'static str {} bytes", std::mem::size_of::<&'static str>());
    println!(
        "Cow<str> {} bytes",
        std::mem::size_of::<std::borrow::Cow<'static, str>>()
    );

    println!("ArcStr {} bytes", std::mem::size_of::<arcstr::ArcStr>());
    println!(
        "CompactString {} bytes",
        std::mem::size_of::<compact_str::CompactString>()
    );
    println!("ecow {} bytes", std::mem::size_of::<ecow::EcoString>());
    println!(
        "FlexStr {} bytes",
        std::mem::size_of::<flexstr::SharedStr>()
    );
    println!("HipStr {} bytes", std::mem::size_of::<hipstr::HipStr>());
    println!("ImString {} bytes", std::mem::size_of::<imstr::ImString>());
    println!("KString {} bytes", std::mem::size_of::<kstring::KString>());
    println!(
        "SmartString {} bytes",
        std::mem::size_of::<smartstring::alias::String>()
    );
}
