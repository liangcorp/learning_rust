pub fn enter_login_detail(name: &String, password: &String) -> bool {
    *name == String::from("bob") && *password == String::from("123")
}