error_chain!{
    foreign_links {
        Io(::std::io::Error);
        LodePNG(lodepng::Error);
        FreeType(freetype::Error);
    }
}