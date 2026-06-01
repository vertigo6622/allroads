pub fn get_template(template_type: &str) -> Vec<&'static str> {
    match template_type {
        "web" => vec![
            "Planning & Design",
            "Backend Setup",
            "Frontend Development",
            "Authentication System",
            "Payment Integration",
            "Testing & QA",
            "Deployment",
        ],
        "mobile" => vec![
            "UI/UX Design",
            "Core Architecture",
            "User Authentication",
            "Main Features",
            "Push Notifications",
            "App Store Submission",
            "Marketing Launch",
        ],
        "api" => vec![
            "API Specification",
            "Database Design",
            "Authentication & Auth",
            "Core Endpoints",
            "Documentation",
            "Testing Suite",
            "Monitoring Setup",
        ],
        _ => vec![],
    }
}
