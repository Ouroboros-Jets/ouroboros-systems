#[macro_export]
macro_rules! if_gui {
    ($if_body:block else $else_body:block) => {
        #[cfg(feature = "gui")]
        {
            $if_body
        }
        #[cfg(not(feature = "gui"))]
        {
            $else_body
        }
    };
}
