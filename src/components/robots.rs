/// Creates the robots.txt file
pub fn robots(hidden_services: &[&str]) -> String {
    let mut robots = String::new();

    robots.push_str("User-agent: *\n");

    for service in hidden_services {
        robots.push_str(&format!("Disallow: /{}\n", service));
    }

    robots
}
