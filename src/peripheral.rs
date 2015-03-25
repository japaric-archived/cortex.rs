extern {
    #![allow(dead_code)]

    static __NVIC__: ::nvic::Nvic;
}

pub fn nvic() -> &'static ::nvic::Nvic {
    &__NVIC__
}

