  extern crate regex;
  use regex::Regex;

  pub fn choose_method_delete(http_method: &str, route: &str, body: &str) -> &'static str {
        let re = Regex::new("/deleteVehicle").unwrap();
        let filtered_route: &str = &regex_route(re, route)[..];

        match filtered_route {
                "/deleteVehicle" => return delete_vehicle(),
                &_ => return "route not found",
        };

  }

  pub fn choose_method_post(http_method: &str, route: &str, body: &str) -> &'static str {

          let re = Regex::new("/addVehicle|/updateVehicle").unwrap();
          let filtered_route: &str = &regex_route(re, route)[..];
          match filtered_route {
                         "/addVehicle" => return add_vehicle(),
                         "/updateVehicle" => return update_vehicle(),
                          &_ => return "route not found",
                       };
  }

  pub fn choose_method_get(http_method: &str, route: &str, body: &str) -> &'static str {
          let re = Regex::new(r"/getVehicle\?|/getVehicles").unwrap();
          let filtered_route: &str = &regex_route(re, route)[..];
          match filtered_route {
                         "/getVehicle?" => return get_vehicle(),
                         "/getVehicles" => return get_vehicles(),
                         &_ => return "route not found",
                       };
  }

  pub fn choose_method_put(http_method: &str, route: &str, body: &str) -> &'static str {
          match route {
                           &_ => return "route not found",
                       };
  }

  pub fn choose_method_patch(http_method: &str, route: &str, body: &str) -> &'static str {
          match route {
                         &_ => return "route not found",
                      };
  }


  pub fn regex_route(re: Regex, route: &str) -> String {
        if re.is_match(route) {
            let cap = re.captures(route).unwrap();
            return (&cap[0]).to_string();
        } else {
            return "".to_string();
        }
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




