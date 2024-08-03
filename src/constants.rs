pub const SERVER_STARTED: &str = "✅ Server started successfully";
pub const DATABASE_STARTED: &str = "✅ Connected to database and table created !";
pub const PATH_UPLOAD_CV: &str = "uploads/cv";
pub const MESSAGE_SIGNUP_SUCCESS: &str = "Signup successfully";
//pub const MESSAGE_LOGIN_SUCCESS: &str = "Login successfully";
pub const MESSAGE_LOGIN_FAILED: &str = "Wrong username or password, please try again";
pub const MESSAGE_USER_NOT_FOUND: &str = "User not found, please signup";
//pub const MESSAGE_LOGOUT_SUCCESS: &str = "Logout successfully";
pub const MESSAGE_PROCESS_TOKEN_ERROR: &str = "Error while processing token";
pub const MESSAGE_INVALID_TOKEN: &str = "Invalid token, please login again";
//pub const MESSAGE_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";
pub const MESSAGE_SUPERADMIN_NOT_FOUND: &str =
    "You are not a super admin, please contact the super admin";

// Bad request messages
pub const MESSAGE_TOKEN_MISSING: &str = "Token is missing";
//pub const MESSAGE_BAD_REQUEST: &str = "Bad Request";

// Headers
//pub const AUTHORIZATION: &str = "Authorization";

// Misc
pub const EMPTY: &str = "";

// ignore routes
pub const IGNORE_ROUTES: [&str; 4] = ["/health-check", "/", "/jobs", "/admin/login"];
