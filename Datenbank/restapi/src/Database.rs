
struct fahrzeug {
    model: String,
    marke: String,
    id: i32,
    email: String,
    leistung: i32
}

pub fn add_vehicle() -> &'static str {
    return "ERFOLG mit add_vehicle";
}

pub fn get_vehicle() -> &'static str {
    return "ERFOLG mit get_vehicle";
}

pub fn get_vehicles() -> &'static str {
    return "ERFOLG mit get_vehicles";
}

pub fn update_vehicle() -> &'static str {
    return "ERFOLG mit update_vehicle";
}

pub fn delete_vehicle() -> &'static str {
    return "ERFOLG mit delete_vehicle";
}




