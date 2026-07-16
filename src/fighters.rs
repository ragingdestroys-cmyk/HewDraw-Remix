pub fn install() {
    #[cfg(not(feature = "runtime"))]
    {
        common::install();
    }

    // Only Plizardon (Charizard)
    plizardon::install();
}
