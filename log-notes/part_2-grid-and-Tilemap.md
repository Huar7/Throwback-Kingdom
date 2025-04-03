pada dasarnya tilemap itu hanya seperti grid system dengan fitur tambahan untuk
menggambar dan hal lainnya


// map generasi:::
struct MapGenerator{operasi generasi dunia dilakukann disini}

map generator akan mencipatakan satu tilemap


// Tilemap itu sendiri:::
struct Tilemap(){// operasi tilemap dilakukan disini}
impl tilemap{contoh: drop_loot(pos: Vec2, item: Vec<Item>), drop_wood(pos), drop_stone(pos), drop...}

-> notice jika sistem yang digunakan membuat kita menggunakan satu object saja sebagai tilemap manager

