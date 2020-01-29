mod house
{
    pub mod hosting
    {
        pub fn add() { }
        fn seat() { }
    }

    mod serving
    {
        fn take() {}
        fn order() {}
    }
}

pub fn eat_out()
{
    crate::house::hosting::add();

    house::hosting::add();
}