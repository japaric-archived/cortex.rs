extern {
    #![allow(dead_code)]

    #[link_name = "__NVIC__"] static NVIC: ::nvic::Nvic;
}

pub fn nvic() -> &'static ::nvic::Nvic {
    &NVIC
}

