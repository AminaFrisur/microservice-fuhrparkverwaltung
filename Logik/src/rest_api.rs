  extern crate regex;
  use regex::Regex;

  pub fn choose_method_delete(http_method: &str, route: &str, body: &str) -> &'static str {

        println!("{}", route);
        println!("{}", body);

        match route {
                "/deleteVehicle" => return delete_vehicle(),
                &_ => return "route not found",
        };

  }

  pub fn choose_method_post(http_method: &str, route: &str, body: &str) -> &'static str {
          println!("{}", route);
          println!("{}", body);
          match route {
                         "/addVehicle" => return add_vehicle(),
                         "/updateVehicle" => return update_vehicle(),
                          &_ => return "route not found",
                       };
  }

  pub fn choose_method_get(http_method: &str, route: &str, body: &str) -> &'static str {
          println!("{}", route);
          println!("{}", body);
          match route {
                         "/getVehicle/" => return get_vehicle(),
                         "/getVehicles" => return get_vehicles(),
                         &_ => return "route not found",
                       };
  }

  pub fn choose_method_put(http_method: &str, route: &str, body: &str) -> &'static str {
          println!("{}", route);
          println!("{}", body);
          match route {
                           &_ => return "route not found",
                       };
  }

  pub fn choose_method_patch(http_method: &str, route: &str, body: &str) -> &'static str {
          println!("{}", route);
          println!("{}", body);
          match route {
                         &_ => return "route not found",
                      };
  }

  pub fn add_vehicle() -> &'static str {
      println!("addVehicle");
      return "ERFOLG mit add_vehicle";
  }

  pub fn get_vehicle() -> &'static str {
      println!("getVehicle");
      return "ERFOLG mit get_vehicle";
  }

  pub fn get_vehicles() -> &'static str {
        println!("getVehicles");
        return "ERFOLG mit get_vehicles";
    }

  pub fn update_vehicle() -> &'static str {
      println!("updateVehicle");
      return "ERFOLG mit update_vehicle";
  }

  pub fn delete_vehicle() -> &'static str {
      println!("deleteVehicle");
      return "ERFOLG mit delete_vehicle";
  }




